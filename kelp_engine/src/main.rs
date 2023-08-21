mod kelp;
extern crate simplelog;
#[macro_use]
extern crate log;

use crate::kelp::board::fen::FenParse;
use crate::kelp::board::piece::{BoardPiece, Color};
use crate::kelp::mov_gen::generator::MovGen;
use kelp::kelp_core::bitboard::BitBoard;
use kelp::kelp_core::lookup_table::LookupTable;
use kelp::Squares::{self, *};
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;

use crate::kelp::board::piece::BoardPiece::{
    BlackPawn, BlackQueen, WhiteBishop, WhiteKing, WhiteKnight, WhitePawn, WhiteQueen, WhiteRook,
};
use crate::kelp::kelp::Kelp;
use crate::kelp::mov_gen::perft::*;
use crate::kelp::uci_trait::UCI;
use kelp::board::{board::Board, fen::Fen};
use simplelog::{Config, LevelFilter, WriteLogger};

use crate::kelp::board::moves::Move;
use crate::kelp::search::eval::eval;
use crate::kelp::search::negamax::Negamax;
use kelp::STOP;

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
    let file_path = std::env::var("KELP_LOG").unwrap_or("kelp.log".to_string());
    let _ = WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        std::fs::File::create(file_path).unwrap(),
    );

    let starring_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fein = "8/8/4R3/3B4/8/8/8/8 w - - 0 1";
    let fein = "r3k2r/p1ppqPb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBpPP/R3K2R w Kkq - 0 1";
    let fein = "r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ";
    let tricky = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
    let empty = "8/8/8/8/8/8/8/8 w KQ - 0 1";

    let d_fen = "r3k2r/p1Ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";

    // table.populate();

    let mut search = Negamax::default();
    // let a = kelp.parse_move("a2a4");
    // println!("{:?}", a);

    let debug = true;
    if debug {
        let mut table = LookupTable::new();
        let mut kelp = Kelp::new(&mut table);
        kelp.handle_position(&["fen", tricky]);


        let mut nodes = 0;
        perft_test(4, &mut kelp.board, &mut kelp.mov_gen, &mut nodes);
        println!("Nodes: {}", nodes);

        // kelp.handle_position(&["fen", d_fen]); //38CC6D0BACBD4BD9
        // println!("{:?}", kelp.board);
        // let mv = kelp.parse_move("e1g1").unwrap();
        // kelp.make_move(mv);
        // println!("{:?}", kelp.board);
        // let gen_hash = kelp.board.zobrist.get_key(&kelp.board);
        //
        // if kelp.board.hash != gen_hash {
        //     println!("Hashes don't match: {:016X} {:016X}", kelp.board.hash, gen_hash);
        // } else {
        //     println!("\n\n\n\n\n\n\
        //     ========================================================\n\
        //     HURRAY! Hashes match: {:016X} {:016X}\n\
        //     ========================================================\n\n\n\n\n\n", kelp.board.hash, gen_hash);

        // kelp.board.info.toggle_turn();
        // let mv2 = kelp.parse_move("g1h1").unwrap();
        // kelp.make_move(mv2);

        // println!("{:?}", kelp.board);
        // kelp.unmake_move();
        // println!("{:?}", kelp.board);
        // kelp.unmake_move();
        // println!("{:?}", kelp.board);
        // use strum::IntoEnumIterator;
        // for sq in Squares::iter() {
        //     print!("{} ", sq);
        //     println!("{}", sq as u8);
        // }
        // kelp.handle_position(&["fen", starring_fen]);
        // let mut best_move: Option<Move> = None;
        // let score = search.negamax(-10000, 10000, 2, &mut kelp.board, &mut kelp.mov_gen, &mut best_move);
        // println!("best move: {:?} score: {}", best_move, score);
        // kelp.handle_position(&["fen", tricky]);
        // kelp.handle_go(&["depth", "5"]);
        // kelp.mov_gen.generate_moves(&kelp.board);
        // let list = kelp.mov_gen.move_list.clone();
        // for i in list.iter() {
        //     let a = kelp.board.make_move(*i, true);
        //     if a.is_some() {
        //         print!("{}  ", i);
        //         println!("{:?}", i);
        //         kelp.board.unmake_move(a.unwrap());
        //     }
        // }
    } else {
        let mut table = LookupTable::new();
        let mut kelp = Kelp::new(&mut table);
        kelp.uci_loop();
        // let (tx, rx) = mpsc::channel();
        // let shared_input = Arc::new(Mutex::new(String::new()));
        //
        // // Start the input thread
        // let io_thread = thread::spawn(move || {
        //     loop {
        //         let mut input = String::new();
        //         std::io::stdin().read_line(&mut input).unwrap();
        //
        //         if input.trim() == "stop" {
        //             STOP.store(true, Ordering::Relaxed);
        //             println!("stop\n\n\n\n\n\n\n\n\nn\n\n");
        //             continue;
        //         }
        //         if input.trim() == "quit" {
        //             STOP.store(true, Ordering::Relaxed); //stop engine then pass quit to uci loop
        //         }
        //         tx.send(input).unwrap();
        //     }
        // });
        //
        // // Start the UCI loop in the main thread
        // kelp.uci_loop(&rx, &shared_input);
        //
        // io_thread.join().unwrap();
    }

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
