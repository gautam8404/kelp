mod kelp;
extern crate log;

use crate::kelp::kelp::Kelp;
use crate::kelp::uci_trait::UCI;
use kelp::kelp_core::lookup_table::LookupTable;



fn main() {
    let file_path = std::env::var("KELP_LOG");
    if let Ok(file_path) = file_path {
        let file_name = format!("kelp-{}.log", std::process::id());

        let full_path = std::path::Path::new(&file_path).join(&file_name);
        let file_path = full_path.to_str().unwrap();


        let logger = simple_logging::log_to_file(file_path, log::LevelFilter::Trace);
        if logger.is_err() {
            panic!("Failed to create log file: {}", logger.err().unwrap());
        }
    }

    let _starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let _tricky = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
    let _empty = "8/8/8/8/8/8/8/8 w KQ - 0 1";

    let mut table = LookupTable::default();
    let mut kelp = Kelp::new(&mut table);
    kelp.uci_loop();
}
