use super::piece::{BoardPiece, Color};
use std::fmt::Debug;
use strum_macros::{Display, EnumIter, EnumString};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display, EnumIter, EnumString)]
pub enum CastlingRights {
    #[strum(serialize = "K")]
    WhiteKingSide = 1,
    #[strum(serialize = "Q")]
    WhiteQueenSide = 2,
    #[strum(serialize = "k")]
    BlackKingSide = 4,
    #[strum(serialize = "q")]
    BlackQueenSide = 8,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Castle(pub u8);

impl Debug for Castle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut castle = String::new();
        if self.can_castle_king_side(Color::White) {
            castle.push('K');
        }
        if self.can_castle_queen_side(Color::White) {
            castle.push('Q');
        }
        if self.can_castle_king_side(Color::Black) {
            castle.push('k');
        }
        if self.can_castle_queen_side(Color::Black) {
            castle.push('q');
        }
        write!(f, "Castle({})", castle)
    }
}

impl Castle {
    pub fn new() -> Self {
        Castle(
            CastlingRights::WhiteKingSide as u8
                | CastlingRights::WhiteQueenSide as u8
                | CastlingRights::BlackKingSide as u8
                | CastlingRights::BlackQueenSide as u8,
        )
    }
    pub fn remove(&mut self, castle: CastlingRights) {
        self.0 &= !(castle as u8);
    }

    pub fn add(&mut self, castle: CastlingRights) {
        self.0 |= castle as u8;
    }
    pub fn can_castle(&self, castle: CastlingRights) -> bool {
        self.0 & (castle as u8) != 0
    }

    pub fn can_castle_king_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle(CastlingRights::WhiteKingSide),
            Color::Black => self.can_castle(CastlingRights::BlackKingSide),
        }
    }

    pub fn can_castle_queen_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle(CastlingRights::WhiteQueenSide),
            Color::Black => self.can_castle(CastlingRights::BlackQueenSide),
        }
    }
}

pub enum MoveType {
    Normal,
    DoublePawnPush,
    EnPassant,
    Castle(CastlingRights),
    Promotion(Option<BoardPiece>),
}

pub struct Move {
    pub from: u8,
    pub to: u8,
    pub move_type: MoveType,
}
