use super::{
    BISHOP_SCORES, KING_SCORES, KNIGHT_SCORES, MATERIAL_SCORE, MVV_LVA, PAWN_SCORES, ROOK_SCORES, QUEEN_SCORES
};
use crate::kelp::board::board::Board;
use crate::kelp::board::moves::Move;
use crate::kelp::board::piece::BoardPiece::{self, *};
use crate::kelp::board::piece::Color;
use crate::kelp::MIRROR;
use strum::IntoEnumIterator;

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
