use super::bitboard::BitBoard;
use super::generate_attacks::*;
use super::magics::*;
use super::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS};
use crate::kelp::board::piece::Color;
use log::info;

pub struct LookupTable {
    pawn_attacks: Vec<Vec<BitBoard>>,
    knight_attacks: Vec<BitBoard>,
    king_attacks: Vec<BitBoard>,

    bishop_attacks: Vec<Vec<BitBoard>>, // 64 * 512
    rook_attacks: Vec<Vec<BitBoard>>, // 64 * 4096

    magic_table: MagicTable,
}

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}

impl LookupTable {
    pub fn populate(&mut self) {
        self.magic_table.generate_magic_table();
        self.init_leaper_pieces();
        self.init_slider_pieces();
        info!("Lookup table populated");
    }

    fn init_leaper_pieces(&mut self) {
        for i in 0..64 {
            self.pawn_attacks[Color::White as usize][i] = generate_pawn_attacks(Color::White, i);
            self.pawn_attacks[Color::Black as usize][i] = generate_pawn_attacks(Color::Black, i);
            self.knight_attacks[i] = generate_knight_attacks(i);
            self.king_attacks[i] = generate_king_attacks(i);
        }
        info!("Leaper pieces initialized")
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
                self.bishop_attacks[sq][magic_index as usize] =
                    generate_bishop_attacks(sq, occupancy);
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
        info!("Slider pieces initialized")
    }

    pub fn new() -> LookupTable {
        LookupTable {
            pawn_attacks: vec![vec![BitBoard(0); 64]; 2],
            knight_attacks: vec![BitBoard(0); 64],
            king_attacks: vec![BitBoard(0); 64],
            bishop_attacks: vec![vec![BitBoard(0); 512]; 64],
            rook_attacks: vec![vec![BitBoard(0); 4096]; 64],
            magic_table: MagicTable::new(),
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
