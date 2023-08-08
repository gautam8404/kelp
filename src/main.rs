mod kelp;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::thread::sleep;
use crate::kelp::board::fen::FenParse;
use crate::kelp::board::piece::{BoardPiece, Color};
use crate::kelp::mov_gen::generator::MovGen;
use kelp::kelp_core::bitboard::BitBoard;
use kelp::kelp_core::lookup_table::LookupTable;
use kelp::Squares::{self, *};
use kelp::{BLACK_OCCUPIED, OCCUPIED, WHITE_OCCUPIED};

use kelp::board::{board::Board, fen::Fen};

fn main() {
    pretty_env_logger::init();
    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR \
    b KQkq - 0 1";
    let fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";
    let fein = "r3k2r/p1ppqPb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBpPP/R3K2R w KQkq - 0 1";
    let fein = "r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ";

    let mut table = LookupTable::new();
    table.populate();

    let mut board = Board::parse(Fen(fein.to_string()));
    if board.is_err() {
        println!("{:?}", board.err().unwrap());
        return;
    }
    let mut board = board.unwrap();

    let mut movgen = MovGen::new(&table);
    movgen.print_attacked(Color::White, &board);
    println!("{}\n", board);
    println!("{:?}", board);
    let time = std::time::Instant::now();
    movgen.generate_moves(Color::White, &board);
    println!("Time: {:?}", time.elapsed());
    for i in movgen.move_list.iter() {
        print!("{} ", i);
        println!("{:?}", i);
    }

    println!("{}", Squares::A8.rank());



    // print!("{}", board.get_piece_at(Squares::E6).unwrap());
}
