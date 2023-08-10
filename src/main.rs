mod kelp;
extern crate env_logger;
#[macro_use]
extern crate log;

use crate::kelp::board::fen::FenParse;
use crate::kelp::board::piece::{BoardPiece, Color};
use crate::kelp::mov_gen::generator::MovGen;
use kelp::kelp_core::bitboard::BitBoard;
use kelp::kelp_core::lookup_table::LookupTable;
use kelp::Squares::{self, *};
use kelp::{BLACK_OCCUPIED, OCCUPIED, WHITE_OCCUPIED};
use std::thread::sleep;

use crate::kelp::board::piece::BoardPiece::BlackQueen;
use kelp::board::{board::Board, fen::Fen};
use crate::kelp::mov_gen::perft::perft;

fn main() {
    env_logger::init();
    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR \
    b KQkq - 0 1";
    let fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";
    let fein = "r3k2r/p1ppqPb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBpPP/R3K2R w Kkq - 0 1";
    let fein = "r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ";
    let tricky = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";

    let mut table = LookupTable::new();
    table.populate();

    let mut board = Board::parse(Fen(starring_fen.to_string()));
    if board.is_err() {
        println!("{:?}", board.err().unwrap());
        return;
    }
    let mut board = board.unwrap();
    // board.replace_piece(BoardPiece::BlackRook, E2);
    // println!("{}", board.get_piece(G6).unwrap());
    // println!("{}", board);

    let mut movgen = MovGen::new(&table);

    // movgen.print_attacked(Color::White, &board);
    // println!("{}\n", board.get_piece_occ(BlackQueen));
    // board.add_piece(BoardPiece::BlackRook, C1);
    // println!("{:?}", board);
    // println!("{}",  board.is_king_checked(Color::White, & movgen));
    // // let time = std::time::Instant::now();
    // movgen.generate_moves(Color::Black, &board);
    movgen.generate_moves(Color::White, &board);
    // // println!("Time: {:?}", time.elapsed());
    // for i in movgen.move_list.iter() {
    //     print!("{} ", i);
    //     println!("{:?}", i);
    // }

    // println!("mv length: {}", movgen.move_list.len());

    // println!("dsfbs");
    // for i in movgen.move_list.iter() {
    //     print!("{}  ", i);
    //     println!("{:?}", i);
    //     let a = board.make_move(*i, false);
    //     if a.is_none() {
    //         println!("Invalid move: {}", i);
    //         continue;
    //     }
    //     println!("{}", board);
    //     println!("{:?}", board);
    //     println!("History: {:?}", a.unwrap()
    //     );
    //
    //     std::io::stdin().read_line(&mut String::new()).unwrap();
    //     board.unmake_move(a.unwrap());
    // }

    movgen.generate_moves(Color::White, &board);
    println!("mv length: {}", movgen.move_list.len());
    use kelp::mov_gen::perft::*;
    let mut pft = Perft::default();
    let time = std::time::Instant::now();
    println!("Perft 5");
    perft(5 , &mut board, &mut movgen, &mut pft);
    println!("Time: {:?}", time.elapsed());
    println!("Nodes: {}", pft.nodes);
    println!("Captures: {}", pft.captures);
    println!("En Passant: {}", pft.en_passants);
    println!("Castles: {}", pft.castles);
    println!("Promotions: {}", pft.promotions);
    println!("Checks: {}", pft.checks);


    // print!("{}", board.get_piece_at(Squares::E6).unwrap());
}
