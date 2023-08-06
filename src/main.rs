mod kelp;
use crate::kelp::board::piece::Color;
use kelp::board::bitboard::BitBoard;
use kelp::board::generate_attacks::*;
use kelp::*;
use kelp::board::lookup_table::LookupTable;

fn main() {


    let mut table : LookupTable = LookupTable::new();
    table.populate();
     let board = BitBoard::empty();

    print!("{}", table.get_bishop_attacks(0, board));



}
