use super::board::Board;
use crate::kelp::BoardInfo;
use strum_macros::{Display, EnumIter, EnumString};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum FenParseError {
    InvalidFen(String),
    InvalidPiece(String),
    InvalidTurn(String),
    InvalidCastlingRights(String),
    InvalidEnPassant(String),
    InvalidHalfMoveClock(String),
    InvalidFullMoveClock(String),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Fen {
    pub fen: String,
}

pub trait FenParse<I, T, E>: Sized {
    fn parse(val: I) -> Result<T, E>;
}

impl Fen {
    pub fn new(fen: String) -> Fen {
        Fen { fen }
    }

    pub fn is_valid(&self) -> Result<(), FenParseError> {
        let parts: Vec<&str> = self.fen.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(FenParseError::InvalidFen(format!(
                "Invalid number of parts: {}, \
            A FEN string must have exactly 6 parts.",
                parts.len()
            )));
        }
        for c in parts[0].chars() {
            match c {
                'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' | '/' => {}
                '1'..='8' => {}
                _ => return Err(FenParseError::InvalidPiece(format!("Invalid piece: {}", c))),
            }
        }

        match parts[1] {
            "w" | "b" => {}
            _ => {
                return Err(FenParseError::InvalidTurn(format!(
                    "Invalid turn: {}, \
            must be 'w' or 'b'",
                    parts[1]
                )))
            }
        }

        for c in parts[2].chars() {
            match c {
                'K' | 'Q' | 'k' | 'q' | '-' => {}
                _ => {
                    return Err(FenParseError::InvalidCastlingRights(format!(
                        "Invalid castling rights: {}, \
                must be 'K', 'Q', 'k', 'q', or '-'",
                        c
                    )))
                }
            }
        }

        for c in parts[3].chars() {
            match c {
                'a'..='h' | '1'..='8' | '-' => {}
                _ => {
                    return Err(FenParseError::InvalidEnPassant(format!(
                        "Invalid en passant: {}, \
                must be a file (a-h) and rank (1-8) or '-'",
                        c
                    )))
                }
            }
        }

        match parts[4].parse::<u8>() {
            Ok(_) => {}
            Err(_) => {
                return Err(FenParseError::InvalidHalfMoveClock(format!(
                    "Invalid halfmove clock: {}, \
            must be a number that can be parsed as a u8",
                    parts[4]
                )))
            }
        }

        match parts[5].parse::<u8>() {
            Ok(_) => {}
            Err(_) => {
                return Err(FenParseError::InvalidFullMoveClock(format!(
                    "Invalid fullmove number: {}, \
            must be a number that can be parsed as a u8",
                    parts[5]
                )))
            }
        }

        Ok(())
    }
}
