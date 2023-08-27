mod kelp;
extern crate log;
extern crate simplelog;

use crate::kelp::kelp::Kelp;
use crate::kelp::search::eval::Eval;
use crate::kelp::uci_trait::UCI;
use crate::kelp::{MIRROR, Squares};
use kelp::kelp_core::lookup_table::LookupTable;
use kelp_engine::kelp::Squares::{B2, G1};
use simplelog::{Config, LevelFilter, WriteLogger};
use crate::kelp::board::board::Board;
use crate::kelp::board::fen::{Fen, FenParse};
use crate::kelp::mov_gen::generator::MovGen;
// use crate::kelp::search::BISHOP_SCORE;
use crate::kelp::Squares::{C1, E8};

fn main() {
    let file_path = std::env::var("KELP_LOG");
    if file_path.is_ok() {
        let file_path = file_path.unwrap();
        let _ = WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            std::fs::File::create(file_path).unwrap(),
        );
    }

    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";
    let fein = "r3k2r/p1ppqPb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBpPP/R3K2R w Kkq - 0 1";
    let fein = "r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ";
    let tricky = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
    let empty = "8/8/8/8/8/8/8/8 w KQ - 0 1";

    let d_fen = "rn6/pppppppp/8/8/8/8/PPPPPPPP/RN6 w KQkq - 0 1";

    // let mut table = LookupTable::default();
    // let mut kelp = Kelp::new(&mut table);
    // kelp.uci_loop();

    let debug = false;

    if debug {
        let eval = Eval::default();
        let board = Board::parse(Fen(d_fen.to_string())).unwrap();
        let mut table = LookupTable::default();
        table.populate();
        let movgen = MovGen::new(&table);

        println!("{}", board);
        println!("{}", eval.evaluate(&board, &movgen));

        println!("{}", Squares::A2.rank());
        println!("{}", Squares::A7.rank());
        // for i in 0..8 {
        //     // println!("{}", eval.file_mask[i]);
        //     println!("{}", eval.isolated_mask[i])
        // }
        // println!("{}", eval.passed_mask[1][Squares::A8 as usize]);

    } else {
        let mut table = LookupTable::default();
        let mut kelp = Kelp::new(&mut table);
        kelp.uci_loop();
    }
}
