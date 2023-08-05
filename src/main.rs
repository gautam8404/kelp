mod kelp;
use crate::kelp::board::piece::Color;
use kelp::board::bitboard::BitBoard;
use kelp::board::generate_attacks::*;
use kelp::*;
use kelp::board::lookup_table::LookupTable;

fn main() {
    // let mut  test = BitBoard(0);
    // test.set_bit(Squares::D2 as u8);
    // let board = generate_rook_attacks(Squares::D5 as usize, test);
    // println!("{}", board);
    let mut board = BitBoard(0);
    board.set_bit(Squares::D5 as u8);

    // let at = generate_rook_attacks(0 as usize, board);
    // println!("{}", at);

    let mut table : LookupTable = LookupTable::new();
    table.populate();

    let a = table.get_rook_attacks(0, board);
    println!("{}", a);

}
