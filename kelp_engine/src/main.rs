mod kelp;
extern crate log;
extern crate simplelog;

use crate::kelp::kelp::Kelp;
use crate::kelp::uci_trait::UCI;
use kelp::kelp_core::lookup_table::LookupTable;
use simplelog::{Config, LevelFilter, WriteLogger};
use crate::kelp::board::fen::{FenParse};

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

    let _starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let _fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";
    let _fein = "r3k2r/p1ppqPb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBpPP/R3K2R w Kkq - 0 1";
    let _fein = "r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ";
    let _tricky = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
    let _empty = "8/8/8/8/8/8/8/8 w KQ - 0 1";

    let mut table = LookupTable::default();
    let mut kelp = Kelp::new(&mut table);
    kelp.uci_loop();
}
