mod kelp;

use crate::kelp::board::fen::FenParse;
use crate::kelp::board::piece::Color;
use kelp::board::bitboard::BitBoard;
use kelp::board::lookup_table::LookupTable;
use kelp::Squares;
use kelp::{BLACK_OCCUPIED, OCCUPIED, WHITE_OCCUPIED};

use kelp::board::{board::Board, fen::Fen};

fn main() {
    // let mut table : LookupTable = LookupTable::new();
    // table.populate();
    //  let board = BitBoard::empty();
    //
    // // print!("{}", table.get_bishop_attacks(0, board));
    // println!("{}", kelp::board::piece::BoardPiece::WhiteBishop as u8);

    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR \
    b KQq a8 0 1";

    let mut fen = Fen::new(starring_fen.to_string());
    let board = Board::parse(fen).unwrap();
    println!("{}", board);
    println!("{:?}", board);

    let num = str_to_enum!("d4", Squares);

    println!("{}", num.unwrap());
}
