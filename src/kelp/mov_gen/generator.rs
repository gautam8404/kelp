use log::info;
use crate::kelp::board::board::Board;
use crate::kelp::kelp_core::bitboard::BitBoard;
use crate::kelp::kelp_core::lookup_table::LookupTable;
use crate::kelp::{
    board::piece::{
        BoardPiece::{self, *},
        Color::{self, *},
    },
    board::moves::{Move, MoveList, MoveType},
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
        info!("Initializing move generator");
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
                    & (board.get_piece_occ(WhiteBishop) | board.get_piece_occ(WhiteQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
                if (self.table.get_rook_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(WhiteRook) | board.get_piece_occ(WhiteQueen)))
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

    pub fn get_pawn_moves(&self ,color: Color, bitboard: BitBoard) -> MoveList {
        let mut moves = match color {
            White => {
                let mut moves = MoveList::new();
                for sq in bitboard {
                    let square = Squares::from_repr(sq).unwrap();
                    let mut mv = Move::new(square, square + 8, WhitePawn, None, MoveType::Normal);
                    if square.rank() == 2 {
                        mv.set_type(MoveType::DoublePawnPush);
                        moves.push(mv);
                    } else if square.rank() == 7 {
                        mv.set_type(MoveType::Promotion(Some(WhiteQueen)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(WhiteRook)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(WhiteBishop)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(WhiteKnight)));
                        moves.push(mv);
                    } else {
                        moves.push(mv);
                    }
                }
                moves
            }
            Black => {
                let mut moves = MoveList::new();
                for sq in bitboard {
                    let square = Squares::from_repr(sq).unwrap();
                    let mut mv = Move::new(square, square - 8, BlackPawn, None, MoveType::Normal);
                    if square.rank() == 7 {
                        mv.set_type(MoveType::DoublePawnPush);
                        moves.push(mv);
                    } else if square.rank() == 2 {
                        mv.set_type(MoveType::Promotion(Some(BlackQueen)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(BlackRook)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(BlackBishop)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(BlackKnight)));
                        moves.push(mv);
                    } else {
                        moves.push(mv);
                    }
                }
                moves
            }
        };

        moves
    }
    pub fn generate_moves(&self, color: Color, board: &Board) -> MoveList {
        todo!("generate moves")
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
