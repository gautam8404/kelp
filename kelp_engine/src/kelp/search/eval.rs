use super::{BISHOP_SCORES, KING_SCORES, KNIGHT_SCORES, MATERIAL_SCORE, PAWN_SCORES, ROOK_SCORES};
use crate::kelp::board::board::Board;
use crate::kelp::board::piece::BoardPiece::{self, *};
use crate::kelp::board::piece::Color;
use crate::kelp::MIRROR;
use strum::IntoEnumIterator;

pub fn total_board_eval(board: &Board) -> i32 {
    let mut score = 0;

    for piece in BoardPiece::iter() {
        let bb = board.get_bitboard(piece);

        for bit in bb {
            score += MATERIAL_SCORE[piece as usize];

            match piece {
                WhitePawn => score += PAWN_SCORES[bit as usize] as i32,
                WhiteKnight => score += KNIGHT_SCORES[bit as usize] as i32,
                WhiteBishop => score += BISHOP_SCORES[bit as usize] as i32,
                WhiteRook => score += ROOK_SCORES[bit as usize] as i32,
                WhiteKing => score += KING_SCORES[bit as usize] as i32,

                BlackPawn => score -= PAWN_SCORES[MIRROR[bit as usize] as usize] as i32,
                BlackKnight => score -= KNIGHT_SCORES[MIRROR[bit as usize] as usize] as i32,
                BlackBishop => score -= BISHOP_SCORES[MIRROR[bit as usize] as usize] as i32,
                BlackRook => score -= ROOK_SCORES[MIRROR[bit as usize] as usize] as i32,
                BlackKing => score -= KING_SCORES[MIRROR[bit as usize] as usize] as i32,

                _ => {}
            }
        }
    }

    if board.get_side_to_move() == Color::White {
        score
    } else {
        -score
    }
}
