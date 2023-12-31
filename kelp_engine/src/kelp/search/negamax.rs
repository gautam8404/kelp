use super::draw_table::DrawTable;
use super::transposition::{Entry, EntryType, TranspositionTable};
use crate::kelp::board::board::Board;
use crate::kelp::board::moves::Move;
use crate::kelp::mov_gen::generator::MovGen;
use crate::kelp::search::eval::{get_mvv_lva, Eval};
use crate::kelp::STOP;
use std::sync::atomic::Ordering;

pub struct Negamax {
    pub nodes: u64,
    killer_moves: [[Option<Move>; Self::MAX_DEPTH]; 2],
    history_moves: [[i32; 64]; 12],
    pv_length: [usize; Self::MAX_DEPTH],
    pv_table: [[Option<Move>; Self::MAX_DEPTH]; Self::MAX_DEPTH],
    draw_table: DrawTable,
    eval: Eval,
    pub follow_pv: bool,
    pub score_pv: bool,
    pub tt: TranspositionTable,
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
            draw_table: DrawTable::new(),
            eval: Eval::default(),
            follow_pv: false,
            score_pv: false,
            tt: TranspositionTable::new(),
        }
    }
}

impl Negamax {
    pub const MIN: i32 = -50000;
    pub const MAX: i32 = 50000;
    pub const MAX_DEPTH: usize = 128;
    pub const MATE_VALUE: i32 = 49000;
    pub const MATE_SCORE: i32 = 48000;

    const NULL_MOVE_REDUCTION: usize = 3;
    const FULL_DEPTH: usize = 4;
    const NULL_WINDOW: usize = 2;

    #[inline(always)]
    fn score_move(&mut self, mov: &Move, ply: usize) -> i32 {
        if self.score_pv && self.pv_table[0][ply] == Some(*mov) {
            self.score_pv = false;
            return 20000;
        }


        if mov.capture.is_some() {
            get_mvv_lva(mov) + 10000
        } else if self.killer_moves[0][ply] == Some(*mov) {
            return 9000;
        } else if self.killer_moves[1][ply] == Some(*mov) {
            return 8000;
        } else {
            return self.history_moves[mov.piece as usize][mov.to as usize];
        }
    }

    #[inline(always)]
    pub fn negamax(
        &mut self,
        mut alpha: i32,
        mut beta: i32,
        mut depth: usize,
        board: &mut Board,
        gen: &mut MovGen,
        ply: usize,
    ) -> i32 {
        self.pv_length[ply] = ply;
        let mut score = Self::MIN;

        let mut entry_def = Entry::default();
        entry_def.flag = EntryType::Alpha;

        if ply != 0 && self.draw_table.is_repeat(board.hash) {
            return 0;
        }

        if board.is_fifty_move_draw() {
            return 0;
        }

        let pv_node = beta - alpha > 1;

        if let Some(entry) = self.tt.get(board.hash) {
            if entry.depth >= depth as u8 && entry.hash == board.hash && ply != 0 && !pv_node {
                match entry.flag {
                    EntryType::Exact => {
                        self.pv_length[ply] = ply + 1;
                        self.pv_table[ply][ply] = entry.best_move;

                        if entry.score < -Self::MATE_SCORE {
                            return entry.score + ply as i32;
                        } else if entry.score > Self::MATE_SCORE {
                            return entry.score - ply as i32;
                        }
                        return entry.score;
                    }
                    EntryType::Alpha => {
                        if entry.score <= alpha {
                            return alpha;
                        }
                    }
                    EntryType::Beta => {
                        if entry.score >= beta {
                            return beta;
                        }
                    }
                }
            }
        }

        if depth == 0 {
            return self.quiescence(alpha, beta, board, gen, ply + 1);
        }

        if ply >= Self::MAX_DEPTH - 1 {
            return self.eval.evaluate(board, gen);
        }

        self.nodes += 1;
        let in_check = board.is_check(gen);

        if in_check {
            depth += 1;
        }

        //Null Move Pruning
        if depth >= 3 && in_check == false && ply != 0 {
            self.draw_table.push(board.hash);
            let (enpassant, old_hash) = board.make_null_move();

            score = -self.negamax(
                -beta,
                -beta + 1,
                depth - 1 - Self::NULL_WINDOW,
                board,
                gen,
                ply + 1,
            );

            self.draw_table.pop();
            board.unmake_null_move(enpassant, old_hash);

            if STOP.load(Ordering::Relaxed) {
                return 0;
            }

            if score >= beta {
                let ent = Entry {
                    hash: board.hash,
                    depth: depth as u8,
                    flag: EntryType::Beta,
                    score: beta,
                    best_move: None,
                };
                self.tt.insert(board.hash, ent);
                return beta;
            }
        }

        gen.generate_moves(board);
        let mut moves_list = gen.move_list.clone();
        let mut legal_moves = 0;

        if self.follow_pv {
            if moves_list.iter().any(|x| self.pv_table[0][ply] == Some(*x)) {
                self.score_pv = true;
                self.follow_pv = true;
            } else {
                self.follow_pv = false;
            }
        }

        moves_list
            .0
            .sort_by(|a, b| self.score_move(b, ply).cmp(&self.score_move(a, ply)));

        let mut moves_searched = 0;

        for moves in moves_list.iter() {
            self.draw_table.push(board.hash);

            let a = board.make_move(*moves, false);
            if a.is_none() {
                self.draw_table.pop();
                continue;
            }

            if board.is_check_opp(gen) {
                board.unmake_move(a.unwrap());
                self.draw_table.pop();
                continue;
            }

            legal_moves += 1;
            // self.draw_table.push(board.hash);

            if moves_searched == 0 {
                score = -self.negamax(-beta, -alpha, depth - 1, board, gen, ply + 1);
            } else {
                //Late Move Reduction
                if moves_searched >= Self::FULL_DEPTH
                    && depth >= Self::NULL_MOVE_REDUCTION
                    && in_check == false
                    && moves.capture.is_none()
                    && moves.is_promotion() == false
                {
                    score = -self.negamax(-alpha - 1, -alpha, depth - 2, board, gen, ply + 1);
                } else {
                    score = alpha + 1;
                }

                // PVS
                if score > alpha {
                    score = -self.negamax(-alpha - 1, -alpha, depth - 1, board, gen, ply + 1);

                    if (score > alpha) && (score < beta) {
                        score = -self.negamax(-beta, -alpha, depth - 1, board, gen, ply + 1);
                    }
                }
            }

            self.draw_table.pop();
            board.unmake_move(a.unwrap());

            if STOP.load(Ordering::Relaxed) {
                return 0;
            }

            moves_searched += 1;

            if score > alpha {
                if moves.capture.is_none() {
                    self.history_moves[moves.piece as usize][moves.to as usize] += depth as i32;
                }

                alpha = score;

                self.pv_table[ply][ply] = Some(*moves);

                for i in (ply + 1)..self.pv_length[ply + 1] {
                    self.pv_table[ply][i] = self.pv_table[ply + 1][i];
                }

                self.pv_length[ply] = self.pv_length[ply + 1];

                entry_def.best_move = Some(*moves);
                entry_def.flag = EntryType::Exact;

                if score >= beta {
                    if moves.capture.is_none() {
                        self.killer_moves[1][ply] = self.killer_moves[0][ply];
                        self.killer_moves[0][ply] = Some(*moves);
                    }

                    let ent = Entry {
                        hash: board.hash,
                        depth: depth as u8,
                        flag: EntryType::Beta,
                        score: beta,
                        best_move: None,
                    };
                    self.tt.insert(board.hash, ent);
                    return beta;
                }
            }
        }

        if legal_moves == 0 {
            return if in_check {
                -Self::MATE_VALUE + ply as i32
            } else {
                0
            };
        }

        let sc = {
            if entry_def.flag == EntryType::Exact {
                if alpha > Self::MATE_SCORE {
                    alpha + ply as i32
                } else if alpha < -Self::MATE_SCORE {
                    alpha - ply as i32
                } else {
                    alpha
                }
            } else {
                alpha
            }
        };

        let entry = Entry {
            hash: board.hash,
            depth: depth as u8,
            flag: entry_def.flag,
            score: sc,
            best_move: entry_def.best_move,
        };

        self.tt.insert(board.hash, entry);

        alpha
    }

    #[inline(always)]
    fn quiescence(
        &mut self,
        mut alpha: i32,
        beta: i32,
        board: &mut Board,
        gen: &mut MovGen,
        ply: usize,
    ) -> i32 {
        self.nodes += 1;

        if ply > Self::MAX_DEPTH - 1 {
            return self.eval.evaluate(board, gen);
        }

        let eval = self.eval.evaluate(board, gen);

        if eval >= beta {
            return beta;
        }

        if eval > alpha {
            alpha = eval;
        }


        gen.generate_moves(board);
        let mut moves_list = gen.move_list.clone();

        moves_list.0.sort_by(|a, b| {
            self.score_move(b, ply as usize)
                .cmp(&self.score_move(a, ply as usize))
        });

        for m in moves_list.iter() {
            self.draw_table.push(board.hash);
            let a = board.make_move(*m, true);
            if a.is_none() {
                self.draw_table.pop();
                continue;
            }
            if board.is_check_opp(gen) {
                board.unmake_move(a.unwrap());
                self.draw_table.pop();
                continue;
            }

            let score = -self.quiescence(-beta, -alpha, board, gen, ply + 1);

            self.draw_table.pop();
            board.unmake_move(a.unwrap());


            if score > alpha {
                alpha = score;

                if score >= beta {
                    return beta;
                }
            }
        }
        alpha
    }

    pub fn add_draw(&mut self, key: u64) {
        self.draw_table.push(key);
    }

    pub fn clear_draw(&mut self) {
        self.draw_table.clear();
    }

    pub fn clear_tt(&mut self) {
        self.tt.clear();
    }

    pub fn reset(&mut self) {
        // for iterative deepening
        self.nodes = 0;
        self.killer_moves = [[None; Self::MAX_DEPTH]; 2];
        self.history_moves = [[0; 64]; 12];
        self.pv_length = [0; Self::MAX_DEPTH];
        self.pv_table = [[None; Self::MAX_DEPTH]; Self::MAX_DEPTH];
        self.follow_pv = false;
        self.score_pv = false;
    }

    pub fn reset_tables(&mut self) {
        //for position command and new game
        self.tt.clear();
        self.draw_table.clear();
    }

    pub fn get_pv_str(&self) -> String {
        let mut pv = String::new();
        for i in 0..self.pv_length[0] {
            let pv_str = {
                if self.pv_table[0][i].is_none() {
                    continue;
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

    #[allow(dead_code)]
    pub fn get_pv_length(&self, x: usize) -> usize {
        self.pv_length[x]
    }

    #[allow(dead_code)]
    pub fn print_move_scores(&mut self, gen: &mut MovGen, ply: usize) {
        let moves_list = gen.move_list.clone();

        for moves in moves_list.iter() {
            println!("{}: {}", moves, self.score_move(moves, ply));
        }
    }
}
