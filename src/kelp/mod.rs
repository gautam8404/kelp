pub mod board;
pub mod kelp;
pub mod kelp_core;
pub mod mov_gen;

use std::fmt::Debug;
use strum_macros::{Display, EnumIter, EnumString, FromRepr};

use board::moves::Castle;
use board::piece::Color;
use kelp_core::bitboard::BitBoard;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BoardInfo {
    pub turn: Color,
    pub castling_rights: Castle,
    pub en_passant: Option<Squares>,
    pub halfmove_clock: u8,
    pub fullmove_clock: u8,
}

pub enum GameResult {
    Mate(Color),
    Stalemate,
    InsufficientMaterial,
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromRepr, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
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

impl Squares {
    pub fn from_rank_file(rank: u8, file: u8) -> Squares {
        let index = (rank * 8 + file) as usize;
        Squares::from_repr(index as u8).unwrap()
    }
}
