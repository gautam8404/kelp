use kelp_engine::kelp::board::{
    board::Board,
    fen::{Fen, FenParse},
};
use kelp_engine::kelp::kelp_core::lookup_table::LookupTable;
use kelp_engine::kelp::mov_gen::{generator::MovGen, perft::*};
fn print_usage() {
    println!("Usage: kelp_perft <depth> <fen>");
}
fn main() {
    let mut table = LookupTable::new();
    table.populate();
    let mut movgen = MovGen::new(&table);
    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let args = std::env::args().collect::<Vec<String>>();
    let depth = args.get(1);
    let fen = args.get(2);

    if depth.is_none() || fen.is_none() {
        print_usage();
        return;
    }

    let depth = depth.unwrap().parse::<u16>();
    let fen = fen.unwrap();

    if depth.is_err() {
        print_usage();
        return;
    }

    let depth = depth.unwrap();
    let fen = fen.to_string();

    let mut board = Board::parse(Fen(fen)).unwrap();
    let mut nodes = 0;
    perft_test(depth, &mut board, &mut movgen, &mut nodes);
}
