use super::{
    BISHOP_SCORES, KING_SCORES, KNIGHT_SCORES, MATERIAL_SCORE, MVV_LVA, PAWN_SCORES, QUEEN_SCORES,
    ROOK_SCORES,
};
use crate::kelp::board::board::Board;
use crate::kelp::board::moves::Move;
use crate::kelp::board::piece::BoardPiece::{self, *};
use crate::kelp::board::piece::Color;
use crate::kelp::board::piece::Color::{Black, White};
use crate::kelp::kelp_core::bitboard::BitBoard;
use crate::kelp::mov_gen::generator::MovGen;
use crate::kelp::{Squares, MIRROR};
use strum::IntoEnumIterator;

const DOUBLE_PAWN_PENALTY: i32 = -10;
const ISOLATED_PAWN_PENALTY: i32 = -20;

const PASSED_PAWN_BONUS: [i32; 8] = [0, 5, 10, 20, 35, 60, 100, 200];

const SEMI_OPEN_FILE_SCORE: i32 = 10;
const OPEN_FILE_SCORE: i32 = 15;
const KING_SHIELD_BONUS: i32 = 5;
pub struct Eval {
    pub file_mask: [BitBoard; 8],
    pub rank_mask: [BitBoard; 8],
    pub isolated_mask: [BitBoard; 8],
    pub passed_mask: [[BitBoard; 64]; 2],
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
    pub fn get_pawn_score(&self, color: Color, square: Squares, board: &Board) -> i32 {
        let mut score = 0;

        match color {
            White => {
                score += PAWN_SCORES[MIRROR[square as usize] as usize];

                let doubled = (board.get_bitboard(WhitePawn) & self.get_file_mask(square))
                    .count_bits();
                if doubled > 1 {
                    score += DOUBLE_PAWN_PENALTY * doubled as i32;
                }

                if (board.get_bitboard(WhitePawn) & self.get_isolated_mask(square))
                    .is_empty()
                {
                    score += ISOLATED_PAWN_PENALTY;
                }

                if (self.get_passed_mask(White, square) & board.get_bitboard(BlackPawn))
                    .is_empty()
                {
                    score += PASSED_PAWN_BONUS[square.rank() as usize];
                }
            }
            Black => {
                score -= PAWN_SCORES[square as usize];

                let doubled = (board.get_bitboard(BlackPawn) & self.get_file_mask(square))
                    .count_bits();
                if doubled > 1 {
                    score -= DOUBLE_PAWN_PENALTY * doubled as i32;
                }

                if (board.get_bitboard(BlackPawn) & self.get_isolated_mask(square))
                    .is_empty()
                {
                    score -= ISOLATED_PAWN_PENALTY;
                }

                if (self.get_passed_mask(Black, square) & board.get_bitboard(WhitePawn))
                    .is_empty()
                {
                    score -= PASSED_PAWN_BONUS[7 - square.rank() as usize];
                }
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_knight_score(&self, color: Color, square: Squares, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;

        match color {
            White => score += KNIGHT_SCORES[MIRROR[square as usize] as usize],
            Black => score -= KNIGHT_SCORES[square as usize],
        }

        score
    }

    #[inline(always)]
    pub fn get_bishop_score(&self, color: Color, square: Squares, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;

        match color {
            White => {
                score += BISHOP_SCORES[MIRROR[square as usize] as usize];

                // mobility bonus
                // mobility bonus
                score += (gen.table.get_bishop_attacks(square as u8, board.get_occ())).count_bits()
                    as i32;
            }
            Black => {
                score -= BISHOP_SCORES[square as usize];

                // mobility bonus
                score -= (gen.table.get_bishop_attacks(square as u8, board.get_occ())).count_bits()
                    as i32;
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_rook_score(&self, color: Color, square: Squares, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;

        match color {
            White => {
                score += ROOK_SCORES[MIRROR[square as usize] as usize];

                if (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).is_empty() {
                    score += SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                    .is_empty()
                {
                    score += OPEN_FILE_SCORE;
                }
            }
            Black => {
                score -= ROOK_SCORES[square as usize];

                if (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).is_empty() {
                    score -= SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                    .is_empty()
                {
                    score -= OPEN_FILE_SCORE;
                }
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_queen_score(&self, color: Color, square: Squares, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;

        match color {
            White => {
                score += QUEEN_SCORES[MIRROR[square as usize] as usize];

                // mobility bonus
                score +=
                    (gen.table.get_queen_attacks(square as u8, board.get_occ())).count_bits() as i32;
            }
            Black => {
                score -= QUEEN_SCORES[square as usize];

                // mobility bonus
                score -=
                    (gen.table.get_queen_attacks(square as u8, board.get_occ())).count_bits() as i32;
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_king_score(&self, color: Color, square: Squares, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;

        match color {
            White => {
                score += KING_SCORES[MIRROR[square as usize] as usize];

                if (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).is_empty() {
                    score -= SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                    .is_empty()
                {
                    score -= OPEN_FILE_SCORE;
                }

                // king shield bonus
                score += ((gen.table.get_knight_attacks(square as u8) & board.get_white_occ()).count_bits() * KING_SHIELD_BONUS as u8) as i32;
            }
            Black => {
                score -= KING_SCORES[square as usize];

                if (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).is_empty() {
                    score += SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                    .is_empty()
                {
                    score += OPEN_FILE_SCORE;
                }

                // king shield bonus
                score -= ((gen.table.get_knight_attacks(square as u8) & board.get_black_occ()).count_bits() * KING_SHIELD_BONUS as u8) as i32;
            }
        }

        score
    }

    #[inline(always)]
    pub fn evaluate(&self, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;

        for piece in BoardPiece::iter() {
            let bb = board.get_bitboard(piece);

            for bit in bb {
                let bit_sq = Squares::from_repr(bit).unwrap();
                score += MATERIAL_SCORE[piece as usize];
                match piece {
                    WhitePawn | BlackPawn => {
                        score += self.get_pawn_score(piece.get_color(), bit_sq, board);
                    }
                    WhiteKnight | BlackKnight => {
                        score += self.get_knight_score(piece.get_color(), bit_sq, board, gen);
                    }
                    WhiteBishop | BlackBishop => {
                        score += self.get_bishop_score(piece.get_color(), bit_sq, board, gen);
                    }
                    WhiteRook | BlackRook => {
                        score += self.get_rook_score(piece.get_color(), bit_sq, board, gen);
                    }
                    WhiteQueen | BlackQueen => {
                        score += self.get_queen_score(piece.get_color(), bit_sq, board, gen);
                    }
                    WhiteKing | BlackKing => {
                        score += self.get_king_score(piece.get_color(), bit_sq, board, gen);
                    }
                }
            }
        }

        if board.get_side_to_move() == White {
            score
        } else {
            -score
        }
    }
}

impl Default for Eval {
    fn default() -> Self {
        let mut eval = Eval {
            file_mask: [BitBoard(0); 8],
            rank_mask: [BitBoard(0); 8],
            isolated_mask: [BitBoard(0); 8],
            passed_mask: [[BitBoard(0); 64]; 2],
        };

        eval.init_file_mask();
        eval.init_rank_mask();
        eval.init_isolated_mask();
        eval.init_passed_mask();

        eval
    }
}

#[inline(always)]
pub fn eval(board: &Board) -> i32 {
    let mut score = 0;

    for piece in BoardPiece::iter() {
        let bb = board.get_bitboard(piece);

        for bit in bb {
            score += MATERIAL_SCORE[piece as usize];
            match piece {
                WhitePawn => score += PAWN_SCORES[MIRROR[bit as usize] as usize],
                WhiteKnight => score += KNIGHT_SCORES[MIRROR[bit as usize] as usize],
                WhiteBishop => score += BISHOP_SCORES[MIRROR[bit as usize] as usize],
                WhiteRook => score += ROOK_SCORES[MIRROR[bit as usize] as usize],
                WhiteKing => score += KING_SCORES[MIRROR[bit as usize] as usize],
                WhiteQueen => score += QUEEN_SCORES[MIRROR[bit as usize] as usize],

                BlackPawn => score -= PAWN_SCORES[bit as usize],
                BlackKnight => score -= KNIGHT_SCORES[bit as usize],
                BlackBishop => score -= BISHOP_SCORES[bit as usize],
                BlackRook => score -= ROOK_SCORES[bit as usize],
                BlackKing => score -= KING_SCORES[bit as usize],
                BlackQueen => score -= QUEEN_SCORES[bit as usize],
            }
        }
    }

    if board.get_side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

#[inline(always)]
pub fn get_mvv_lva(mov: &Move) -> i32 {
    if mov.capture.is_none() {
        return 0;
    }

    MVV_LVA[mov.piece as usize][mov.capture.unwrap() as usize]
}
