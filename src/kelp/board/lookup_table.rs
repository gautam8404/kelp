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
        let magic_table = generate_magic_table();

        for i in 0..64 {
            self.bishop_masks[i] = generate_bishop_mask(i);
            self.rook_masks[i] = generate_rook_mask(i);
        }

        // Bishop
        for sq in 0..64 {
            let magic = magic_table.bishop[sq];
            let bishop_mask = self.bishop_masks[sq];
            let attack_mask = bishop_mask;

            let relevant_bits  = attack_mask.count_bits();
            let occ_idx: u32 = (1 << relevant_bits);

            for idx in 0..occ_idx {
                let occupancy = set_occupancy(idx, relevant_bits, attack_mask);
                let magic_index: u16 = ((occupancy.0.wrapping_mul(magic.magic.0)) >> magic.shift) as u16;
                self.bishop_attacks[sq][magic_index as usize] = generate_bishop_attacks(sq, occupancy);
            }
        }

        // Rook
        for sq in 0..64 {
            let magic = magic_table.rook[sq];
            let rook_mask = self.rook_masks[sq];
            let attack_mask = rook_mask;

            let relevant_bits  = attack_mask.count_bits();
            let occ_idx: u32 = (1 << relevant_bits);

            for idx in 0..occ_idx {
                let occupancy = set_occupancy(idx, relevant_bits, attack_mask);
                let magic_index = ((occupancy.0.wrapping_mul(magic.magic.0)) >> magic.shift) as u16;
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
        let mut occ = occupancy.0;
        occ &= self.bishop_masks[square as usize].0;
        occ = occ.wrapping_mul(BISHOP_MAGICS[square as usize]);
        occ >>= 64 - BISHOP_RELEVANT_BITS[square as usize];
        self.bishop_attacks[square as usize][occ as usize]
    }

    #[inline(always)]
    pub fn get_rook_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        let mut  occ = occupancy.0;
        occ &= self.rook_masks[square as usize].0;
        occ = occ.wrapping_mul(ROOK_MAGICS[square as usize]);
        occ >>= 64 - ROOK_RELEVANT_BITS[square as usize];
        self.rook_attacks[square as usize][occ as usize]
    }

    #[inline(always)]
    pub fn get_queen_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy)
    }
}
