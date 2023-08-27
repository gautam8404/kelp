use super::{
    BISHOP_SCORES, ENDGAME_PHASE_SCORE, KING_SCORES, KNIGHT_SCORES, MATERIAL_SCORES, MVV_LVA,
    OPENING_PHASE_SCORE, PAWN_SCORES, QUEEN_SCORES, ROOK_SCORES,
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

const DOUBLE_PAWN_PENALTY: i32 = -10;
const ISOLATED_PAWN_PENALTY: i32 = -20;

const PASSED_PAWN_BONUS: [i32; 8] = [0, 5, 10, 20, 35, 60, 100, 200];

const SEMI_OPEN_FILE_SCORE: i32 = 10;
const OPEN_FILE_SCORE: i32 = 15;
const ROOK_ON_SEVENTH_OPENING_SCORE: i32 = 20;
const ROOK_ON_SEVENTH_ENDGAME_SCORE: i32 = 40;
const QUEEN_ON_SEVENTH_OPENING_SCORE: i32 = 10;
const QUEEN_ON_SEVENTH_ENDGAME_SCORE: i32 = 20;
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
    fn get_interpolated_score(
        &self,
        opening_score: i32,
        endgame_score: i32,
        game_phase_score: i32,
    ) -> i32 {
        (opening_score * game_phase_score
            + endgame_score * (OPENING_PHASE_SCORE - game_phase_score))
            / OPENING_PHASE_SCORE
    }

    #[inline(always)]
    fn get_interpolated_positional_score(
        &self,
        piece: BoardPiece,
        square: Squares,
        game_phase_score: i32,
    ) -> i32 {
        let mut score = 0;

        match piece {
            WhitePawn => {
                score += self.get_interpolated_score(
                    PAWN_SCORES[Opening as usize][MIRROR[square as usize] as usize],
                    PAWN_SCORES[EndGame as usize][MIRROR[square as usize] as usize],
                    game_phase_score,
                );
            }
            WhiteKnight => {
                score += self.get_interpolated_score(
                    KNIGHT_SCORES[Opening as usize][MIRROR[square as usize] as usize],
                    KNIGHT_SCORES[EndGame as usize][MIRROR[square as usize] as usize],
                    game_phase_score,
                );
            }
            WhiteBishop => {
                score += self.get_interpolated_score(
                    BISHOP_SCORES[Opening as usize][MIRROR[square as usize] as usize],
                    BISHOP_SCORES[EndGame as usize][MIRROR[square as usize] as usize],
                    game_phase_score,
                );
            }
            WhiteRook => {
                score += self.get_interpolated_score(
                    ROOK_SCORES[Opening as usize][MIRROR[square as usize] as usize],
                    ROOK_SCORES[EndGame as usize][MIRROR[square as usize] as usize],
                    game_phase_score,
                );
            }
            WhiteQueen => {
                score += self.get_interpolated_score(
                    QUEEN_SCORES[Opening as usize][MIRROR[square as usize] as usize],
                    QUEEN_SCORES[EndGame as usize][MIRROR[square as usize] as usize],
                    game_phase_score,
                );
            }
            WhiteKing => {
                score += self.get_interpolated_score(
                    KING_SCORES[Opening as usize][MIRROR[square as usize] as usize],
                    KING_SCORES[EndGame as usize][MIRROR[square as usize] as usize],
                    game_phase_score,
                );
            }
            BlackPawn => {
                score += self.get_interpolated_score(
                    PAWN_SCORES[Opening as usize][square as usize],
                    PAWN_SCORES[EndGame as usize][square as usize],
                    game_phase_score,
                );
            }
            BlackKnight => {
                score += self.get_interpolated_score(
                    KNIGHT_SCORES[Opening as usize][square as usize],
                    KNIGHT_SCORES[EndGame as usize][square as usize],
                    game_phase_score,
                );
            }
            BlackBishop => {
                score += self.get_interpolated_score(
                    BISHOP_SCORES[Opening as usize][square as usize],
                    BISHOP_SCORES[EndGame as usize][square as usize],
                    game_phase_score,
                );
            }
            BlackRook => {
                score += self.get_interpolated_score(
                    ROOK_SCORES[Opening as usize][square as usize],
                    ROOK_SCORES[EndGame as usize][square as usize],
                    game_phase_score,
                );
            }
            BlackQueen => {
                score += self.get_interpolated_score(
                    QUEEN_SCORES[Opening as usize][square as usize],
                    QUEEN_SCORES[EndGame as usize][square as usize],
                    game_phase_score,
                );
            }
            BlackKing => {
                score += self.get_interpolated_score(
                    KING_SCORES[Opening as usize][square as usize],
                    KING_SCORES[EndGame as usize][square as usize],
                    game_phase_score,
                );
            }
        }

        score
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
    pub fn get_pawn_score(&self, color: Color, square: Squares, board: &Board, game_phase_score: i32, phase: GamePhase) -> i32 {
        let mut score = 0;

        match color {
            White => {
                if phase == MiddleGame {
                    score += self.get_interpolated_positional_score(
                        WhitePawn,
                        square,
                        game_phase_score);
                } else {
                    score += PAWN_SCORES[phase as usize][MIRROR[square as usize] as usize];
                }

                let doubled =
                    (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).count_bits();
                if doubled > 1 {
                    score += DOUBLE_PAWN_PENALTY * doubled as i32;
                }

                if (board.get_bitboard(WhitePawn) & self.get_isolated_mask(square)).is_empty() {
                    score += ISOLATED_PAWN_PENALTY;
                }

                if (self.get_passed_mask(White, square) & board.get_bitboard(BlackPawn)).is_empty()
                {
                    score += PASSED_PAWN_BONUS[square.rank() as usize];
                }
            }
            Black => {
                if phase == MiddleGame {
                    score -= self.get_interpolated_positional_score(
                        BlackPawn,
                        square,
                        game_phase_score);
                } else {
                    score -= PAWN_SCORES[phase as usize][square as usize];
                }

                let doubled =
                    (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).count_bits();
                if doubled > 1 {
                    score -= DOUBLE_PAWN_PENALTY * doubled as i32;
                }

                if (board.get_bitboard(BlackPawn) & self.get_isolated_mask(square)).is_empty() {
                    score -= ISOLATED_PAWN_PENALTY;
                }

                if (self.get_passed_mask(Black, square) & board.get_bitboard(WhitePawn)).is_empty()
                {
                    score -= PASSED_PAWN_BONUS[7 - square.rank() as usize];
                }
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_knight_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        game_phase_score: i32,
        phase: GamePhase
    ) -> i32 {
        let mut score = 0;

        match color {
            White => {
                if phase == MiddleGame {
                    score += self.get_interpolated_positional_score(
                        WhiteKnight,
                        square,
                        game_phase_score
                    );
                } else {
                    score += KNIGHT_SCORES[phase as usize][MIRROR[square as usize] as usize];
                }
            }
            Black => {
                if phase == MiddleGame {
                    score -= self.get_interpolated_positional_score(
                        BlackKnight,
                        square,
                        game_phase_score
                    );
                } else {
                    score -= KNIGHT_SCORES[phase as usize][square as usize];
                }
            },
        }

        score
    }

    #[inline(always)]
    pub fn get_bishop_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        game_phase_score: i32,
        phase: GamePhase
    ) -> i32 {
        let mut score = 0;

        match color {
            White => {
                if phase == MiddleGame {
                    score += self.get_interpolated_positional_score(
                        WhiteBishop,
                        square,
                        game_phase_score
                    );
                } else {
                    score += BISHOP_SCORES[phase as usize][MIRROR[square as usize] as usize];
                }

                // mobility bonus
                score += (gen.table.get_bishop_attacks(square as u8, board.get_occ())).count_bits()
                    as i32;
            }
            Black => {
                if phase == MiddleGame {
                    score -= self.get_interpolated_positional_score(
                        BlackBishop,
                        square,
                        game_phase_score
                    );
                } else {
                    score -= BISHOP_SCORES[phase as usize][square as usize];
                }

                // mobility bonus
                score -= (gen.table.get_bishop_attacks(square as u8, board.get_occ())).count_bits()
                    as i32;
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_rook_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        game_phase_score: i32,
        phase: GamePhase
    ) -> i32 {
        let mut score = 0;

        match color {
            White => {
                if phase == MiddleGame {
                    score += self.get_interpolated_positional_score(
                        WhiteRook,
                        square,
                        game_phase_score
                    );
                } else {
                    score += ROOK_SCORES[phase as usize][MIRROR[square as usize] as usize];
                }

                if (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).is_empty() {
                    score += SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    score += OPEN_FILE_SCORE;
                }

                // rook on 7th bonus
                if phase == Opening || phase == MiddleGame {
                    if square.rank() == 6 {
                        score += ROOK_ON_SEVENTH_OPENING_SCORE;
                    }
                } else if square.rank() == 6 {
                    score += ROOK_ON_SEVENTH_ENDGAME_SCORE;
                }
            }
            Black => {
                if phase == MiddleGame {
                    score -= self.get_interpolated_positional_score(
                        BlackRook,
                        square,
                        game_phase_score
                    );
                } else {
                    score -= ROOK_SCORES[phase as usize][square as usize];
                }

                //open - semi open file penalty
                if (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).is_empty() {
                    score -= SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    score -= OPEN_FILE_SCORE;
                }

                // rook on 7th bonus
                if phase == Opening || phase == MiddleGame {
                    if square.rank() == 1 {
                        score -= ROOK_ON_SEVENTH_OPENING_SCORE;
                    }
                } else if square.rank() == 1 {
                    score -= ROOK_ON_SEVENTH_ENDGAME_SCORE;
                }

            }
        }

        score
    }

    #[inline(always)]
    pub fn get_queen_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        game_phase_score: i32,
        phase: GamePhase
    ) -> i32 {
        let mut score = 0;

        match color {
            White => {
                if phase == MiddleGame {
                    score += self.get_interpolated_positional_score(
                        WhiteQueen,
                        square,
                        game_phase_score
                    );
                } else {
                    score += QUEEN_SCORES[phase as usize][MIRROR[square as usize] as usize];
                }

                // mobility bonus
                score += (gen.table.get_queen_attacks(square as u8, board.get_occ())).count_bits()
                    as i32;

                // queen on 7th bonus
                if phase == Opening || phase == MiddleGame {
                    if square.rank() == 6 {
                        score += QUEEN_ON_SEVENTH_OPENING_SCORE;
                    }
                } else if square.rank() == 6 {
                    score += QUEEN_ON_SEVENTH_ENDGAME_SCORE;
                }
            }
            Black => {
                if phase == MiddleGame {
                    score -= self.get_interpolated_positional_score(
                        BlackQueen,
                        square,
                        game_phase_score
                    );
                } else {
                    score -= QUEEN_SCORES[phase as usize][square as usize];
                }

                // mobility bonus
                score -= (gen.table.get_queen_attacks(square as u8, board.get_occ())).count_bits()
                    as i32;

                // queen on 7th bonus
                if phase == Opening || phase == MiddleGame {
                    if square.rank() == 1 {
                        score -= QUEEN_ON_SEVENTH_OPENING_SCORE;
                    }
                } else if square.rank() == 1 {
                    score -= QUEEN_ON_SEVENTH_ENDGAME_SCORE;
                }
            }
        }

        score
    }

    #[inline(always)]
    pub fn get_king_score(
        &self,
        color: Color,
        square: Squares,
        board: &Board,
        gen: &MovGen,
        game_phase_score: i32,
        phase: GamePhase
    ) -> i32 {
        let mut score = 0;

        match color {
            White => {
                if phase == MiddleGame {
                    score += self.get_interpolated_positional_score(
                        WhiteKing,
                        square,
                        game_phase_score
                    );
                } else {
                    score += KING_SCORES[phase as usize][MIRROR[square as usize] as usize];
                }

                if (board.get_bitboard(WhitePawn) & self.get_file_mask(square)).is_empty() {
                    score -= SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    score -= OPEN_FILE_SCORE;
                }

                // king shield bonus
                score += ((gen.table.get_king_attacks(square as u8) & board.get_white_occ())
                    .count_bits()
                    * KING_SHIELD_BONUS as u8) as i32;
            }
            Black => {
                if phase == MiddleGame {
                    score -= self.get_interpolated_positional_score(
                        BlackKing,
                        square,
                        game_phase_score
                    );
                } else {
                    score -= KING_SCORES[phase as usize][square as usize];
                }

                if (board.get_bitboard(BlackPawn) & self.get_file_mask(square)).is_empty() {
                    score += SEMI_OPEN_FILE_SCORE;
                } else if ((board.get_bitboard(WhitePawn) | board.get_bitboard(BlackPawn))
                    & self.get_file_mask(square))
                .is_empty()
                {
                    score += OPEN_FILE_SCORE;
                }

                // king shield bonus
                score -= ((gen.table.get_king_attacks(square as u8) & board.get_black_occ())
                    .count_bits()
                    * KING_SHIELD_BONUS as u8) as i32;
            }
        }

        score
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

        let mut score = 0;

        for piece in BoardPiece::iter() {
            let bb = board.get_piece_occ(piece);

            for bit in bb {
                let bit_sq = Squares::from_repr(bit).unwrap();

                // score += MATERIAL_SCORE[piece as usize];
                if game_phase == MiddleGame {
                    score += self.get_interpolated_score(
                        MATERIAL_SCORES[Opening as usize][piece as usize],
                        MATERIAL_SCORES[EndGame as usize][piece as usize],
                        game_phase_score,
                    );

                } else {
                    score += MATERIAL_SCORES[game_phase as usize][piece as usize];
                }

                match piece {
                    WhitePawn | BlackPawn => {
                        score += self.get_pawn_score(piece.get_color(), bit_sq, board, game_phase_score, game_phase);
                    }
                    WhiteKnight | BlackKnight => {
                        score += self.get_knight_score(piece.get_color(), bit_sq, board, gen, game_phase_score, game_phase);
                    }
                    WhiteBishop | BlackBishop => {
                        score += self.get_bishop_score(piece.get_color(), bit_sq, board, gen, game_phase_score, game_phase);
                    }
                    WhiteRook | BlackRook => {
                        score += self.get_rook_score(piece.get_color(), bit_sq, board, gen, game_phase_score, game_phase);
                    }
                    WhiteQueen | BlackQueen => {
                        score += self.get_queen_score(piece.get_color(), bit_sq, board, gen, game_phase_score, game_phase);
                    }
                    WhiteKing | BlackKing => {
                        score += self.get_king_score(piece.get_color(), bit_sq, board, gen, game_phase_score, game_phase);
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
pub fn get_mvv_lva(mov: &Move) -> i32 {
    if mov.capture.is_none() {
        return 0;
    }

    MVV_LVA[mov.piece as usize][mov.capture.unwrap() as usize]
}
