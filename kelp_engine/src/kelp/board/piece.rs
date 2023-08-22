use crate::kelp::{BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE};
use std::ops::Not;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display)]
pub enum Color {
    White = 0,
    Black = 1,
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

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Display, EnumIter, EnumString,
)]
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
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => {
                Color::White
            }
            BlackPawn | BlackKnight | BlackBishop | BlackRook | BlackQueen | BlackKing => {
                Color::Black
            }
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

    pub fn get_value(&self) -> i32 {
        use BoardPiece::*;
        match self {
            WhitePawn | BlackPawn => PAWN_VALUE,
            WhiteKnight | BlackKnight => KNIGHT_VALUE,
            WhiteBishop | BlackBishop => BISHOP_VALUE,
            WhiteRook | BlackRook => ROOK_VALUE,
            WhiteQueen | BlackQueen => QUEEN_VALUE,
            WhiteKing | BlackKing => KING_VALUE,
        }
    }
}

impl From<u8> for BoardPiece {
    fn from(piece: u8) -> Self {
        use BoardPiece::*;
        match piece {
            0 => WhitePawn,
            1 => WhiteKnight,
            2 => WhiteBishop,
            3 => WhiteRook,
            4 => WhiteQueen,
            5 => WhiteKing,
            6 => BlackPawn,
            7 => BlackKnight,
            8 => BlackBishop,
            9 => BlackRook,
            10 => BlackQueen,
            11 => BlackKing,
            _ => panic!("Invalid piece: {}", piece),
        }
    }
}

impl From<char> for BoardPiece {
    fn from(piece: char) -> Self {
        use BoardPiece::*;
        match piece {
            'P' => WhitePawn,
            'N' => WhiteKnight,
            'B' => WhiteBishop,
            'R' => WhiteRook,
            'Q' => WhiteQueen,
            'K' => WhiteKing,
            'p' => BlackPawn,
            'n' => BlackKnight,
            'b' => BlackBishop,
            'r' => BlackRook,
            'q' => BlackQueen,
            'k' => BlackKing,
            _ => panic!("Invalid piece: {}", piece),
        }
    }
}
