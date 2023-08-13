use crate::kelp::board::board::Board;
use crate::kelp::board::piece::Color;
use crate::kelp::Squares;
use std::fmt::Display;

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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Fen(pub String);

impl Display for Fen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub trait FenParse<I, O, E>: Sized {
    fn parse(val: I) -> Result<O, E>;
}

impl FenParse<&Board, Fen, FenParseError> for Fen {
    fn parse(val: &Board) -> Result<Fen, FenParseError> {
        let mut fen = String::new();
        let mut empty = 0;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                if let Some(piece) = &val.get_piece(Squares::from_repr(sq as u8).unwrap()) {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push_str(&piece.to_string());
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push_str(if val.info.turn == Color::White {
            "w"
        } else {
            "b"
        });
        fen.push(' ');
        let mut castle = String::new();
        if val.info.castle.can_castle_king_side(Color::White) {
            castle.push('K');
        }
        if val.info.castle.can_castle_queen_side(Color::White) {
            castle.push('Q');
        }
        if val.info.castle.can_castle_king_side(Color::Black) {
            castle.push('k');
        }
        if val.info.castle.can_castle_queen_side(Color::Black) {
            castle.push('q');
        }
        if castle.is_empty() {
            castle.push('-');
        }
        fen.push_str(&castle);
        fen.push(' ');
        let en_passant = val.info.en_passant;
        if let Some(..) = en_passant {
            fen.push_str(&en_passant.unwrap().to_string());
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(&val.info.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&val.info.fullmove_clock.to_string());
        Ok(Fen(fen))
    }
}

impl Fen {
    pub fn new(fen: String) -> Fen {
        Fen(fen)
    }

    pub fn is_valid(&self) -> Result<(), FenParseError> {
        let parts: Vec<&str> = self.0.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 6 && parts.len() != 4 {
            return Err(FenParseError::InvalidFen(format!(
                "Invalid number of parts: {}, \
        A FEN string must have exactly 6 or 4 parts separated by whitespace",
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

        if parts.len() > 4 {
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
        }

        Ok(())
    }
}
