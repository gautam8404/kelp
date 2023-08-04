use super::bitboard::BitBoard;
use super::piece::BoardPiece;
use super::piece::Color;
use super::generate_attacks::*;

struct LookupTable {
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub knight_attacks: [BitBoard; 64],
    pub king_attacks: [BitBoard; 64],

    pub bishop_masks: [BitBoard; 64],
    pub rook_masks: [BitBoard; 64],
    pub bishop_attacks: [[BitBoard; 1024]; 64],
    pub rook_attacks: [[BitBoard; 4096]; 64],
}

