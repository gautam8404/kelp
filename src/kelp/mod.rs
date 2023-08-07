pub mod board;
// pub mod kelp;

use std::fmt::Debug;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

use board::bitboard::BitBoard;
use board::piece::BoardPiece;
use board::piece::Color;

pub type BitBoardArray = [BitBoard; 12];

pub enum GamePhase {
    Opening,
    MiddleGame,
    EndGame,
}

pub enum SideToMove {
    White,
    Black,
}

pub const WHITE_OCCUPIED: u64 = 0x000000000000FFFF;
pub const BLACK_OCCUPIED: u64 = 0xFFFF000000000000;
pub const OCCUPIED: u64 = 0xFFFF00000000FFFF;

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
pub struct Castle(u8);

impl Debug for Castle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut castle = String::new();
        if self.can_castle_king_side(Color::White) {
            castle.push_str("K");
        }
        if self.can_castle_queen_side(Color::White) {
            castle.push_str("Q");
        }
        if self.can_castle_king_side(Color::Black) {
            castle.push_str("k");
        }
        if self.can_castle_queen_side(Color::Black) {
            castle.push_str("q");
        }
        write!(f, "Castle({})", castle)
    }
}

impl Castle {
    fn new() -> Self {
        Castle(
            CastlingRights::WhiteKingSide as u8
                | CastlingRights::WhiteQueenSide as u8
                | CastlingRights::BlackKingSide as u8
                | CastlingRights::BlackQueenSide as u8,
        )
    }
    fn remove(&mut self, castle: CastlingRights) {
        self.0 &= !(castle as u8);
    }

    fn add(&mut self, castle: CastlingRights) {
        self.0 |= castle as u8;
    }
    fn can_castle(&self, castle: CastlingRights) -> bool {
        self.0 & (castle as u8) != 0
    }

    fn can_castle_king_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle(CastlingRights::WhiteKingSide),
            Color::Black => self.can_castle(CastlingRights::BlackKingSide),
        }
    }

    fn can_castle_queen_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle(CastlingRights::WhiteQueenSide),
            Color::Black => self.can_castle(CastlingRights::BlackQueenSide),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BoardInfo {
    pub turn: Color,
    pub castling_rights: Castle,
    pub en_passant: Option<Squares>,
    pub halfmove_clock: u8,
    pub fullmove_clock: u8,
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

pub enum GameResult {
    Mate(Color),
    Stalemate,
    InsufficientMaterial,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[rustfmt::skip]
pub enum Squares {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2 = 8, B2, C2, D2, E2, F2, G2, H2,
    A3 = 16, B3, C3, D3, E3, F3, G3, H3,
    A4 = 24, B4, C4, D4, E4, F4, G4, H4,
    A5 = 32, B5, C5, D5, E5, F5, G5, H5,
    A6 = 40, B6, C6, D6, E6, F6, G6, H6,
    A7 = 48, B7, C7, D7, E7, F7, G7, H7,
    A8 = 56, B8, C8, D8, E8, F8, G8, H8,
}

#[macro_export]
macro_rules! str_to_enum {
    ($s:expr, $enum_ty:ty) => {{
        use std::str::FromStr;

        match <$enum_ty>::from_str($s) {
            Ok(variant) => Ok(variant),
            Err(_) => Err(format!("Invalid {} value: {}", stringify!($enum_ty), $s)),
        }
    }};
}
