mod kelp;

use crate::kelp::board::fen::FenParse;
use crate::kelp::board::piece::Color;
use kelp::kelp_core::bitboard::BitBoard;
use kelp::kelp_core::lookup_table::LookupTable;
use kelp::Squares::{self, *};
use kelp::{BLACK_OCCUPIED, OCCUPIED, WHITE_OCCUPIED};

use kelp::board::{board::Board, fen::Fen};

fn main() {

    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR \
    b KQkq - 0 1";

    let mut fen = Fen::new(starring_fen.to_string());
    let board = Board::parse(fen).unwrap();
    println!("{}", board);
    println!("{:?}", board);
    use std::str::FromStr;
    let mut num = Squares::from_str("a1");
    println!("{}", num.unwrap());
}
