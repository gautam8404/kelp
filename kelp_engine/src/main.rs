mod kelp;
extern crate log;
extern crate simplelog;

use crate::kelp::kelp::Kelp;
use crate::kelp::search::eval::Eval;
use crate::kelp::uci_trait::UCI;
use crate::kelp::{MIRROR, Squares};
use kelp::kelp_core::lookup_table::LookupTable;
use kelp_engine::kelp::Squares::B2;
use simplelog::{Config, LevelFilter, WriteLogger};
use crate::kelp::Squares::E8;

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

    let d_fen = "r3k2r/p1Ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";

    // let mut table = LookupTable::default();
    // let mut kelp = Kelp::new(&mut table);
    // kelp.uci_loop();

    let debug = true;

    if debug {
        let eval = Eval::default();
        // for i in 0..8 {
        //     // println!("{}", eval.file_mask[i]);
        //     println!("{}", eval.isolated_mask[i])
        // }
        println!("{}", eval.passed_mask[1][Squares::A8 as usize]);

        let debug = true;
    } else {
        let mut table = LookupTable::default();
        let mut kelp = Kelp::new(&mut table);
        kelp.uci_loop();
    }
}
