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

use crate::kelp::board::piece::BoardPiece::{
    BlackPawn, BlackQueen, WhiteBishop, WhiteKing, WhiteKnight, WhitePawn, WhiteQueen, WhiteRook,
};
use crate::kelp::kelp::Kelp;
use crate::kelp::mov_gen::perft::*;
use kelp::board::{board::Board, fen::Fen};
use crate::kelp::UCI;

fn ptest(gen: &mut MovGen) {
    use crate::kelp::mov_gen::perft::*;
    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let args = std::env::args().collect::<Vec<String>>();
    let depth = args
        .get(1)
        .unwrap_or(&"4".to_string())
        .parse::<u16>()
        .unwrap();
    let mut fen = args.get(2).unwrap_or(&starring_fen.to_string()).to_string();

    let mut board = Board::parse(Fen(fen)).unwrap();
    let mut nodes = 0;
    perft_test(depth, &mut board, gen, &mut nodes)
}
fn main() {
    env_logger::init();
    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";
    let fein = "r3k2r/p1ppqPb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBpPP/R3K2R w Kkq - 0 1";
    let fein = "r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ";
    let tricky = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
    let empty = "8/8/8/8/8/8/8/8 w KQ - 0 1";

    let d_fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

    let mut table = LookupTable::new();
    // table.populate();

    let mut kelp = Kelp::new(&mut table);
    let a = kelp.parse_move("a2a4");
    println!("{:?}", a);
    kelp.uci_loop();

    // let mut board = Board::parse(Fen(starring_fen.to_string()));
    // if board.is_err() {
    //     println!("{:?}", board.err().unwrap());
    //     return;
    // }
    // let mut board = board.unwrap();
    // board.replace_piece(BoardPiece::BlackRook, E2);
    // println!("{}", board.get_piece(G6).unwrap());
    // println!("{}", board);

    // let mut movgen = MovGen::new(&mut table);
    // println!("{}", A5 as u8);

    // movgen.print_attacked(Color::White, &board);
    // println!("{}\n", board.get_piece_occ(BlackQueen));
    // board.add_piece(BoardPiece::BlackRook, C1);
    // println!("{:?}", board);
    // println!("{}",  board.is_king_checked(Color::White, & movgen));
    // // let time = std::time::Instant::now();
    // movgen.generate_moves(Color::Black, &board);
    // movgen.generate_moves( &board);
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

    // ptest(&mut movgen);
    // let args = std::env::args().collect::<Vec<String>>();
    // let depth = args.get(1).unwrap_or(&"4".to_string()).parse::<u16>().unwrap();
    // let mut fen = args.get(2).unwrap_or(&starring_fen.to_string()).to_string();
    // //
    // let mut board = Board::parse(Fen(d_fen.to_string())).unwrap();
    // // let mut board = Board::parse(Fen(empty.to_string())).unwrap();
    // // board.add_piece(WhitePawn, H4);
    // // board.add_piece(BlackPawn, A4);
    // // board.set_en_passant(H3);
    // // board.info.set_turn(Color::Black);
    //
    // // board.add_piece(WhiteKnight, H4 - 8);
    // println!("{:?}", board);
    // movgen.generate_moves(&board);
    //
    // for i in movgen.move_list.iter() {
    //     print!("{}  ", i);
    //     println!("{:?}", i);
    // }
    // println!("{:?}", board.get_side_to_move());
    // println!("{}", board.is_king_checked(Color::White, &movgen));
    //
    // for i in movgen.move_list.iter() {
    //     let a = board.make_move(*i);
    //     println!("{}", board.is_king_checked(board.get_side_to_move(), &movgen));
    //     board.unmake_move(a.unwrap());
    //
    // }

    // let mut empty_board = Board::parse(Fen(empty.to_string())).unwrap();
    // board.add_piece(WhiteKnight, E1);
    // board.add_piece(WhiteRook, A1);
    // board.add_piece(WhiteRook, H1);
    // // board.add_piece(BlackPawn, G5);
    // // board.set_en_passant(B6);
    // board.info.set_turn(Color::White);
    // movgen.generate_castling_moves(Color::White, &board);
    // println!("{:?}", board);
    // let mut new_bb = BitBoard::empty();
    // for i in movgen.move_list.iter() {
    //     // print!("{}  ", i);
    //     // println!("{:?}", i);
    // //        println!("{:?}  ", i);
    //     println!("board after making move: {}", i);
    //     let a = board.make_move(*i);
    //     if a.is_none() {
    //         println!("Invalid move: {}", i);
    //         continue;
    //     }
    //     println!("{:?}", board);
    //     println!("board after unmaking move: {}", i);
    //     board.unmake_move(a.unwrap());
    //     println!("{:?}", board);
    //     std::io::stdin().read_line(&mut String::new()).unwrap();
    // //         new_bb.set_bit(i.to as u8);
    // }
    //
    // println!("{}", new_bb);
    // let mut nodes = 0;
    // perft_test(depth, &mut board, &mut movgen, &mut nodes);

    // print!("{}", board.get_piece_at(Squares::E6).unwrap());
}
