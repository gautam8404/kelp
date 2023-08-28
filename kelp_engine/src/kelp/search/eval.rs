use std::cmp::min;
use super::{
    BISHOP_SCORES, ENDGAME_PHASE_SCORE, KING_SCORES, KNIGHT_SCORES, MATERIAL_SCORES, MVV_LVA,
    OPENING_PHASE_SCORE, PAWN_SCORES, QUEEN_SCORES, ROOK_SCORES, CENTER_MANHATTAN_DISTANCE
};

use crate::kelp::GamePhase::{self, *};

use crate::kelp::board::board::Board;
use crate::kelp::board::moves::Move;
use crate::kelp::board::piece::BoardPiece::{self, *};
use crate::kelp::board::piece::Color;
use crate::kelp::board::piece::Color::{Black, White};
use crate::kelp::kelp_core::bitboard::BitBoard;
use crate::kelp::mov_gen::generator::MovGen;
use crate::kelp::{Squares, MIRROR};
use strum::IntoEnumIterator;

const DOUBLE_PAWN_PENALTY_OPENING: i32 = -5;
const DOUBLE_PAWN_PENALTY_ENDGAME: i32 = -10;
const ISOLATED_PAWN_PENALTY_OPENING: i32 = -5;
const ISOLATED_PAWN_PENALTY_ENDGAME: i32 = -10;
const SEMI_OPEN_FILE_SCORE: i32 = 10;
const OPEN_FILE_SCORE: i32 = 15;
const ROOK_ON_SEVENTH_OPENING_SCORE: i32 = 20;
const ROOK_ON_SEVENTH_ENDGAME_SCORE: i32 = 40;
const QUEEN_ON_SEVENTH_OPENING_SCORE: i32 = 10;
const QUEEN_ON_SEVENTH_ENDGAME_SCORE: i32 = 20;
const KING_SHIELD_BONUS: i32 = 5;

const PASSED_PAWN_BONUS: [i32; 8] = [ 0, 10, 30, 50, 75, 100, 150, 200 ];
const KING_PAWN_SHIELD_SCORES: [i32; 6] = [ 4, 7, 4, 3, 6, 3];

// mobility & units

const KNIGHT_UNITS: i32 = 3;
const BISHOP_UNITS: i32 = 4;
const ROOK_UNITS: i32 = 6;
const QUEEN_UNITS: i32 = 9;


const KNIGHT_MOB_OPENING: i32 = 4;
const KNIGHT_MOB_ENDGAME: i32 = 4;
const BISHOP_MOB_OPENING: i32 = 5;
const BISHOP_MOB_ENDGAME: i32 = 5;
const ROOK_MOB_OPENING: i32 = 2;
const ROOK_MOB_ENDGAME: i32 = 4;
const QUEEN_MOB_OPENING: i32 = 1;
const QUEEN_MOB_ENDGAME: i32 = 2;

pub struct Eval {
    pub file_mask: [BitBoard; 8],
    pub rank_mask: [BitBoard; 8],
    pub isolated_mask: [BitBoard; 8],
    pub passed_mask: [[BitBoard; 64]; 2],
    pub orthogonal_distance: [[i32; 64]; 64],
}

impl Eval {
    fn init_file_mask(&mut self) {
        for r in 0..8 {
            for f in 0..8 {
                let square = r * 8 + f;
                self.file_mask[f].set_bit(square as u8);
            }
        }
    }

    fn init_rank_mask(&mut self) {
        for r in 0..8 {
            for f in 0..8 {
                let square = r * 8 + f;
                self.rank_mask[r].set_bit(square as u8);
            }
        }
    }

    pub fn init_isolated_mask(&mut self) {
        for i in 0..8 {
            if i != 0 {
                self.isolated_mask[i] |= self.file_mask[i - 1];
            }
            if i != 7 {
                self.isolated_mask[i] |= self.file_mask[i + 1];
            }
        }
    }

    pub fn init_passed_mask(&mut self) {
        for r in 0..8 {
            for f in 0..8 {
                let mut bb = self.file_mask[f] | self.isolated_mask[f];

                let square = r * 8 + f;

                for i in 0..8 * (r + 1) {
                    bb.clear_bit(i as u8);
                }

                self.passed_mask[Color::White as usize][square] = bb;
            }
        }

        for r in 0..8 {
            for f in 0..8 {
                let mut bb = self.file_mask[f] | self.isolated_mask[f];

                let square = r * 8 + f;

                for i in 8 * r..64 {
                    bb.clear_bit(i as u8);
                }

                self.passed_mask[Color::Black as usize][square] = bb;
            }
        }
    }

    pub fn init_orhtogonal_distance(&mut self) {
        for square_a in 0..64 {
            let coord_a = Squares::from_repr(square_a as u8).unwrap();

            for square_b in 0..64 {
                let coord_b = Squares::from_repr(square_b as u8).unwrap();

                let rank_distance = (coord_a.rank() as i32 - coord_b.rank() as i32).abs();
                let file_distance = (coord_a.file() as i32 - coord_b.file() as i32).abs();

                self.orthogonal_distance[square_a][square_b] = file_distance + rank_distance;
            }
        }
    }

    #[inline(always)]
    pub fn get_file_mask(&self, sq: Squares) -> BitBoard {
        self.file_mask[sq.file() as usize]
    }

    #[inline(always)]
    pub fn get_rank_mask(&self, sq: Squares) -> BitBoard {
        self.rank_mask[sq.rank() as usize]
    }

    #[inline(always)]
    pub fn get_isolated_mask(&self, sq: Squares) -> BitBoard {
        self.isolated_mask[sq.file() as usize]
    }

    #[inline(always)]
    pub fn get_passed_mask(&self, color: Color, sq: Squares) -> BitBoard {
        self.passed_mask[color as usize][sq as usize]
    }

    #[inline(always)]
    pub fn get_game_phase_score(&self, board: &Board) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        for piece in BoardPiece::iter() {
            if piece == WhiteKing || piece == BlackKing || piece == WhitePawn || piece == BlackPawn
            {
                continue;
            }

            match piece.get_color() {
                White => {
                    white_score += board.get_bitboard(piece).count_bits() as i32
                        * MATERIAL_SCORES[Opening as usize][piece as usize];
                }
                Black => {
                    black_score += board.get_bitboard(piece).count_bits() as i32
                        * -MATERIAL_SCORES[Opening as usize][piece as usize];
                }
            }
        }

        white_score + black_score
    }

    #[inline(always)]
    pub fn get_pawn_score(&self, color: Color, square: Squares, board: &Board, opening_score: &mut i32, endgame_score: &mut i32) {

        match color {
            White => {
                *opening_score += PAWN_SCORES[Opening as usize][MIRROR[square as usize] as usize];
                *endgame_score += PAWN_SCORES[EndGame as usize][MIRROR[square as usize] as usize];

                let doubled =
                    (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).count_bits();
                if doubled > 1 {
                    *opening_score += DOUBLE_PAWN_PENALTY_OPENING * (doubled - 1) as i32;
                    *endgame_score += DOUBLE_PAWN_PENALTY_ENDGAME * (doubled - 1) as i32;
                }

                if (board.get_bitboard(WhitePawn) & self.get_isolated_mask(square)).is_empty() {
                    *opening_score += ISOLATED_PAWN_PENALTY_OPENING;
                    *endgame_score += ISOLATED_PAWN_PENALTY_ENDGAME;
                }

                if (self.get_passed_mask(White, square) & board.get_bitboard(BlackPawn)).is_empty()
                {
                    *opening_score += PASSED_PAWN_BONUS[square.rank() as usize];
                    *endgame_score += PASSED_PAWN_BONUS[square.rank() as usize];
                }
            }
            Black => {
                *opening_score -= PAWN_SCORES[Opening as usize][square as usize];
                *endgame_score -= PAWN_SCORES[EndGame as usize][square as usize];

                let doubled =
                    (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).count_bits();
                if doubled > 1 {
                    *opening_score -= DOUBLE_PAWN_PENALTY_OPENING * (doubled - 1) as i32;
                    *endgame_score -= DOUBLE_PAWN_PENALTY_ENDGAME * (doubled - 1) as i32;
                }

                if (board.get_bitboard(BlackPawn) & self.get_isolated_mask(square)).is_empty() {
                    *opening_score -= ISOLATED_PAWN_PENALTY_OPENING;
                    *endgame_score -= ISOLATED_PAWN_PENALTY_ENDGAME;
                }

                if (self.get_passed_mask(Black, square) & board.get_bitboard(WhitePawn)).is_empty()
                {
                    *opening_score -= PASSED_PAWN_BONUS[square.rank() as usize];
                    *endgame_score -= PASSED_PAWN_BONUS[square.rank() as usize];
                }
            }
        }
    }

    #[inline(always)]
    pub fn get_knight_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        opening_score: &mut i32,
        endgame_score: &mut i32,
    ) {
        match color {
            White => {
                *opening_score += KNIGHT_SCORES[Opening as usize][MIRROR[square as usize] as usize];
                *endgame_score += KNIGHT_SCORES[EndGame as usize][MIRROR[square as usize] as usize];

                // mobility bonus
                *endgame_score += ((gen.table.get_knight_attacks(square as u8) & board.get_white_occ()).count_bits() as i32 - KNIGHT_UNITS) * KNIGHT_MOB_ENDGAME;
                *opening_score += ((gen.table.get_knight_attacks(square as u8) & board.get_white_occ()).count_bits() as i32 - KNIGHT_UNITS) * KNIGHT_MOB_OPENING;
            }
            Black => {
                *opening_score -= KNIGHT_SCORES[Opening as usize][square as usize];
                *endgame_score -= KNIGHT_SCORES[EndGame as usize][square as usize];

                // mobility bonus
                *endgame_score -= ((gen.table.get_knight_attacks(square as u8) & board.get_black_occ()).count_bits() as i32 - KNIGHT_UNITS) * KNIGHT_MOB_ENDGAME;
                *opening_score -= ((gen.table.get_knight_attacks(square as u8) & board.get_black_occ()).count_bits() as i32 - KNIGHT_UNITS) * KNIGHT_MOB_OPENING;
            },
        }
    }

    #[inline(always)]
    pub fn get_bishop_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        opening_score: &mut i32,
        endgame_score: &mut i32,
    ) {

        match color {
            White => {
                *opening_score += BISHOP_SCORES[Opening as usize][MIRROR[square as usize] as usize];
                *endgame_score += BISHOP_SCORES[EndGame as usize][MIRROR[square as usize] as usize];

                // mobility bonus
                *endgame_score += (gen.table.get_bishop_attacks(square as u8, board.get_occ()).count_bits() as i32 - BISHOP_UNITS) * BISHOP_MOB_ENDGAME;
                *opening_score += (gen.table.get_bishop_attacks(square as u8, board.get_occ()).count_bits() as i32 - BISHOP_UNITS) * BISHOP_MOB_OPENING;
            }
            Black => {
                *opening_score -= BISHOP_SCORES[Opening as usize][square as usize];
                *endgame_score -= BISHOP_SCORES[EndGame as usize][square as usize];

                // mobility bonus
                *endgame_score -= (gen.table.get_bishop_attacks(square as u8, board.get_occ()).count_bits() as i32 - BISHOP_UNITS) * BISHOP_MOB_ENDGAME;
                *opening_score -= (gen.table.get_bishop_attacks(square as u8, board.get_occ()).count_bits() as i32 - BISHOP_UNITS) * BISHOP_MOB_OPENING;
            }
        }

    }

    #[inline(always)]
    pub fn get_rook_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        opening_score: &mut i32,
        endgame_score: &mut i32,
    ) {
        match color {
            White => {
                *opening_score += ROOK_SCORES[Opening as usize][MIRROR[square as usize] as usize];
                *endgame_score += ROOK_SCORES[EndGame as usize][MIRROR[square as usize] as usize];

                if (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).is_empty() {
                    *opening_score += SEMI_OPEN_FILE_SCORE;
                    *endgame_score += SEMI_OPEN_FILE_SCORE; // semi open file bonus
                }
                if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    *opening_score += OPEN_FILE_SCORE;
                    *endgame_score += OPEN_FILE_SCORE; // open file bonus
                }

                // rook on 7th bonus
                if square.rank() == 6 {
                    *opening_score += ROOK_ON_SEVENTH_OPENING_SCORE;
                    *endgame_score += ROOK_ON_SEVENTH_ENDGAME_SCORE;
                }

                // mobility bonus
                *endgame_score += (gen.table.get_rook_attacks(square as u8, board.get_occ()).count_bits() as i32 - ROOK_UNITS) * ROOK_MOB_ENDGAME;
                *opening_score += (gen.table.get_rook_attacks(square as u8, board.get_occ()).count_bits() as i32 - ROOK_UNITS) * ROOK_MOB_OPENING;
            }
            Black => {
                *opening_score -= ROOK_SCORES[Opening as usize][square as usize];
                *endgame_score -= ROOK_SCORES[EndGame as usize][square as usize];

                //open - semi open file penalty
                if (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).is_empty() {
                    *opening_score -= SEMI_OPEN_FILE_SCORE;
                    *endgame_score -= SEMI_OPEN_FILE_SCORE; // semi open file bonus
                }

                if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    *opening_score -= OPEN_FILE_SCORE;
                    *endgame_score -= OPEN_FILE_SCORE; // open file bonus
                }

                // rook on 7th bonus
                if square.rank() == 1 {
                    *opening_score -= ROOK_ON_SEVENTH_OPENING_SCORE;
                    *endgame_score -= ROOK_ON_SEVENTH_ENDGAME_SCORE;
                }

                // mobility bonus
                *endgame_score -= (gen.table.get_rook_attacks(square as u8, board.get_occ()).count_bits() as i32 - ROOK_UNITS) * ROOK_MOB_ENDGAME;
                *opening_score -= (gen.table.get_rook_attacks(square as u8, board.get_occ()).count_bits() as i32 - ROOK_UNITS) * ROOK_MOB_OPENING;

            }
        }
    }

    #[inline(always)]
    pub fn get_queen_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        opening_score: &mut i32,
        endgame_score: &mut i32,
    ) {

        match color {
            White => {
                *opening_score += QUEEN_SCORES[Opening as usize][MIRROR[square as usize] as usize];
                *endgame_score += QUEEN_SCORES[EndGame as usize][MIRROR[square as usize] as usize];

                // mobility bonus
                *endgame_score += (gen.table.get_queen_attacks(square as u8, board.get_occ()).count_bits() as i32 - QUEEN_UNITS) * QUEEN_MOB_ENDGAME;
                *opening_score += (gen.table.get_queen_attacks(square as u8, board.get_occ()).count_bits() as i32 - QUEEN_UNITS) * QUEEN_MOB_OPENING;

                // queen on 7th bonus
                if square.rank() == 6 {
                    *opening_score += QUEEN_ON_SEVENTH_OPENING_SCORE;
                    *endgame_score += QUEEN_ON_SEVENTH_ENDGAME_SCORE;
                }
            }
            Black => {
                *opening_score -= QUEEN_SCORES[Opening as usize][square as usize];
                *endgame_score -= QUEEN_SCORES[EndGame as usize][square as usize];

                // mobility bonus
                *endgame_score -= (gen.table.get_queen_attacks(square as u8, board.get_occ()).count_bits() as i32 - QUEEN_UNITS) * QUEEN_MOB_ENDGAME;
                *opening_score -= (gen.table.get_queen_attacks(square as u8, board.get_occ()).count_bits() as i32 - QUEEN_UNITS) * QUEEN_MOB_OPENING;

                // queen on 7th bonus
                if square.rank() == 1 {
                    *opening_score -= QUEEN_ON_SEVENTH_OPENING_SCORE;
                    *endgame_score -= QUEEN_ON_SEVENTH_ENDGAME_SCORE;
                }
            }
        }
    }

    #[inline(always)]
    pub fn get_king_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        opening_score: &mut i32,
        endgame_score: &mut i32,
    ){

        match color {
            White => {
                *opening_score += KING_SCORES[Opening as usize][MIRROR[square as usize] as usize];
                *endgame_score += KING_SCORES[EndGame as usize][MIRROR[square as usize] as usize];

                if (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).is_empty() {
                    *opening_score -= SEMI_OPEN_FILE_SCORE;
                    *endgame_score -= SEMI_OPEN_FILE_SCORE; // semi open file penalty
                }

                if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    *opening_score -= OPEN_FILE_SCORE;
                    *endgame_score -= OPEN_FILE_SCORE; // open file penalty
                }

                // king shield bonus
                *opening_score += (gen.table.get_king_attacks(square as u8) & board.get_white_occ()).count_bits() as i32 * KING_SHIELD_BONUS;
                *endgame_score += (gen.table.get_king_attacks(square as u8) & board.get_white_occ()).count_bits() as i32 * KING_SHIELD_BONUS;
            }
            Black => {
                *opening_score -= KING_SCORES[Opening as usize][square as usize];
                *endgame_score -= KING_SCORES[EndGame as usize][square as usize];

                if (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).is_empty() {
                    *opening_score += SEMI_OPEN_FILE_SCORE;
                    *endgame_score += SEMI_OPEN_FILE_SCORE; // semi open file penalty
                }

                if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    *opening_score += OPEN_FILE_SCORE;
                    *endgame_score += OPEN_FILE_SCORE; // open file penalty
                }

                // king shield bonus
                *opening_score -= (gen.table.get_king_attacks(square as u8) & board.get_black_occ()).count_bits() as i32 * KING_SHIELD_BONUS;
                *endgame_score -= (gen.table.get_king_attacks(square as u8) & board.get_black_occ()).count_bits() as i32 * KING_SHIELD_BONUS;
            }
        }
    }

    pub fn get_endgame_weight(&self, color: Color, board: &Board) -> f32 {
        const QUEEN_WEIGHT: u8 = 45;
        const ROOK_WEIGHT: u8 = 20;
        const BISHOP_WEIGHT: u8 = 10;
        const KNIGHT_WEIGHT: u8 = 10;

        match color {
            White => {
                let num_knights = board.get_bitboard(WhiteKnight).count_bits();
                let num_bishops = board.get_bitboard(WhiteBishop).count_bits();
                let num_rooks = board.get_bitboard(WhiteRook).count_bits();
                let num_queens = board.get_bitboard(WhiteQueen).count_bits();

                let endgame_start_weight = 2 * QUEEN_WEIGHT + 2 * ROOK_WEIGHT + 2 * BISHOP_WEIGHT + 2 * KNIGHT_WEIGHT;
                let endgame_weight_sum = num_knights * KNIGHT_WEIGHT + num_bishops * BISHOP_WEIGHT + num_rooks * ROOK_WEIGHT + num_queens * QUEEN_WEIGHT;

                let eg = endgame_weight_sum as f32/ endgame_start_weight as f32;
                return 1f32 - f32::min(1f32, eg);
            }
            Black => {
                let num_knights = board.get_bitboard(BlackKnight).count_bits();
                let num_bishops = board.get_bitboard(BlackBishop).count_bits();
                let num_rooks = board.get_bitboard(BlackRook).count_bits();
                let num_queens = board.get_bitboard(BlackQueen).count_bits();

                let endgame_start_weight = 2 * QUEEN_WEIGHT + 2 * ROOK_WEIGHT + 2 * BISHOP_WEIGHT + 2 * KNIGHT_WEIGHT;
                let endgame_weight_sum = num_knights * KNIGHT_WEIGHT + num_bishops * BISHOP_WEIGHT + num_rooks * ROOK_WEIGHT + num_queens * QUEEN_WEIGHT;

                let eg = endgame_weight_sum as f32/ endgame_start_weight as f32;
                return  1f32 - f32::min(1f32, eg);
            }
        }
    }

    pub fn mop_eval(&self, color: Color, board: &Board, friendly_score: i32, enemy_score: i32) -> i32 {
        const PAWN_VALUE: i32 = 80;
        let endgame_weight = self.get_endgame_weight(color, board);

        if friendly_score > enemy_score + 2 * PAWN_VALUE && endgame_weight > 0.0 {
            let mut mopup = 0.0;
            let white_king_square = board.get_king_square(White);
            let black_king_square = board.get_king_square(Black);

            let (friendly_king_square, enemy_king_square) = if board.get_side_to_move() == White {
                (white_king_square, black_king_square)
            } else {
                (black_king_square, white_king_square)
            };

            mopup += ((14 - self.orthogonal_distance[friendly_king_square as usize][enemy_king_square as usize]) * 4) as f32;
            mopup += (CENTER_MANHATTAN_DISTANCE[enemy_king_square as usize] * 10) as f32;

            return if board.get_side_to_move() == White {
                (mopup * endgame_weight) as i32
            } else {
                -(mopup * endgame_weight) as i32
            }
        }
        0
    }


    #[inline(always)]
    pub fn evaluate(&self, board: &Board, gen: &MovGen) -> i32 {
        let game_phase_score = self.get_game_phase_score(board);
        let mut game_phase;

        if game_phase_score > OPENING_PHASE_SCORE {
            game_phase = Opening;
        } else if game_phase_score < ENDGAME_PHASE_SCORE {
            game_phase = EndGame;
        } else {
            game_phase = MiddleGame;
        }

        let mut opening_score = 0;
        let mut endgame_score = 0;
        let mut white_material_score = 0;
        let mut black_material_score = 0;
        let mut score = 0;

        for piece in BoardPiece::iter() {
            let bb = board.get_piece_occ(piece);

            for bit in bb {
                let bit_sq = Squares::from_repr(bit).unwrap();

                // score += MATERIAL_SCORE[piece as usize];
                opening_score += MATERIAL_SCORES[Opening as usize][piece as usize];
                endgame_score += MATERIAL_SCORES[EndGame as usize][piece as usize];

                if piece.get_color() == White { // for mop eval
                    white_material_score += MATERIAL_SCORES[Opening as usize][piece as usize];
                } else {
                    black_material_score += MATERIAL_SCORES[Opening as usize][piece as usize];
                }

                match piece {
                    WhitePawn | BlackPawn => {
                        self.get_pawn_score(piece.get_color(), bit_sq, board, &mut opening_score, &mut endgame_score);
                    }
                    WhiteKnight | BlackKnight => {
                        self.get_knight_score(piece.get_color(), bit_sq, board, gen, &mut opening_score, &mut endgame_score);
                    }
                    WhiteBishop | BlackBishop => {
                        self.get_bishop_score(piece.get_color(), bit_sq, board, gen, &mut opening_score, &mut endgame_score);
                    }
                    WhiteRook | BlackRook => {
                        self.get_rook_score(piece.get_color(), bit_sq, board, gen, &mut opening_score, &mut endgame_score);
                    }
                    WhiteQueen | BlackQueen => {
                        self.get_queen_score(piece.get_color(), bit_sq, board, gen, &mut opening_score, &mut endgame_score);
                    }
                    WhiteKing | BlackKing => {
                        self.get_king_score(piece.get_color(), bit_sq, board, gen, &mut opening_score, &mut endgame_score);
                    }
                }
            }
        }

        if game_phase == EndGame {
            let white_mop = self.mop_eval(White, board, white_material_score, black_material_score);
            let black_mop = self.mop_eval(Black, board, black_material_score, white_material_score);

            endgame_score += white_mop + black_mop;
        }

        match game_phase {
            Opening => {
                score = opening_score;
            }
            EndGame => {
                score = endgame_score;
            }
            MiddleGame => {
                score = (opening_score * game_phase_score
                    + endgame_score * (OPENING_PHASE_SCORE - game_phase_score))
                    / OPENING_PHASE_SCORE;
            }
        }

        if board.get_side_to_move() == Black {
            score = -score;
        }

        score
    }
}

impl Default for Eval {
    fn default() -> Self {
        let mut eval = Eval {
            file_mask: [BitBoard(0); 8],
            rank_mask: [BitBoard(0); 8],
            isolated_mask: [BitBoard(0); 8],
            passed_mask: [[BitBoard(0); 64]; 2],
            orthogonal_distance: [[0; 64]; 64],
        };

        eval.init_file_mask();
        eval.init_rank_mask();
        eval.init_isolated_mask();
        eval.init_passed_mask();
        eval.init_orhtogonal_distance();

        eval
    }
}


#[inline(always)]
pub fn get_mvv_lva(mov: &Move) -> i32 {
    if mov.capture.is_none() {
        return 0;
    }

    MVV_LVA[mov.piece as usize][mov.capture.unwrap() as usize]
}
