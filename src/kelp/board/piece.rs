use std::ops::Not;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self {
        use Color::*;
        match self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display, EnumIter, EnumString)]
pub enum BoardPiece {
    #[strum(serialize = "P")]
    WhitePawn,
    #[strum(serialize = "N")]
    WhiteKnight,
    #[strum(serialize = "B")]
    WhiteBishop,
    #[strum(serialize = "R")]
    WhiteRook,
    #[strum(serialize = "Q")]
    WhiteQueen,
    #[strum(serialize = "K")]
    WhiteKing,
    #[strum(serialize = "p")]
    BlackPawn,
    #[strum(serialize = "n")]
    BlackKnight,
    #[strum(serialize = "b")]
    BlackBishop,
    #[strum(serialize = "r")]
    BlackRook,
    #[strum(serialize = "q")]
    BlackQueen,
    #[strum(serialize = "k")]
    BlackKing,
}

impl BoardPiece {
    pub fn get_color(&self) -> Color {
        use BoardPiece::*;
        match self {
            WhitePawn
            | WhiteKnight
            | WhiteBishop
            | WhiteRook
            | WhiteQueen
            | WhiteKing => Color::White,
            BlackPawn
            | BlackKnight
            | BlackBishop
            | BlackRook
            | BlackQueen
            | BlackKing => Color::Black,
        }
    }

    pub fn unicode(&self) -> &'static str {
        use BoardPiece::*;
        match self {
            WhitePawn => "♙",
            WhiteKnight => "♘",
            WhiteBishop => "♗",
            WhiteRook => "♖",
            WhiteQueen => "♕",
            WhiteKing => "♔",
            BlackPawn => "♟︎",
            BlackKnight => "♞",
            BlackBishop => "♝",
            BlackRook => "♜",
            BlackQueen => "♛",
            BlackKing => "♚",
        }
    }
}
