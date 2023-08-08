use super::fen::Fen;
use super::fen::{FenParse, FenParseError};
use super::moves::{Castle, CastlingRights};
use super::piece::{
    BoardPiece::{self, *},
    Color,
};
use crate::kelp::Squares;
use crate::kelp::{kelp_core::bitboard::BitBoard, BitBoardArray, BoardInfo};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Board {
    pub bitboards: BitBoardArray,
    pub info: BoardInfo,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboards: [BitBoard::empty(); 12],
            info: BoardInfo {
                turn: Color::White,
                castling_rights: Castle::new(),
                en_passant: None,
                halfmove_clock: 0,
                fullmove_clock: 1,
            },
        }
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

        for rank in (0..8).rev() {
            let mut file = 0;
            for c in board.next().unwrap().chars() {
                if c.is_alphabetic() {
                    let piece = BoardPiece::from_str(c.to_string().as_str());
                    if piece.is_err() {
                        return Err(FenParseError::InvalidPiece(format!(
                            "Invalid piece: {}",
                            c
                        )));
                    }
                    let piece = piece.unwrap();
                    bitboards[piece as usize].set_bit(rank * 8 + file);
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

        Ok(Board {
            bitboards,
            info: BoardInfo {
                turn,
                castling_rights,
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
