use super::board::board::Board;
use super::board::moves::Move;
use super::kelp_core::lookup_table::LookupTable;
use super::mov_gen::generator::MovGen;
use super::uci_trait::UCI;
use super::TimeControl;
use super::{stop_interval, STOP};
use crate::kelp::board::fen::{Fen, FenParse};
use crate::kelp::board::piece::BoardPiece::{*};
use crate::kelp::search::negamax::Negamax;
use log;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread;

pub struct SearchMoveResult {
    pub best_move: Option<Move>,
    pub score: i32,
    pub depth: usize,
    pub nodes: u64,
    pub time: std::time::Duration,
}

/// Main Implementation for all UCI commands also acts as a library for the engine
/// Kelp contains the board and the mov_gen from kelp::board and kelp::mov_gen respectively
pub struct Kelp<'a> {
    pub board: Board,
    pub mov_gen: MovGen<'a>,
    pub search: Negamax,
}

impl<'a> Kelp<'a> {
    const ASPIRATION_WINDOW: i32 = 50;

    ///Creates a new instance of Kelp, populates the lookup table in case if its not populated beforehand
    pub(crate) fn new(table: &'a mut LookupTable) -> Self {
        table.populate();
        Kelp {
            board: Board::default(),
            mov_gen: MovGen::new(table),
            search: Negamax::default(),
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

    // for uci only
    #[inline(always)]
    fn search_move(&mut self, depth: usize) -> Option<Move> {
        self.search.reset();
        let mut score = 0;

        let mut alpha = Negamax::MIN;
        let mut beta = Negamax::MAX;

        //Iterative Deepening
        for i in 1..=depth {
            self.search.nodes = 0;
            self.search.follow_pv = true;
            let now = std::time::Instant::now();

            score = self
                .search
                .negamax(alpha, beta, i, &mut self.board, &mut self.mov_gen, 0);

            if STOP.load(Ordering::Relaxed) {
                STOP.store(false, Ordering::Relaxed);
                break;
            }

            if (score <= alpha) || (score >= beta) {
                alpha = Negamax::MIN;
                beta = Negamax::MAX;
                continue;
            }

            alpha = score - Self::ASPIRATION_WINDOW;
            beta = score + Self::ASPIRATION_WINDOW;

            let elapsed = now.elapsed();

            self.send_info(
                format!(
                    "depth {} score cp {} nodes {} time {} nps {} hits {} misses {} hashfull {:.2} size {:.2} entries {} pv {}",
                    i,
                    score,
                    self.search.nodes,
                    elapsed.as_millis(),
                    (self.search.nodes as f64 / elapsed.as_secs_f64()) as u64,
                    self.search.tt.get_hits(),
                    self.search.tt.get_misses(),
                    self.search.tt.get_hash_full_percentage() as u8,
                    self.search.tt.get_hashmap_size_mb(),
                    self.search.tt.get_total_entries(),
                    self.search.get_pv_str(),
                )
                .as_str(),
            );
            self.search.tt.reset_hits_and_misses();
        }

        STOP.store(false, Ordering::Relaxed);
        self.search.get_pv_table(0, 0)
    }

    ///search move for library functions
    #[inline(always)]
    pub fn search_move_lib(&mut self, depth: usize) -> SearchMoveResult {
        self.search.reset();
        let mut score = 0;

        let mut alpha = Negamax::MIN;
        let mut beta = Negamax::MAX;

        for i in 1..=depth {
            self.search.nodes = 0;
            // let now = std::time::Instant::now();
            score = self
                .search
                .negamax(alpha, beta, i, &mut self.board, &mut self.mov_gen, 0);

            if (score <= alpha) || (score >= beta) {
                alpha = Negamax::MIN;
                beta = Negamax::MAX;
                continue;
            }

            alpha = score - Self::ASPIRATION_WINDOW;
            beta = score + Self::ASPIRATION_WINDOW;
        }

        SearchMoveResult {
            best_move: self.search.get_pv_table(0, 0),
            score,
            depth,
            nodes: self.search.nodes,
            time: std::time::Instant::now().elapsed(),
        }
    }

    // custom uci handler
    fn uci_handle(&mut self, rx: &mpsc::Receiver<String>) {
        loop {
            let mut input = rx.try_recv();
            if input.is_err() {
                continue;
            }
            let input = input.unwrap();

            if !input.is_empty() {
                self.receive(input.trim());
            }
        }
    }
}

impl UCI for Kelp<'_> {
    fn handle_position(&mut self, arg: &[&str]) {
        STOP.store(false, Ordering::Relaxed);
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

        self.search.tt.clear(); // removing this for concurrency
    }

    fn handle_uci_newgame(&mut self) {
        self.board = Board::default();
        self.search.tt.clear();
    }

    fn handle_go(&mut self, arg: &[&str]) {
        use std::time::Duration;
        let mut depth = 0;
        STOP.store(false, Ordering::Relaxed); // a preventive measure

        if arg.len() < 1 {
            return;
        }

        let mut time_control = TimeControl::default();

        for i in 0..arg.len() {
            if arg[i] == "wtime" {
                time_control.wtime = Some(arg[i + 1].parse::<i128>().unwrap().abs());
            }

            if arg[i] == "btime" {
                time_control.btime = Some(arg[i + 1].parse::<i128>().unwrap().abs());
            }

            if arg[i] == "winc" {
                time_control.winc = arg[i + 1].parse::<i128>().unwrap().abs();
            }

            if arg[i] == "binc" {
                time_control.binc = arg[i + 1].parse::<i128>().unwrap().abs();
            }

            if arg[i] == "movestogo" {
                time_control.movestogo = Some(arg[i + 1].parse::<u32>().unwrap());
            }

            if arg[i] == "movetime" {
                time_control.movetime = Some(arg[i + 1].parse::<i128>().unwrap().abs());
            }

            if arg[i] == "infinite" {
                time_control.infinite = true;
            }

            if arg[i] == "depth" {
                let dep = arg[i + 1].parse::<usize>();
                if dep.is_ok() {
                    depth = dep.unwrap();
                } else {
                    depth = Negamax::MAX_DEPTH;
                }
            }
        }

        let time_to_search = time_control.calculate_time(self.board.get_side_to_move());

        if time_to_search.is_none() && depth == 0 && !time_control.infinite {
            return;
        }
        if time_to_search.is_some() || time_control.infinite {
            depth = Negamax::MAX_DEPTH;
        }


        if time_to_search.is_some()  && !time_control.infinite && time_to_search.unwrap() >= 0 {
            let time_to_search = time_to_search.unwrap_or(0);
            let duration = Duration::from_millis(time_to_search as u64);
            stop_interval(duration);
        }

        let best_move = self.search_move(depth);
        STOP.store(false, Ordering::Relaxed);

        if best_move.is_none() {
            STOP.store(false, Ordering::Relaxed);
        } else {
            self.send_bestmove(format!("{}", best_move.unwrap()).as_str());
        }
    }

    fn handle_uci(&self, arg: &[&str]) {
        self.send("id name Kelp Engine");
        self.send("id author Gautam Dhingra");
        self.send("uciok");
    }

    fn handle_quit(&self) {
        std::process::exit(0);
    }

    fn handle_stop(&self) {
        STOP.store(true, Ordering::Relaxed);
    }

    fn handle_ready(&self) {
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

    // Multi threaded UCI loop, takes input in a parallel thread
    fn uci_loop(&mut self) {
        let (tx, rx) = mpsc::channel();

        // Start the input thread
        let io_thread = thread::spawn(move || {
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "stop" {
                    STOP.store(true, Ordering::Relaxed);
                    continue;
                }
                if input.trim() == "quit" {
                    STOP.store(true, Ordering::Relaxed); //stop engine then pass quit to uci loop
                }
                tx.send(input).unwrap();
            }
        });

        // Start the UCI loop in the main thread
        self.uci_handle(&rx);

        io_thread.join().unwrap();
    }

    fn log_stdio(&self, arg: &str) {
        log::info!("{}", arg);
    }
}
