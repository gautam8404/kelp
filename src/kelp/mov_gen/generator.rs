use crate::kelp::board::board::Board;
use crate::kelp::kelp_core::bitboard::BitBoard;
use crate::kelp::kelp_core::lookup_table::LookupTable;
use crate::kelp::{
    board::piece::{
        BoardPiece::{self, *},
        Color::{self, *},
    },
    Squares,
};
use strum::IntoEnumIterator;

enum GenType {
    Quiet,
    Capture,
}

pub struct MovGen<'a> {
    pub table: &'a LookupTable,
}

impl<'a> MovGen<'a> {
    pub fn new(table: &'a LookupTable) -> MovGen<'a> {
        MovGen { table: table }
    }
    pub fn is_attacked(&self, square: Squares, color: Color, board: &Board) -> bool {
        match color {
            White => {
                if (self.table.get_pawn_attacks(Black, square as u8)
                    & board.get_piece_occ(WhitePawn))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_knight_attacks(square as u8) & board.get_piece_occ(WhiteKnight))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_king_attacks(square as u8) & board.get_piece_occ(WhiteKing))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_bishop_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(WhiteBishop)) | (board.get_piece_occ(WhiteQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_rook_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(WhiteRook)) | (board.get_piece_occ(WhiteQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
            }
            Black => {
                if (self.table.get_pawn_attacks(White, square as u8)
                    & board.get_piece_occ(BlackPawn))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_knight_attacks(square as u8) & board.get_piece_occ(BlackKnight))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_king_attacks(square as u8) & board.get_piece_occ(BlackKing))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_bishop_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(BlackBishop) | board.get_piece_occ(BlackQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_rook_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(BlackRook) | board.get_piece_occ(BlackQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn print_attacked(&self, color: Color, board: &Board) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                if self.is_attacked(Squares::from_rank_file(rank, file), color, board) {
                    print!("  1");
                } else {
                    print!("  0");
                }

                if file == 7 {
                    print!("\t{}", rank + 1);
                }
            }
            println!();
        }
        print!("\n  a  b  c  d  e  f  g  h\n");
    }
}
