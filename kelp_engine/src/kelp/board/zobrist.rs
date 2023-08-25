use super::piece::{BoardPiece, Color};
use crate::kelp::board::board::Board;
use crate::kelp::Squares;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use strum::IntoEnumIterator;
use crate::kelp::ZobristKey;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Zobrist {
    pub piece_keys: [[ZobristKey; 64]; 12],
    pub side_key: ZobristKey,
    pub castle_keys: [ZobristKey; 16],
    pub en_passant_keys: [ZobristKey; 64],
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(15 * 8 * 1947);

        let mut piece_keys = [[0; 64]; 12];

        for piece in BoardPiece::iter() {
            for sq in 0..64 {
                piece_keys[piece as usize][sq] = rng.next_u64();
            }
        }

        let side_key = rng.next_u64();

        let mut castle_keys = [0; 16];

        for i in 0..16 {
            castle_keys[i] = rng.next_u64();
        }

        let mut en_passant_keys = [0; 64];

        for i in 0..64 {
            en_passant_keys[i] = rng.next_u64();
        }

        Self {
            piece_keys,
            side_key,
            castle_keys,
            en_passant_keys,
        }
    }

    pub fn get_piece_key(&self, piece: BoardPiece, sq: Squares) -> ZobristKey {
        self.piece_keys[piece as usize][sq as usize]
    }

    pub fn get_side_key(&self) -> ZobristKey {
        self.side_key
    }

    pub fn get_castle_key(&self, castle: u8) -> ZobristKey {
        self.castle_keys[castle as usize]
    }

    pub fn get_en_passant_key(&self, sq: Squares) -> ZobristKey {
        self.en_passant_keys[sq as usize]
    }

    pub fn get_key(&self, board: &Board) -> ZobristKey {
        let mut key = 0;

        for sq in Squares::iter() {
            if let Some(piece) = board.get_piece(sq) {
                key ^= self.get_piece_key(piece, sq);
            }
        }

        if board.get_side_to_move() == Color::Black {
            key ^= self.get_side_key();
        }

        key ^= self.get_castle_key(board.info.castle.0);

        if let Some(sq) = board.info.en_passant {
            key ^= self.get_en_passant_key(sq);
        }

        key
    }
}
