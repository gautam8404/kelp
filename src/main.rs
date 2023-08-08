mod kelp;

use crate::kelp::board::fen::FenParse;
use crate::kelp::board::piece::Color;
use crate::kelp::mov_gen::generator::MovGen;
use kelp::kelp_core::bitboard::BitBoard;
use kelp::kelp_core::lookup_table::LookupTable;
use kelp::Squares::{self, *};
use kelp::{BLACK_OCCUPIED, OCCUPIED, WHITE_OCCUPIED};

use kelp::board::{board::Board, fen::Fen};

fn main() {
    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR \
    b KQkq - 0 1";
    let fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";

    // let mut board = Board::parse(Fen(starring_fen.to_string())).unwrap();
    // let mut mov_gen = MovGen::new();
    //
    // let mut bit_arr = [BitBoard::empty(); 12];
    // bit_arr = board.bitboards;
    // mov_gen.is_attacked(Squares::A3,Color::White, &board);

    let mut table = LookupTable::new();
    table.populate();

    let mut board = Board::parse(Fen(fein.to_string())).unwrap();
    println!("{}", board.get_occ());

    println!("{}", board);
    // println!(
    //     "{}",
    //     table.get_rook_attacks(Squares::E6 as u8, board.get_occ())
    // );
    // println!("{}", table.get_bishop_attacks(Squares::D5 as u8, board.get_occ()));
    let movgen = MovGen { table: &table };
    movgen.print_attacked(Color::White, &board);
}
