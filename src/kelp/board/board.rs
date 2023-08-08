use super::fen::Fen;
use super::fen::{FenParse, FenParseError};
use super::moves::{Castle, CastlingRights};
use super::piece::{
    BoardPiece::{self, *},
    Color,
};
use crate::kelp::mov_gen::generator::MovGen;
use crate::kelp::Squares;
use crate::kelp::{kelp_core::bitboard::BitBoard, BitBoardArray, BoardInfo, GamePhase, GameState};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Board {
    pub bitboards: BitBoardArray,
    pub hash: u64,
    pub state: GameState,
    pub phase: GamePhase,
    pub info: BoardInfo,
}

impl Board {
    pub fn add_to_bb(&mut self, piece: BoardPiece, square: Squares) {
        self.bitboards[piece as usize].set_bit(square as u8);
    }

    pub fn remove_from_bb(&mut self, piece: BoardPiece, square: Squares) {
        self.bitboards[piece as usize].clear_bit(square as u8);
    }

    pub fn add_piece(&mut self, piece: BoardPiece, square: Squares) {
        self.add_to_bb(piece, square);
        // self.hash ^= crate::kelp::zobrist::get_piece_hash(piece, square); TODO
    }

    pub fn remove_piece(&mut self, piece: BoardPiece, square: Squares) {
        self.remove_from_bb(piece, square);
        // self.hash ^= crate::kelp::zobrist::get_piece_hash(piece, square); TODO
    }

    pub fn move_piece(&mut self, piece: BoardPiece, from: Squares, to: Squares) {
        self.remove_piece(piece, from);
        self.add_piece(piece, to);
    }

    pub fn get_piece_at(&self, square: Squares) -> Option<BoardPiece> {
        BoardPiece::iter().find(|&piece| self.bitboards[piece as usize].get_bit(square as u8))
    }

    pub fn get_king_square(&self, color: Color) -> Squares {
        let king = match color {
            Color::White => WhiteKing,
            Color::Black => BlackKing,
        };

        let sq = Squares::from_repr(self.bitboards[king as usize].get_lsb());
        if sq.is_none() {
            panic!("No king found for color {:?}", color);
        }
        sq.unwrap()
    }

    pub fn is_king_checked(&self, color: Color, gen: &MovGen) -> bool {
        let king = self.get_king_square(color);
        gen.is_attacked(king, !color, self)
    }

    pub fn get_piece_occ(&self, piece: BoardPiece) -> BitBoard {
        self.bitboards[piece as usize]
    }

    pub fn get_white_occ(&self) -> BitBoard {
        let mut out = BitBoard::empty();
        out = self.bitboards[WhitePawn as usize]
            | self.bitboards[WhiteKnight as usize]
            | self.bitboards[WhiteBishop as usize]
            | self.bitboards[WhiteRook as usize]
            | self.bitboards[WhiteQueen as usize]
            | self.bitboards[WhiteKing as usize];
        out
    }

    pub fn get_black_occ(&self) -> BitBoard {
        let mut out = BitBoard::empty();
        out = self.bitboards[BlackPawn as usize]
            | self.bitboards[BlackKnight as usize]
            | self.bitboards[BlackBishop as usize]
            | self.bitboards[BlackRook as usize]
            | self.bitboards[BlackQueen as usize]
            | self.bitboards[BlackKing as usize];
        out
    }

    pub fn get_occ(&self) -> BitBoard {
        self.get_white_occ() | self.get_black_occ()
    }
}

// Trait implementations

impl FenParse<Fen, Board, FenParseError> for Board {
    fn parse(fen: Fen) -> Result<Board, FenParseError> {
        let mut bitboards: BitBoardArray = [BitBoard::empty(); 12];

        let is_valid = fen.is_valid();
        match is_valid {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let parts: Vec<&str> = fen.0.split_whitespace().collect::<Vec<&str>>();
        let mut board = parts[0].split("/");
        let mut number_of_pieces = 0;

        for rank in (0..8).rev() {
            let mut file = 0;
            for c in board.next().unwrap().chars() {
                if c.is_alphabetic() {
                    let piece = BoardPiece::from_str(c.to_string().as_str());
                    if piece.is_err() {
                        return Err(FenParseError::InvalidPiece(format!("Invalid piece: {}", c)));
                    }
                    let piece = piece.unwrap();
                    bitboards[piece as usize].set_bit(rank * 8 + file);
                    number_of_pieces += 1;
                    file += 1;
                } else if c.is_numeric() {
                    file += (c.to_digit(10).unwrap()) as u8;
                }
            }
        }

        let turn = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => {
                return Err(FenParseError::InvalidTurn(format!(
                    "Invalid turn: {}, must be 'w' or 'b'",
                    parts[1]
                )))
            }
        };

        let mut castling_rights: Castle = Castle(0);
        for c in parts[2].chars() {
            match c {
                'K' => castling_rights.add(CastlingRights::WhiteKingSide),
                'Q' => castling_rights.add(CastlingRights::WhiteQueenSide),
                'k' => castling_rights.add(CastlingRights::BlackKingSide),
                'q' => castling_rights.add(CastlingRights::BlackQueenSide),
                '-' => {}
                _ => {
                    return Err(FenParseError::InvalidCastlingRights(format!(
                        "Invalid castling rights: {}, must be 'K', 'Q', 'k', 'q', or '-'",
                        parts[2]
                    )))
                }
            }
        }

        let en_passant = match parts[3] {
            "-" => None,
            _ => Some({
                let enp = Squares::from_str(parts[3]);
                if enp.is_err() {
                    return Err(FenParseError::InvalidEnPassant(format!(
                        "Invalid en passant square: {}",
                        parts[3]
                    )));
                }
                enp.unwrap()
            }),
        };

        let halfmove_clock = parts[4].parse::<u8>().unwrap();
        let fullmove_clock = parts[5].parse::<u8>().unwrap();

        let game_phase = {
            if number_of_pieces <= 12 {
                GamePhase::EndGame
            } else if number_of_pieces <= 24 {
                GamePhase::MiddleGame
            } else {
                GamePhase::Opening
            }
        };

        let game_state = {
            if game_phase == GamePhase::EndGame {
                if halfmove_clock >= 100 {
                    GameState::Draw
                } else {
                    GameState::Playing
                }
            } else {
                GameState::Playing
            }
        };

        Ok(Board {
            bitboards,
            hash: 0, // TODO
            state: game_state,
            phase: game_phase,
            info: BoardInfo {
                turn,
                castle: castling_rights,
                en_passant,
                halfmove_clock,
                fullmove_clock,
            },
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mut piece: Option<BoardPiece> = None;
                for i in 0..12 {
                    if self.bitboards[i].get_bit(rank * 8 + file) {
                        piece = Some(BoardPiece::from(i as u8));
                        break;
                    }
                }
                board.push_str(&format!(
                    "{} ",
                    match piece {
                        Some(p) => format!(" {}", p.unicode()),
                        None => " .".to_string(),
                    }
                ));
                if file == 7 {
                    board.push_str(&format!("\t{}", rank + 1));
                }
            }
            board.push('\n');
        }
        board.push_str("\n a  b  c  d  e  f  g  h\n");
        write!(f, "{}", board)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mut piece: Option<BoardPiece> = None;
                for i in 0..12 {
                    if self.bitboards[i].get_bit(rank * 8 + file) {
                        piece = Some(BoardPiece::from(i as u8));
                        break;
                    }
                }
                board.push_str(&format!(
                    "{} ",
                    match piece {
                        Some(p) => format!(" {}", p.to_string()),
                        None => " .".to_string(),
                    }
                ));

                if file == 7 {
                    board.push_str(&format!("\t{}", rank + 1));
                }
            }
            board.push_str("\n");
        }
        board.push_str("\n a  b  c  d  e  f  g  h\n");
        let board_info = format!("{:?}", self.info);
        board.push_str(&board_info);
        write!(f, "{}", board)
    }
}
