mod kelp;
use crate::kelp::board::piece::Color;
use kelp::board::bitboard::BitBoard;
use kelp::{WHITE_OCCUPIED, BLACK_OCCUPIED, OCCUPIED};
use kelp::board::lookup_table::LookupTable;

fn main() {


    let mut table : LookupTable = LookupTable::new();
    table.populate();
     let board = BitBoard::empty();

    // print!("{}", table.get_bishop_attacks(0, board));
    println!("{}", kelp::board::piece::BoardPiece::WhiteBishop as u8);


}
