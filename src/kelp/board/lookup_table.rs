use std::process::id;
use super::bitboard::BitBoard;
use super::generate_attacks::*;
use super::piece::Color;
use super::magics::*;
use super::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS};

pub struct LookupTable {
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub knight_attacks: [BitBoard; 64],
    pub king_attacks: [BitBoard; 64],

    pub bishop_masks: [BitBoard; 64],
    pub rook_masks: [BitBoard; 64],
    pub bishop_attacks: [[BitBoard; 1024]; 64],
    pub rook_attacks: [[BitBoard; 4096]; 64],

    pub magic_table: MagicTable,
}

impl LookupTable {
    pub fn populate(&mut self) {
        self.init_leaper_pieces();
        self.init_slider_pieces();
    }

    fn init_leaper_pieces(&mut self) {
        for i in 0..64 {
            self.pawn_attacks[Color::White as usize][i] = generate_pawn_attacks(Color::White, i);
            self.pawn_attacks[Color::Black as usize][i] = generate_pawn_attacks(Color::Black, i);
            self.knight_attacks[i] = generate_knight_attacks(i);
            self.king_attacks[i] = generate_king_attacks(i);
        }
    }


    fn init_slider_pieces(&mut self) {
        let magic_table = self.magic_table;

        // Bishop
        for sq in 0..64 {
            let occ_idx = (1 << BISHOP_RELEVANT_BITS[sq]) as u32;
            for idx in 0..occ_idx {
                let magic = magic_table.bishop[sq];
                let occupancy = set_occupancy(idx, BISHOP_RELEVANT_BITS[sq], magic.mask);
                let magic_index = ((occupancy * magic.magic) >> magic.shift).0 as u16;
                self.bishop_attacks[sq][magic_index as usize] = generate_bishop_attacks(sq, occupancy);
            }
        }

        // Rook
        for sq in 0..64 {
            let occ_idx = (1 << ROOK_RELEVANT_BITS[sq]) as u32;
            for idx in 0..occ_idx {
                let magic = magic_table.rook[sq];
                let occupancy = set_occupancy(idx, ROOK_RELEVANT_BITS[sq], magic.mask);
                let magic_index = ((occupancy * magic.magic) >> magic.shift).0 as u16;
                self.rook_attacks[sq][magic_index as usize] = generate_rook_attacks(sq, occupancy);
            }
        }

    }

    pub fn new() -> LookupTable {
        LookupTable {
            pawn_attacks: [[BitBoard(0); 64]; 2],
            knight_attacks: [BitBoard(0); 64],
            king_attacks: [BitBoard(0); 64],
            bishop_masks: [BitBoard(0); 64],
            rook_masks: [BitBoard(0); 64],
            bishop_attacks: [[BitBoard(0); 1024]; 64],
            rook_attacks: [[BitBoard(0); 4096]; 64],
            magic_table: generate_magic_table(),
        }
    }

    #[inline(always)]
    pub fn get_pawn_attacks(&self, color: Color, square: u8) -> BitBoard {
        self.pawn_attacks[color as usize][square as usize]
    }

    #[inline(always)]
    pub fn get_knight_attacks(&self, square: u8) -> BitBoard {
        self.knight_attacks[square as usize]
    }

    #[inline(always)]
    pub fn get_king_attacks(&self, square: u8) -> BitBoard {
        self.king_attacks[square as usize]
    }

    #[inline(always)]
    pub fn get_bishop_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        let magic = self.magic_table.bishop[square as usize];
        let mut occ = occupancy;
        occ &= magic.mask;
        occ *= magic.magic;
        occ >>= magic.shift;
        self.bishop_attacks[square as usize][occ.0 as usize]
    }

    #[inline(always)]
    pub fn get_rook_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        let magic = self.magic_table.rook[square as usize];
        let mut occ = occupancy;
        occ &= magic.mask;
        occ *= magic.magic;
        occ >>= magic.shift;
        self.rook_attacks[square as usize][occ.0 as usize]
    }

    #[inline(always)]
    pub fn get_queen_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy)
    }
}
