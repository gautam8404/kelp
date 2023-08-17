use std::time::Duration;

use crate::kelp::board::board::Board;
use crate::kelp::board::moves::Move;
use crate::kelp::mov_gen::generator::MovGen;
use crate::kelp::search::eval::{eval, get_mvv_lva};

pub struct Negamax {
    pub nodes: u64,
    killer_moves: [[Option<Move>; Self::MAX_DEPTH]; 2],
    history_moves: [[i32; 64]; 12],
    pv_length: [usize; Self::MAX_DEPTH],
    pv_table: [[Option<Move>; Self::MAX_DEPTH]; Self::MAX_DEPTH],
}

impl Default for Negamax {
    fn default() -> Self {
        log::info!("Negamax::default() Initialized");
        Negamax {
            nodes: 0,
            killer_moves: [[None; Self::MAX_DEPTH]; 2],
            history_moves: [[0; 64]; 12],
            pv_length: [0; Self::MAX_DEPTH],
            pv_table: [[None; Self::MAX_DEPTH]; Self::MAX_DEPTH],
        }
    }
}

impl Negamax {
    pub const MIN: i32 = -50000;
    pub const MAX: i32 = 50000;
    pub const MAX_DEPTH: usize = 64;
    pub const MATE_SCORE: i32 = -49000;

    #[inline(always)]
    fn score_move(&mut self, mov: &Move, ply: usize) -> i32 {
        if mov.capture.is_some() {
            return get_mvv_lva(mov) + 10000;
        } else if self.killer_moves[0][ply] == Some(*mov) {
            return 9000;
        } else if self.killer_moves[1][ply] == Some(*mov) {
            return 8000;
        } else {
            return self.history_moves[mov.piece as usize][mov.to as usize];
        }
    }
    #[inline(always)] //TODO: refractor and improve
    pub fn negamax(
        &mut self,
        mut alpha: i32,
        mut beta: i32,
        depth: usize,
        board: &mut Board,
        gen: &mut MovGen,
        mut ply: usize,
    ) -> i32 {
        self.pv_length[ply] = ply;

        if depth == 0 {
            return self.quiescence(alpha, beta, board, gen, ply);
        }

        if ply >= Self::MAX_DEPTH - 1 {
            return eval(board);
        }

        self.nodes += 1;
        let in_check = board.is_check(gen);

        gen.generate_moves(board);
        let mut moves_list = gen.move_list.clone();
        let mut legal_moves = 0;

        let mut score = Self::MIN;
        moves_list.0.sort_by(|a, b| {
            self.score_move(b, ply)
                .cmp(&self.score_move(a, ply))
        });

        for moves in moves_list.iter() {
            ply += 1;

            let a = board.make_move(*moves, false);
            if a.is_none() {
                ply -= 1;
                continue;
            }

            if board.is_check_opp(gen) {
                board.unmake_move(a.unwrap());
                ply -= 1;
                continue;
            }

            legal_moves += 1;
            score = i32::max(score, -self.negamax(-beta, -alpha, depth - 1, board, gen, ply));

            ply -= 1;
            board.unmake_move(a.unwrap());

            if score >= beta {
                if moves.capture.is_none() {
                    self.killer_moves[1][ply] = self.killer_moves[0][ply];
                    self.killer_moves[0][ply] = Some(*moves);
                }
                return beta;
            }

            if score > alpha {
                if moves.capture.is_none() {
                    self.history_moves[moves.piece as usize][moves.to as usize] += depth as i32;
                }

                alpha = score;

                self.pv_table[ply][ply] = Some(*moves);

                for i in (ply + 1)..self.pv_length[ply as usize] {
                    self.pv_table[ply][i] = self.pv_table[ply + 1][i];
                }

                self.pv_length[ply] = self.pv_length[ply + 1];
            }
        }

        if legal_moves == 0 {
            return if in_check {
                Self::MATE_SCORE + ply as i32
            } else {
                0
            }
        }

        alpha
    }

    #[inline(always)]
    fn quiescence(
        &mut self,
        mut alpha: i32,
        beta: i32,
        board: &mut Board,
        gen: &mut MovGen,
        mut ply: usize,
    ) -> i32 {
        self.nodes += 1;
        let eval = eval(board);

        if eval >= beta {
            return beta;
        }

        alpha = alpha.max(eval);

        gen.generate_moves(board);
        let mut moves_list = gen.move_list.clone();

        moves_list.0.sort_by(|a, b| {
            self.score_move(b, ply as usize)
                .cmp(&self.score_move(a, ply as usize))
        });

        for m in moves_list.iter() {
            ply += 1;

            let a = board.make_move(*m, true);
            if a.is_none() {
                ply -= 1;
                continue;
            }
            if board.is_check_opp(gen) {
                board.unmake_move(a.unwrap());
                ply -= 1;
                continue;
            }

            let score = -self.quiescence(-beta, -alpha, board, gen, ply);
            ply -= 1;

            board.unmake_move(a.unwrap());

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }
        alpha
    }

    pub fn get_nodes(&self) -> u64 {
        self.nodes
    }

    pub fn reset(&mut self) {
        self.nodes = 0;
        self.killer_moves = [[None; Self::MAX_DEPTH]; 2];
        self.history_moves = [[0; 64]; 12];
        self.pv_length = [0; Self::MAX_DEPTH];
        self.pv_table = [[None; Self::MAX_DEPTH]; Self::MAX_DEPTH];
    }

    pub fn get_pv_str(&self) -> String {
        let mut pv = String::new();
        for i in 0..self.pv_length[0] {
            let pv_str = {
                if self.pv_table[0][i].is_none() {
                    break;
                }
                self.pv_table[0][i].unwrap().to_string()
            };
            pv.push_str(&pv_str);
            pv.push(' ');
        }
        pv
    }

    pub fn get_pv_table(&self, x: usize, y: usize) -> Option<Move> {
        self.pv_table[x][y]
    }

}