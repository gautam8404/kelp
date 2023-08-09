use crate::kelp::board::board::Board;
use crate::kelp::board::moves::{CastlingRights, GenType};
use crate::kelp::kelp_core::bitboard::BitBoard;
use crate::kelp::kelp_core::lookup_table::LookupTable;
use crate::kelp::{
    board::moves::{Move, MoveList, MoveType},
    board::piece::{
        BoardPiece::{self, *},
        Color::{self, *},
    },
    Squares,
};
use log::info;
use strum::IntoEnumIterator;

pub struct MovGen<'a> {
    pub table: &'a LookupTable,
    pub move_list: MoveList, // TODO: Make this private
}

// setters and getters
impl<'a> MovGen<'a> {
    pub fn get_move_list(&self) -> MoveList {
        self.move_list.clone()
    }
    
    pub fn get_move_list_ref(&self) -> &MoveList {
        &self.move_list
    }

    pub fn clear_move_list(&mut self) {
        self.move_list.clear();
    }
}

// Move generation
impl<'a> MovGen<'a> {
    pub fn new(table: &'a LookupTable) -> MovGen<'a> {
        info!("Initializing move generator");
        MovGen {
            table: table,
            move_list: MoveList::new(),
        }
    }

    pub fn is_attacked(&self, square: Squares, color: Color, board: &Board) -> bool {
        match color {
            White => {
                // is attacked by white pawn
                if (self.table.get_pawn_attacks(Black, square as u8)
                    & board.get_piece_occ(WhitePawn))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by white knight
                if (self.table.get_knight_attacks(square as u8) & board.get_piece_occ(WhiteKnight))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by white king
                if (self.table.get_king_attacks(square as u8) & board.get_piece_occ(WhiteKing))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by white bishop or queen
                if (self.table.get_bishop_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(WhiteBishop) | board.get_piece_occ(WhiteQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by white rook or queen
                if (self.table.get_rook_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(WhiteRook) | board.get_piece_occ(WhiteQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
            }
            Black => {
                // is attacked by black pawn
                if (self.table.get_pawn_attacks(White, square as u8)
                    & board.get_piece_occ(BlackPawn))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by black knight
                if (self.table.get_knight_attacks(square as u8) & board.get_piece_occ(BlackKnight))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by black king
                if (self.table.get_king_attacks(square as u8) & board.get_piece_occ(BlackKing))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by black bishop or queen
                if (self.table.get_bishop_attacks(square as u8, board.get_occ())
                    & (board.get_piece_occ(BlackBishop) | board.get_piece_occ(BlackQueen)))
                    != BitBoard::empty()
                {
                    return true;
                }
                // is attacked by black rook or queen
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

    fn generate_pawn_moves(&mut self, side: Color, board: &Board) {
        match side {
            White => {
                let bitboard = board.get_piece_occ(WhitePawn);
                let moves = &mut self.move_list;

                for sq in bitboard {
                    // Check if pawn is on the 8th rank or 1st rank
                    if sq >= 56 || sq <= 7 {
                        continue;
                    }
                    // Check if pawn is blocked
                    if board.get_occ().get_bit(sq + 8) {
                        continue;
                    }
                    let square = Squares::from_repr(sq).unwrap();

                    // Generate normal pawn push
                    let mut mv = Move::new(
                        square,
                        square + 8,
                        WhitePawn,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    );
                    moves.push(mv);

                    // Generate double pawn push
                    if square.rank() == 1 && !board.get_occ().get_bit(sq + 16) {
                        mv = Move::new(
                            square,
                            square + 16,
                            WhitePawn,
                            None,
                            MoveType::DoublePawnPush,
                            GenType::Quiet,
                        );
                        moves.push(mv);

                        // Generate Pawn Promotions
                    } else if square.rank() == 6 {
                        mv.set_type(MoveType::Promotion(Some(WhiteQueen)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(WhiteRook)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(WhiteBishop)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(WhiteKnight)));
                        moves.push(mv);
                    }
                    // Generate Pawn Captures
                    let attacks = self.table.get_pawn_attacks(White, sq);
                    for attack in attacks {
                        let target = Squares::from_repr(attack as u8).unwrap();
                        let piece = board.get_piece(target);
                        if piece.is_some() {
                            let piece = piece.unwrap();
                            if piece.get_color() != side {
                                mv = Move::new(
                                    square,
                                    target,
                                    WhitePawn,
                                    Some(piece),
                                    MoveType::Normal,
                                    GenType::Capture,
                                );
                                moves.push(mv);

                                // Generate Pawn Promotion Captures
                                if target.rank() == 7 {
                                    mv.set_type(MoveType::Promotion(Some(WhiteQueen)));
                                    moves.push(mv);
                                    mv.set_type(MoveType::Promotion(Some(WhiteRook)));
                                    moves.push(mv);
                                    mv.set_type(MoveType::Promotion(Some(WhiteBishop)));
                                    moves.push(mv);
                                    mv.set_type(MoveType::Promotion(Some(WhiteKnight)));
                                    moves.push(mv);
                                }
                            }
                        }
                    }

                    // Generate En Passant Captures
                    if board.get_en_passant().is_some() {
                        let en_passant = board.get_en_passant().unwrap();
                        if attacks.get_bit(en_passant as u8) {
                            mv = Move::new(
                                square,
                                en_passant,
                                WhitePawn,
                                Some(BlackPawn),
                                MoveType::EnPassant(Some(en_passant)),
                                GenType::Capture,
                            );
                            moves.push(mv);
                        }
                    }
                }
            }
            Black => {
                let bitboard = board.get_piece_occ(BlackPawn);
                let mut moves = &mut self.move_list;
                for sq in bitboard {
                    // Check if pawn is on the 8th rank or 1st rank
                    if sq >= 56 || sq <= 7 {
                        continue;
                    }
                    // Check if pawn is blocked
                    if board.get_occ().get_bit(sq - 8) {
                        continue;
                    }
                    let square = Squares::from_repr(sq).unwrap();

                    // Generate normal pawn push
                    let mut mv = Move::new(
                        square,
                        square - 8,
                        BlackPawn,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    );
                    moves.push(mv);

                    // Generate double pawn push
                    if square.rank() == 6 && !board.get_occ().get_bit(sq - 16) {
                        mv = Move::new(
                            square,
                            square - 16,
                            BlackPawn,
                            None,
                            MoveType::DoublePawnPush,
                            GenType::Quiet,
                        );
                        moves.push(mv);

                        // Generate Pawn Promotions
                    } else if square.rank() == 1 {
                        mv.set_type(MoveType::Promotion(Some(BlackQueen)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(BlackRook)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(BlackBishop)));
                        moves.push(mv);
                        mv.set_type(MoveType::Promotion(Some(BlackKnight)));
                        moves.push(mv);
                    }

                    // Generate Pawn Captures

                    let attacks = self.table.get_pawn_attacks(Black, sq);
                    for attack in attacks {
                        let target = Squares::from_repr(attack as u8).unwrap();
                        let piece = board.get_piece(target);
                        if piece.is_some() {
                            let piece = piece.unwrap();
                            if piece.get_color() != side {
                                mv = Move::new(
                                    square,
                                    target,
                                    BlackPawn,
                                    Some(piece),
                                    MoveType::Normal,
                                    GenType::Capture,
                                );
                                moves.push(mv);

                                // Generate Pawn Promotion Captures
                                if target.rank() == 0 {
                                    mv.set_type(MoveType::Promotion(Some(BlackQueen)));
                                    moves.push(mv);
                                    mv.set_type(MoveType::Promotion(Some(BlackRook)));
                                    moves.push(mv);
                                    mv.set_type(MoveType::Promotion(Some(BlackBishop)));
                                    moves.push(mv);
                                    mv.set_type(MoveType::Promotion(Some(BlackKnight)));
                                    moves.push(mv);
                                }
                            }
                        }
                    }

                    // Generate En Passant Captures
                    if board.get_en_passant().is_some() {
                        let en_passant = board.get_en_passant().unwrap();
                        if attacks.get_bit(en_passant as u8) {
                            mv = Move::new(
                                square,
                                en_passant,
                                BlackPawn,
                                Some(WhitePawn),
                                MoveType::EnPassant(Some(en_passant)),
                                GenType::Capture,
                            );
                            moves.push(mv);
                        }
                    }
                }
            }
        };
    }

    fn generate_castling_moves(&mut self, side: Color, board: &Board) {
        let castle = board.info.castle;

        match side {
            White => {
                // Generate King Side Castle
                if castle.can_castle_king_side(White)
                    && !board.get_occ().get_bit(Squares::F1 as u8)
                    && !board.get_occ().get_bit(Squares::G1 as u8)
                    && !self.is_attacked(Squares::E1, !side, board)
                    && !self.is_attacked(Squares::F1, !side, board)
                {
                    self.move_list.push(Move::new(
                        Squares::E1,
                        Squares::G1,
                        WhiteKing,
                        None,
                        MoveType::Castle(CastlingRights::WhiteKingSide),
                        GenType::Quiet,
                    ));
                }

                // Generate Queen Side Castle
                if castle.can_castle_queen_side(White)
                    && !board.get_occ().get_bit(Squares::D1 as u8)
                    && !board.get_occ().get_bit(Squares::C1 as u8)
                    && !board.get_occ().get_bit(Squares::B1 as u8)
                    && !self.is_attacked(Squares::E1, !side, board)
                    && !self.is_attacked(Squares::D1, !side, board)
                {
                    self.move_list.push(Move::new(
                        Squares::E1,
                        Squares::C1,
                        WhiteKing,
                        None,
                        MoveType::Castle(CastlingRights::WhiteQueenSide),
                        GenType::Quiet,
                    ));
                }
            }
            Black => {
                // Generate King Side Castle
                if castle.can_castle_king_side(Black)
                    && !board.get_occ().get_bit(Squares::F8 as u8)
                    && !board.get_occ().get_bit(Squares::G8 as u8)
                    && !self.is_attacked(Squares::E8, !side, board)
                    && !self.is_attacked(Squares::F8, !side, board)
                {
                    self.move_list.push(Move::new(
                        Squares::E8,
                        Squares::G8,
                        BlackKing,
                        None,
                        MoveType::Castle(CastlingRights::BlackKingSide),
                        GenType::Quiet,
                    ));
                }

                // Generate Queen Side Castle
                if castle.can_castle_queen_side(Black)
                    && !board.get_occ().get_bit(Squares::D8 as u8)
                    && !board.get_occ().get_bit(Squares::C8 as u8)
                    && !board.get_occ().get_bit(Squares::B8 as u8)
                    && !self.is_attacked(Squares::E8, !side, board)
                    && !self.is_attacked(Squares::D8, !side, board)
                {
                    self.move_list.push(Move::new(
                        Squares::E8,
                        Squares::C8,
                        BlackKing,
                        None,
                        MoveType::Castle(CastlingRights::BlackQueenSide),
                        GenType::Quiet,
                    ));
                }
            }
        }
    }

    pub fn generate_king_moves(&mut self, side: Color, board: &Board) {
        let king = match side {
            White => WhiteKing,
            Black => BlackKing,
        };
        let king_bb = board.get_piece_occ(king);

        for sq in king_bb {
            if sq > 63 {
                continue;
            }
            let source = Squares::from_repr(sq).unwrap();
            let attacks = self.table.get_king_attacks(sq);

            for atk in attacks {
                let target = Squares::from_repr(atk).unwrap();
                let piece = board.get_piece(target);
                if piece.is_some() {
                    if piece.unwrap().get_color() != side {
                        self.move_list.push(Move::new(
                            source,
                            target,
                            king,
                            piece,
                            MoveType::Normal,
                            GenType::Capture,
                        ));
                    }
                } else {
                    self.move_list.push(Move::new(
                        source,
                        target,
                        king,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    ));
                }
            }
        }
    }

    fn generate_knight_moves(&mut self, side: Color, board: &Board) {
        let knight = match side {
            White => WhiteKnight,
            Black => BlackKnight,
        };
        let knigt_bb = board.get_piece_occ(knight);

        for sq in knigt_bb {
            if sq > 63 {
                continue;
            }
            let source = Squares::from_repr(sq).unwrap();
            let attacks = self.table.get_knight_attacks(sq);

            for atk in attacks {
                let target = Squares::from_repr(atk).unwrap();
                let piece = board.get_piece(target);
                if piece.is_some() {
                    if piece.unwrap().get_color() != side {
                        self.move_list.push(Move::new(
                            source,
                            target,
                            knight,
                            piece,
                            MoveType::Normal,
                            GenType::Capture,
                        ));
                    }
                } else {
                    self.move_list.push(Move::new(
                        source,
                        target,
                        knight,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    ));
                }

            }
        }
    }

    pub fn generate_bishop_moves(&mut self, side: Color, board: &Board) {
        let bishop = match side {
            White => WhiteBishop,
            Black => BlackBishop,
        };
        let bishop_bb = board.get_piece_occ(bishop);

        for sq in bishop_bb {
            if sq > 63 {
                continue;
            }
            let source = Squares::from_repr(sq).unwrap();
            let attacks = self.table.get_bishop_attacks(sq, board.get_occ());

            for atk in attacks {
                let target = Squares::from_repr(atk).unwrap();
                let piece = board.get_piece(target);
                if piece.is_some() {
                    if piece.unwrap().get_color() != side {
                        self.move_list.push(Move::new(
                            source,
                            target,
                            bishop,
                            piece,
                            MoveType::Normal,
                            GenType::Capture,
                        ));
                    }
                } else {
                    self.move_list.push(Move::new(
                        source,
                        target,
                        bishop,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    ));
                }
            }
        }
    }

    pub fn generate_rook_moves(&mut self, side: Color, board: &Board) {
        let rook = match side {
            White => WhiteRook,
            Black => BlackRook,
        };
        let rook_bb = board.get_piece_occ(rook);

        for sq in rook_bb {
            if sq > 63 {
                continue;
            }
            let source = Squares::from_repr(sq).unwrap();
            let attacks = self.table.get_rook_attacks(sq, board.get_occ());

            for atk in attacks {
                let target = Squares::from_repr(atk).unwrap();
                let piece = board.get_piece(target);
                if piece.is_some() {
                    if piece.unwrap().get_color() != side {
                        self.move_list.push(Move::new(
                            source,
                            target,
                            rook,
                            piece,
                            MoveType::Normal,
                            GenType::Capture,
                        ));
                    }
                } else {
                    self.move_list.push(Move::new(
                        source,
                        target,
                        rook,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    ));
                }
            }
        }
    }

    pub fn generate_queen_moves(&mut self, side: Color, board: &Board) {
        let queen = match side {
            White => WhiteQueen,
            Black => BlackQueen,
        };
        let queen_bb = board.get_piece_occ(queen);

        for sq in queen_bb {
            if sq > 63 {
                continue;
            }
            let source = Squares::from_repr(sq).unwrap();
            let attacks = self.table.get_queen_attacks(sq, board.get_occ());

            for atk in attacks {
                let target = Squares::from_repr(atk).unwrap();
                let piece = board.get_piece(target);
                if piece.is_some() {
                    if piece.unwrap().get_color() != side {
                        self.move_list.push(Move::new(
                            source,
                            target,
                            queen,
                            piece,
                            MoveType::Normal,
                            GenType::Capture,
                        ));
                    }
                } else {
                    self.move_list.push(Move::new(
                        source,
                        target,
                        queen,
                        None,
                        MoveType::Normal,
                        GenType::Quiet,
                    ));
                }
            }
        }
    }
    pub fn generate_moves(&mut self, side: Color, board: &Board) {
        self.move_list.clear();
        self.generate_pawn_moves(side, board);
        self.generate_castling_moves(side, board);
        self.generate_king_moves(side, board);
        self.generate_knight_moves(side, board);
        self.generate_bishop_moves(side, board);
        self.generate_rook_moves(side, board);
        self.generate_queen_moves(side, board);
        self.generate_queen_moves(side, board);
    }

    pub fn print_attacked(&self, side: Color, board: &Board) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                if self.is_attacked(Squares::from_rank_file(rank, file), side, board) {
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
