use super::bitboard::BitBoard;
use super::piece::Color;


const FILE_A: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
const FILE_B: u64 = 0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
const FILE_G: u64 = 0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
const FILE_H: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
const FILE_AB: u64 = FILE_A | FILE_B;
const FILE_GH: u64 = FILE_G | FILE_H;
const NOT_FILE_A: u64 = !FILE_A;
const NOT_FILE_B: u64 = !FILE_B;
const NOT_FILE_G: u64 = !FILE_G;
const NOT_FILE_H: u64 = !FILE_H;
const NOT_FILE_AB: u64 = !FILE_AB;
const NOT_FILE_GH: u64 = !FILE_GH;

pub fn generate_pawn_attacks(color: Color, square: usize) -> BitBoard {
    let mut attacks = BitBoard(0);
    let mut square_bb = BitBoard(0);
    square_bb.set_bit(square as u8);

    if color == Color::Black {
        attacks |= BitBoard((square_bb.0 >> 9) & NOT_FILE_H);
        attacks |= BitBoard((square_bb.0 >> 7) & NOT_FILE_A);
    } else {
        attacks |= BitBoard((square_bb.0 << 9) & NOT_FILE_A);
        attacks |= BitBoard((square_bb.0 << 7) & NOT_FILE_H);
    }
    attacks
}

pub fn generate_king_attacks(square: usize) -> BitBoard {
    let mut attacks = BitBoard(0);
    let mut square_bb = BitBoard(0);
    square_bb.set_bit(square as u8);

    attacks |= BitBoard((square_bb.0 >> 1) & NOT_FILE_H);
    attacks |= BitBoard((square_bb.0 << 1) & NOT_FILE_A);
    attacks |= BitBoard(square_bb.0 >> 8);
    attacks |= BitBoard(square_bb.0 << 8);
    attacks |= BitBoard((square_bb.0 >> 9) & NOT_FILE_H);
    attacks |= BitBoard((square_bb.0 << 9) & NOT_FILE_A);
    attacks |= BitBoard((square_bb.0 >> 7) & NOT_FILE_A);
    attacks |= BitBoard((square_bb.0 << 7) & NOT_FILE_H);

    attacks
}

pub fn generate_knight_attacks(square: usize) -> BitBoard {
    let mut attacks = BitBoard(0);
    let mut square_bb = BitBoard(0);
    square_bb.set_bit(square as u8);

    // 17, 15, 10, 6, -6, -10, -15, -17
    if ((square_bb.0 >> 17) & NOT_FILE_H) != 0 {
        attacks |= BitBoard(square_bb.0 >> 17);
    }
    if ((square_bb.0 >> 15) & NOT_FILE_A) != 0 {
        attacks |= BitBoard(square_bb.0 >> 15);
    }

    if ((square_bb.0 >> 10) & NOT_FILE_GH) != 0 {
        attacks |= BitBoard(square_bb.0 >> 10);
    }
    if ((square_bb.0 >> 6) & NOT_FILE_AB) != 0 {
        attacks |= BitBoard(square_bb.0 >> 6);
    }

    if ((square_bb.0 << 17) & NOT_FILE_A) != 0 {
        attacks |= BitBoard(square_bb.0 << 17);
    }
    if ((square_bb.0 << 15) & NOT_FILE_H) != 0 {
        attacks |= BitBoard(square_bb.0 << 15);
    }
    if ((square_bb.0 << 10) & NOT_FILE_AB) != 0 {
        attacks |= BitBoard(square_bb.0 << 10);
    }
    if ((square_bb.0 << 6) & NOT_FILE_GH) != 0 {
        attacks |= BitBoard(square_bb.0 << 6);
    }

    attacks
}

pub fn generate_bishop_mask(square: usize) -> BitBoard {
    let mut mask = BitBoard(0);
    let (t_rank, t_file): (usize, usize) = (square / 8, square % 8);

    for (rank, file) in (t_rank + 1..7).zip(t_file + 1..7) {
        mask |= BitBoard(1u64 << (rank * 8 + file));
    }
    for (rank, file) in (t_rank + 1..7).zip((1..=t_file).rev()) {
        mask |= BitBoard(1u64 << (rank * 8 + file));
    }
    for (rank, file) in (1..=t_rank).rev().zip(t_file + 1..7) {
        mask |= BitBoard(1u64 << (rank * 8 + file));
    }
    for (rank, file) in (1..=t_rank).rev().zip((1..=t_file).rev()) {
        mask |= BitBoard(1u64 << (rank * 8 + file));
    }

    mask
}

pub fn generate_rook_mask(square: usize) -> BitBoard {
    let mut mask = BitBoard(0);
    let (t_rank, t_file): (usize, usize) = (square / 8, square % 8);

    for rank in (t_rank + 1)..7 {
        mask |= BitBoard(1u64 << (rank * 8 + t_file));
    }
    for rank in (1..=t_rank ).rev() {
        mask |= BitBoard(1u64 << (rank * 8 + t_file));
    }
    for file in (t_file + 1)..7 {
        mask |= BitBoard(1u64 << (t_rank * 8 + file));
    }
    for file in (1..=t_file ).rev() {
        mask |= BitBoard(1u64 << (t_rank * 8 + file));
    }

    mask
}

pub fn generate_bishop_attacks(square: usize, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard(0);
    let (mut t_rank,mut t_file): (usize, usize) = (square / 8, square % 8);


    // Top-right direction
    while t_rank < 7 && t_file < 7 {
        t_rank += 1;
        t_file += 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    (t_rank, t_file) = (square / 8, square % 8);

    // Top-left direction
    while t_rank < 7 && t_file > 0 {
        t_rank += 1;
        t_file -= 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    (t_rank, t_file) = (square / 8, square % 8);
    // Bottom-right direction
    while t_rank > 0 && t_file < 7 {
        t_rank -= 1;
        t_file += 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    (t_rank, t_file) = (square / 8, square % 8);
    // Bottom-left direction
    while t_rank > 0 && t_file > 0 {
        t_rank -= 1;
        t_file -= 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    attacks.clear_bit(square as u8);
    attacks
}

pub fn generate_rook_attacks(square: usize, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard(0);
    let (mut t_rank,mut t_file): (usize, usize) = (square / 8, square % 8);

    // Top direction
    while t_rank < 7 {
        t_rank += 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    (t_rank, t_file) = (square / 8, square % 8);
    // Bottom direction
    while t_rank > 0 {
        t_rank -= 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    (t_rank, t_file) = (square / 8, square % 8);
    // Right direction
    while t_file < 7 {
        t_file += 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }

    (t_rank, t_file) = (square / 8, square % 8);
    // Left direction
    while t_file > 0 {
        t_file -= 1;
        attacks |= BitBoard(1u64 << (t_rank * 8 + t_file));
        if blockers.get_bit((t_rank * 8 + t_file) as u8) {
            break;
        }
    }


    attacks.clear_bit(square as u8);
    attacks
}

pub fn set_occupancy(index: usize, mask: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard(0);
    let mut temp = mask;
    let count = temp.count_bits();

    for i in 0..count {
        let square = temp.pop_lsb();
        if index & (1 << i) != 0 {
            occupancy.set_bit(square);
        }
    }

    occupancy
}


























