pub mod board;
pub mod kelp;
pub mod kelp_core;
pub mod mov_gen;
pub mod search;
pub mod uci_trait;

use std::fmt::Debug;
use std::ops::{Add, Sub};
use strum_macros::{Display, EnumIter, EnumString, FromRepr};

use board::moves::Castle;
use board::piece::Color;
use kelp_core::bitboard::BitBoard;

pub type BitBoardArray = [BitBoard; 12];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GamePhase {
    Opening,
    MiddleGame,
    EndGame,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    Playing,
    Draw,
    Mate(Color),
    Stalemate,
}

pub const MAX_SIZE_MOVES_ARR: usize = 256;

// piece values

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 350;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 1000;
const KING_VALUE: i32 = 10000;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BoardInfo {
    turn: Color,
    pub castle: Castle,
    en_passant: Option<Squares>,
    halfmove_clock: u8,
    fullmove_clock: u16,
}

impl BoardInfo {
    pub fn toggle_turn(&mut self) {
        self.turn = !self.turn;
    }

    pub fn set_turn(&mut self, color: Color) {
        self.turn = color;
    }

    pub fn set_halfmove_clock(&mut self, value: u8) {
        self.halfmove_clock = value;
    }

    pub fn set_fullmove_clock(&mut self, value: u16) {
        self.fullmove_clock = value;
    }

    pub fn set_en_passant(&mut self, square: Option<Squares>) {
        self.en_passant = square;
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    pub fn get_fullmove_clock(&self) -> u16 {
        self.fullmove_clock
    }

    pub fn get_en_passant(&self) -> Option<Squares> {
        self.en_passant
    }
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromRepr, EnumIter, EnumString, Display, PartialOrd, Ord)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum Squares {
    A1 , B1, C1, D1, E1, F1, G1, H1,
    A2 , B2, C2, D2, E2, F2, G2, H2,
    A3 , B3, C3, D3, E3, F3, G3, H3,
    A4 , B4, C4, D4, E4, F4, G4, H4,
    A5 , B5, C5, D5, E5, F5, G5, H5,
    A6 , B6, C6, D6, E6, F6, G6, H6,
    A7 , B7, C7, D7, E7, F7, G7, H7,
    A8 , B8, C8, D8, E8, F8, G8, H8,
}

impl Squares {
    pub fn from_rank_file(rank: u8, file: u8) -> Squares {
        let index = (rank * 8 + file) as usize;
        Squares::from_repr(index as u8).unwrap()
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
    pub fn rank(&self) -> u8 {
        (self.to_index() / 8) as u8
    }

    pub fn file(&self) -> u8 {
        (self.to_index() % 8) as u8
    }

    pub fn mirror(&self) -> Squares {
        MIRROR[self.to_index()]
    }
}

impl Add<u8> for Squares {
    type Output = Squares;

    fn add(self, rhs: u8) -> Self::Output {
        let index = self.to_index() + rhs as usize;
        if index > 63 {
            panic!("Index out of bounds");
        }
        Squares::from_repr(index as u8).unwrap()
    }
}

impl Sub<u8> for Squares {
    type Output = Squares;

    fn sub(self, rhs: u8) -> Self::Output {
        let index = self.to_index() - rhs as usize;
        if index > 63 {
            panic!("Index out of bounds");
        }
        Squares::from_repr(index as u8).unwrap()
    }
}

use Squares::*;

#[rustfmt::skip]
const MIRROR: [Squares; 64] = [
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
];
