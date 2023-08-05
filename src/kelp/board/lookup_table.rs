use super::bitboard::BitBoard;
use super::generate_attacks::*;
use super::piece::Color;
use super::magics::{ROOK_MAGICS, BISHOP_MAGICS};
use super::{BISHOP_OCC_BITS, ROOK_OCC_BITS};

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
        for i in 0..64 {
            self.bishop_masks[i] = generate_bishop_mask(i);
            self.rook_masks[i] = generate_rook_mask(i);

            let (mut rbits, mut occ_index,mut magic): (u32, u32, u32);
            let (mut occupancy,mut attack_mask): (BitBoard, BitBoard);

            attack_mask = self.bishop_masks[i];
            rbits = attack_mask.count_bits() as u32;
            occ_index = (1 << rbits) as u32;

            for j in 0..occ_index {
                occupancy = set_occupancy(j as usize, attack_mask);
                magic = ((occupancy.0.wrapping_mul(BISHOP_MAGICS[i])) >> (64 - BISHOP_OCC_BITS[i])) as u32;
                self.bishop_attacks[i][magic as usize] = generate_bishop_attacks(i, occupancy);
            }

            attack_mask = self.rook_masks[i];
            rbits = attack_mask.count_bits() as u32;
            occ_index = (1 << rbits) as u32;

            for j in 0..occ_index {
                occupancy = set_occupancy(j as usize, attack_mask);
                magic = ((occupancy.0.wrapping_mul(ROOK_MAGICS[i])) >> (64 - ROOK_OCC_BITS[i])) as u32;
                self.rook_attacks[i][magic as usize] = generate_rook_attacks(i, occupancy);

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
        let occ = occupancy.0;
        let magic_val = BISHOP_MAGICS[square as usize];
        let shift = 64 - BISHOP_OCC_BITS[square as usize];
        let magic = ((occ.wrapping_mul(magic_val)).wrapping_shr(shift as u32)) as u32;
        self.bishop_attacks[square as usize][magic as usize]
    }

    #[inline(always)]
    pub fn get_rook_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        let occ = occupancy.0;
        let magic_val = ROOK_MAGICS[square as usize];
        let shift = 64 - ROOK_OCC_BITS[square as usize];
        let magic = ((occ.wrapping_mul(magic_val)).wrapping_shr(shift as u32)) as u32;
        self.rook_attacks[square as usize][magic as usize]
    }

    #[inline(always)]
    pub fn get_queen_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
        self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy)
    }
}
