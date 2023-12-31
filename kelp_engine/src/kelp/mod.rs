pub mod board;
pub mod kelp;
pub mod kelp_core;
pub mod mov_gen;
pub mod search;
pub mod uci_trait;

use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};
use strum_macros::{Display, EnumIter, EnumString, FromRepr};

use board::moves::Castle;
use board::piece::Color;
use kelp_core::bitboard::BitBoard;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
pub static STOP: AtomicBool = AtomicBool::new(false);

pub fn stop_interval(duration: Duration) {
    thread::spawn(move || {
        thread::sleep(duration);
        STOP.store(true, Ordering::Relaxed);
    });
}

pub type BitBoardArray = [BitBoard; 12];
pub type ZobristKey = u64;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GamePhase {
    Opening = 0,
    EndGame = 1,
    MiddleGame = 2,
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

#[rustfmt::skip]
pub const MIRROR: [Squares; 64] = [
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
];

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

use crate::kelp::board::moves::Move;
use Squares::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct TimeControl {
    pub wtime: Option<i128>,
    pub btime: Option<i128>,
    pub winc: i128,
    pub binc: i128,
    pub movestogo: Option<u32>,
    pub movetime: Option<i128>,
    pub infinite: bool,
}

impl TimeControl {
    pub const MOVES_TO_GO: i128 = 40;
    const SAFETY_MARGIN: i128 = 50;
    const MAX_USAGE: f64 = 0.8;

    fn calculate_time(&mut self, color: Color) -> Option<i128> {
        if self.infinite {
            return None;
        }

        let mut time = match color {
            Color::White => self.wtime,
            Color::Black => self.btime,
        };

        let inc = match color {
            Color::White => self.winc,
            Color::Black => self.binc,
        };

        let mut moves_to_go = self.movestogo.unwrap_or(TimeControl::MOVES_TO_GO as u32) as i128;

        if self.movetime.is_some() {
            time = self.movetime;
            moves_to_go = 1;
        }

        if time.is_some() {
            let mut t_time = time.unwrap();
            t_time /= moves_to_go;
            t_time += inc;
            t_time -= TimeControl::SAFETY_MARGIN;
            t_time = t_time.max(0);
            time = Some(t_time);
        } else if inc > 0 {
            time = Some(inc);
        }

        if time.unwrap_or(0) < 0 {
            time = Some(0);
        }

        time
    }
}

pub struct SearchMoveResult {
    pub best_move: Option<Move>,
    pub score: i32,
    pub depth: usize,
    pub nodes: u64,
    pub time: Duration,
    pub nps: u64,
    pub pv: String,
    pub mate_in: Option<i32>,
}
impl Display for SearchMoveResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut info = String::new();

        info.push_str(format!("depth {} ", self.depth).as_str());
        info.push_str(format!("score cp {} ", self.score).as_str());
        info.push_str(format!("nodes {} ", self.nodes).as_str());
        info.push_str(format!("time {} ", self.time.as_millis()).as_str());
        info.push_str(format!("nps {} ", self.nps).as_str());

        if self.mate_in.is_some() {
            info.push_str(format!("mate {} ", self.mate_in.unwrap()).as_str());
        }

        info.push_str(format!("pv {}", self.pv).as_str());

        write!(f, "{}", info)
    }
}

pub struct SearchMoveResultExtended {
    pub best_move: Option<Move>,
    pub score: i32,
    pub depth: usize,
    pub nodes: u64,
    pub time: Duration,
    pub nps: u64,
    pub pv: String,
    pub mate_in: Option<i32>,
    pub hash_full: usize,
    pub tb_hits: usize,
    pub misses: usize,
    pub size: f64,
}

impl Display for SearchMoveResultExtended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut info = String::new();

        info.push_str(format!("depth {} ", self.depth).as_str());
        info.push_str(format!("score cp {} ", self.score).as_str());
        info.push_str(format!("nodes {} ", self.nodes).as_str());
        info.push_str(format!("time {} ", self.time.as_millis()).as_str());
        info.push_str(format!("nps {} ", self.nps).as_str());

        if self.mate_in.is_some() {
            info.push_str(format!("mate {} ", self.mate_in.unwrap()).as_str());
        }

        info.push_str(format!("hashfull {} ", self.hash_full).as_str());
        info.push_str(format!("tbhits {} ", self.tb_hits).as_str());
        info.push_str(format!("misses {} ", self.misses).as_str());
        info.push_str(format!("size {:.2} ", self.size).as_str());
        info.push_str(format!("pv {}", self.pv).as_str());

        write!(f, "{}", info)
    }
}
