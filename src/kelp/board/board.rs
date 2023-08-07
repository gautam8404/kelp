use super::super::{
    board::bitboard::BitBoard,
    board::piece::{
        BoardPiece::{self, *},
        Color,
    },
    BitBoardArray, BoardInfo, Castle, CastlingRights, Move, MoveType,
};
use super::fen::Fen;
use crate::kelp::board::fen::{FenParse, FenParseError};
use std::fmt::{Debug, Display};
use crate::kelp::Squares;
use crate::str_to_enum;

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

        let parts: Vec<&str> = fen.fen.split_whitespace().collect::<Vec<&str>>();
        let mut board = parts[0].split("/");

        for rank in (0..8).rev() {
            let mut file = 0;
            for c in board.next().unwrap().chars() {
                if c.is_alphabetic() {
                    let piece = BoardPiece::from(c);
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
            _ => Some(str_to_enum!(parts[3], Squares).unwrap()),
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
                        Some(p) => p.unicode(),
                        None => ".",
                    }
                ));
            }
            board.push_str("\n");
        }
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
                        Some(p) => p.to_string(),
                        None => ".".to_string(),
                    }
                ));
            }
            board.push_str("\n");
        }

        let board_info = format!("{:?}", self.info);
        board.push_str(&board_info);
        write!(f, "{}", board)
    }
}
