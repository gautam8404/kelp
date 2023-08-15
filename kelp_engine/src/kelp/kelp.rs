use super::board::board::Board;
use super::board::moves::Move;
use super::kelp_core::lookup_table::LookupTable;
use super::mov_gen::generator::MovGen;
use super::uci_trait::UCI;
use crate::kelp::board::fen::{Fen, FenParse};
use crate::kelp::board::piece::BoardPiece::{self, *};
use log;

use rand::seq::SliceRandom; // just for a random game

/// Main Implementation for all UCI commands also acts as a library for the engine
/// Kelp contains the board and the mov_gen from kelp::board and kelp::mov_gen respectively
pub struct Kelp<'a> {
    pub board: Board,
    pub mov_gen: MovGen<'a>,
}

impl<'a> Kelp<'a> {
    ///Creates a new instance of Kelp, populates the lookup table in case if its not populated beforehand
    pub(crate) fn new(table: &'a mut LookupTable) -> Self {
        table.populate();
        Kelp {
            board: Board::default(),
            mov_gen: MovGen::new(table),
        }
    }

    ///compares mov_str by move.to_string, if it exist it checks if the move is legal by making/unmaking the move
    pub fn parse_move(&mut self, mov_str: &str) -> Option<Move> {
        if mov_str.len() < 4 || mov_str.len() > 5 {
            return None;
        }
        self.mov_gen.generate_moves(&self.board);

        for mov in self.mov_gen.move_list.iter() {
            if mov_str == mov.to_string() {
                return Some(*mov);
            }
        }
        None
    }

    /// makes move on board, unmakes it if it is illegal
    pub fn make_move(&mut self, mov: Move) -> bool {
        self.board.make(mov);
        if self.board.is_check_opp(&self.mov_gen) {
            self.board.unmake();
            return false;
        }
        true
    }

    /// unmakes move from move history
    pub fn unmake_move(&mut self) {
        self.board.unmake();
    }

    pub fn get_fen(&self) -> String {
        self.board.to_fen()
    }
}

impl UCI for Kelp<'_> {
    fn handle_position(&mut self, arg: &[&str]) {
        if arg.len() < 1 {
            return;
        }

        if !self.is_position(arg[0]) {
            return;
        }

        let mut point = 0;
        if arg[0] == "startpos" {
            self.board = Board::default();
            point += 1;
        } else if arg[0] == "fen" {
            point += 1;
            let mut fen = String::new();
            while point < arg.len() && arg[point] != "moves" {
                fen.push_str(arg[point]);
                fen.push(' ');
                point += 1;
            }
            self.board = Board::parse(Fen(fen)).unwrap();
        }

        if point < arg.len() && arg[point] == "moves" {
            point += 1;
            while point < arg.len() {
                let mov = self.parse_move(arg[point]);
                if mov.is_none() {
                    break;
                }
                self.make_move(mov.unwrap());
                point += 1;
            }
        }
    }

    fn handle_go(&mut self, arg: &[&str]) {
        self.mov_gen.generate_moves(&self.board);

        // let mv = self.parse_move("a2a4");
        // if mv.is_none() {
        //     self.send_bestmove("a2a4");
        //     return;
        // }

        let mut random_move;

        loop {
            random_move = *self
                .mov_gen
                .move_list
                .0
                .choose(&mut rand::thread_rng())
                .unwrap();

            if self.make_move(random_move) {
                break;
            }
        }

        self.send_bestmove(random_move.to_string().as_str());
    }

    fn handle_uci(&self, arg: &[&str]) {
        self.send("id name Kelp Engine");
        self.send("id author Gautam Dhingra");
        self.send("uciok");
    }

    fn handle_quit(&self, arg: &[&str]) {
        std::process::exit(0);
    }

    fn handle_stop(&self, arg: &[&str]) {
        todo!()
    }

    fn handle_ready(&self, arg: &[&str]) {
        self.send("readyok"); //TODO: Implement this
    }

    fn handle_unknown(&self, command: &str, arg: &[&str]) {
        match command {
            "help" => {
                self.send("Kelp is a UCI compatible chess engine written in Rust");
                self.send("It is released as a free software under GNU GPL v3 License.");
                self.send("For more information visit https://github.com/gautam8404/kelp/#readme");
            }
            _ => {
                self.send(
                    format!(
                        "Unknown command: {}. Type help for more information",
                        command
                    )
                    .as_str(),
                );
            }
        }
    }

    fn print_board(&self) {
        self.send(format!("{}", self.board).as_str());
    }

    fn log_stdio(&self, arg: &str) {
        log::info!("{}", arg);
    }
}
