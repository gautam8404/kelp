use super::piece::{BoardPiece, Color};
use crate::kelp::board::board::Board;
use crate::kelp::board::piece::BoardPiece::{BlackPawn, WhitePawn};
use crate::kelp::SideToMove::Black;
use crate::kelp::{Squares, MAX_SIZE_MOVES_ARR};
use log;
use std::fmt::{Debug, Display};
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
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MoveType {
    Normal,
    DoublePawnPush,
    EnPassant,
    Castle(CastlingRights),
    Promotion(Option<BoardPiece>),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GenType {
    Quiet,
    Capture,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    pub from: Squares,
    pub to: Squares,
    pub piece: BoardPiece,
    pub capture: Option<BoardPiece>,
    pub move_type: MoveType,
    pub gen_type: GenType,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.move_type {
            // MoveType::EnPassant => write!(f, "{}{}e.p.", self.from, self.to),
            MoveType::Promotion(Some(promotion)) => {
                write!(
                    f,
                    "{}{}{}",
                    self.from,
                    self.to,
                    promotion.to_string().to_lowercase()
                )
            }
            _ => write!(f, "{}{}", self.from, self.to),
        }
    }
}

impl Move {
    pub fn new(
        from: Squares,
        to: Squares,
        piece: BoardPiece,
        capture: Option<BoardPiece>,
        move_type: MoveType,
        gen_type: GenType,
    ) -> Self {
        Move {
            from,
            to,
            piece,
            capture,
            move_type,
            gen_type,
        }
    }

    pub fn new_promotion(
        from: Squares,
        to: Squares,
        piece: BoardPiece,
        capture: Option<BoardPiece>,
        promotion: Option<BoardPiece>,
        gen_type: GenType,
    ) -> Self {
        let move_type = MoveType::Promotion(promotion);
        Move {
            from,
            to,
            piece,
            capture,
            move_type,
            gen_type,
        }
    }

    pub fn set_type(&mut self, move_type: MoveType) {
        self.move_type = move_type;
    }

    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }

    pub fn is_promotion(&self) -> bool {
        matches!(self.move_type, MoveType::Promotion(_))
    }

    pub fn get_promotion(&self) -> Option<BoardPiece> {
        match self.move_type {
            MoveType::Promotion(promotion) => promotion,
            _ => None,
        }
    }

    pub fn is_en_passant(&self) -> bool {
        matches!(self.move_type, MoveType::EnPassant)
    }

    pub fn is_castle(&self) -> bool {
        matches!(self.move_type, MoveType::Castle(_))
    }
}

#[derive(Debug, Clone)]
pub struct MoveList(pub Vec<Move>);

impl MoveList {
    pub fn new() -> Self {
        MoveList(Vec::with_capacity(MAX_SIZE_MOVES_ARR))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MoveList(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, m: Move) {
        self.0.push(m);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn pop(&mut self) -> Option<Move> {
        self.0.pop()
    }

    pub fn last(&self) -> Option<&Move> {
        self.0.last()
    }

    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.0.iter()
    }

    pub fn extend(&mut self, other: &mut MoveList) {
        self.0.append(&mut other.0);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct MoveHistory {
    pub mov: Move,
    pub castle_rights: Castle,
    pub en_passant: Option<Squares>,
    pub half_move_clock: u8,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MoveArray {
    pub moves: [Option<MoveHistory>; MAX_SIZE_MOVES_ARR],
    pub count: usize,
}

impl MoveArray {
    pub fn new() -> Self {
        MoveArray {
            moves: [None; MAX_SIZE_MOVES_ARR],
            count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn push(&mut self, m: MoveHistory) {
        self.moves[self.count] = Some(m);
        self.count += 1;
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn pop(&mut self) -> Option<MoveHistory> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            self.moves[self.count]
        }
    }

    pub fn last(&self) -> Option<&MoveHistory> {
        if self.count == 0 {
            None
        } else {
            self.moves[self.count - 1].as_ref()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Option<MoveHistory>> {
        self.moves[..self.count].iter()
    }

    pub fn extend(&mut self, other: &mut MoveArray) {
        for m in other.iter() {
            self.push(m.unwrap());
        }
    }
}
