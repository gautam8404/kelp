use super::fen::Fen;
use super::fen::{FenParse, FenParseError};
use super::moves::{Castle, CastlingRights};
use super::piece::{
    BoardPiece::{self, *},
    Color,
};
use super::zobrist::Zobrist;
use crate::kelp::board::moves::{Move, MoveArray, MoveHistory, MoveType};
use crate::kelp::board::piece::Color::*;
use crate::kelp::mov_gen::generator::MovGen;
use crate::kelp::Squares::{self, *};
use crate::kelp::{kelp_core::bitboard::BitBoard, BitBoardArray, BoardInfo, GamePhase, GameState};

// strum
use std::fmt::{Debug, Display};
use std::str::FromStr;
use strum::IntoEnumIterator;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Eq, PartialEq)]
pub struct Board {
    pub bitboards: BitBoardArray,
    pub hash: u64,
    pub state: GameState,
    pub phase: GamePhase,
    pub info: BoardInfo,
    pub move_history: MoveArray,
    zobrist: Zobrist,
}

impl Default for Board {
    fn default() -> Self {
        let board = Board::parse(Fen(STARTPOS.to_string())).unwrap();
        board
    }
}

impl Board {
    #[inline(always)]
    pub fn add_to_bb(&mut self, piece: BoardPiece, square: Squares) {
        self.bitboards[piece as usize].set_bit(square as u8);
    }

    #[inline(always)]
    pub fn remove_from_bb(&mut self, piece: BoardPiece, square: Squares) {
        self.bitboards[piece as usize].clear_bit(square as u8);
    }

    #[allow(dead_code)]
    pub fn replace_piece(&mut self, piece: BoardPiece, square: Squares) {
        for p in BoardPiece::iter() {
            self.remove_piece(p, square);
        }
        self.add_piece(piece, square);
    }

    #[inline(always)]
    pub fn add_piece(&mut self, piece: BoardPiece, square: Squares) {
        self.add_to_bb(piece, square);
        self.hash ^= self.zobrist.get_piece_key(piece, square);
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, piece: BoardPiece, square: Squares) {
        self.remove_from_bb(piece, square);
        self.hash ^= self.zobrist.get_piece_key(piece, square);
    }

    #[inline(always)]
    pub fn move_piece(&mut self, piece: BoardPiece, from: Squares, to: Squares) {
        self.remove_piece(piece, from);
        self.add_piece(piece, to);
    }

    #[inline(always)]
    pub fn get_piece(&self, square: Squares) -> Option<BoardPiece> {
        BoardPiece::iter().find(|&piece| self.bitboards[piece as usize].get_bit(square as u8))
    }

    #[inline(always)]
    pub fn get_king_square(&self, color: Color) -> Squares {
        let king = match color {
            White => WhiteKing,
            Black => BlackKing,
        };

        let sq = Squares::from_repr(self.bitboards[king as usize].get_lsb());
        if sq.is_none() {
            panic!("No king found for color {:?}", color);
        }
        sq.unwrap()
    }

    #[inline(always)]
    pub fn is_king_checked(&self, color: Color, gen: &MovGen) -> bool {
        let king = self.get_king_square(color);
        gen.is_attacked(king, !color, self)
    }

    #[inline(always)]
    pub fn get_piece_occ(&self, piece: BoardPiece) -> BitBoard {
        self.bitboards[piece as usize]
    }

    #[inline(always)]
    pub fn get_white_occ(&self) -> BitBoard {
        let mut out = BitBoard::empty();
        out = self.bitboards[WhitePawn as usize]
            | self.bitboards[WhiteKnight as usize]
            | self.bitboards[WhiteBishop as usize]
            | self.bitboards[WhiteRook as usize]
            | self.bitboards[WhiteQueen as usize]
            | self.bitboards[WhiteKing as usize];
        out
    }

    #[inline(always)]
    pub fn get_black_occ(&self) -> BitBoard {
        let mut out = BitBoard::empty();
        out = self.bitboards[BlackPawn as usize]
            | self.bitboards[BlackKnight as usize]
            | self.bitboards[BlackBishop as usize]
            | self.bitboards[BlackRook as usize]
            | self.bitboards[BlackQueen as usize]
            | self.bitboards[BlackKing as usize];
        out
    }

    #[inline(always)]
    pub fn get_occ(&self) -> BitBoard {
        self.get_white_occ() | self.get_black_occ()
    }

    #[inline(always)]
    pub fn get_en_passant(&self) -> Option<Squares> {
        self.info.en_passant
    }

    #[inline(always)]
    pub fn set_en_passant(&mut self, square: Squares) {
        if self.info.en_passant.is_some() {
            self.hash ^= self
                .zobrist
                .get_en_passant_key(self.info.en_passant.unwrap());
        }
        self.info.en_passant = Some(square);
        self.hash ^= self.zobrist.get_en_passant_key(square);
    }

    #[inline(always)]
    pub fn clear_en_passant(&mut self) {
        if self.info.en_passant.is_some() {
            self.hash ^= self
                .zobrist
                .get_en_passant_key(self.info.en_passant.unwrap());
        }
        self.info.en_passant = None;
    }

    pub fn to_fen(&self) -> String {
        let fen = Fen::parse(self);
        if fen.is_err() {
            return String::from("Error parsing fen");
        }
        fen.unwrap().to_string()
    }

    pub fn toggle_turn(&mut self) {
        self.info.turn = !self.info.turn;
        self.hash ^= self.zobrist.get_side_key();
    }

    #[inline(always)]
    pub fn get_side_to_move(&self) -> Color {
        self.info.turn
    }

    #[inline(always)]
    pub fn is_check(&self, gen: &MovGen) -> bool {
        self.is_king_checked(self.info.turn, gen)
    }

    #[inline(always)]
    pub fn is_check_opp(&self, gen: &MovGen) -> bool {
        self.is_king_checked(!self.info.turn, gen)
    }

    #[inline(always)]
    pub fn get_bitboard(&self, piece: BoardPiece) -> BitBoard {
        self.bitboards[piece as usize]
    }

    #[inline(always)]
    pub fn update_hash(&mut self) {
        self.hash = self.zobrist.get_key(self);
    }

    #[allow(dead_code)]
    pub fn generated_hash(&self) -> u64 {
        self.zobrist.get_key(self)
    }

    pub fn is_fifty_move_draw(&self) -> bool {
        self.info.halfmove_clock >= 100
    }

}

// Make / Unmake move and helper functions
impl Board {
    #[inline(always)]
    fn make_normal(&mut self, mov: Move) {
        if mov.capture.is_some() {
            self.remove_piece(mov.capture.unwrap(), mov.to);
        }
        self.move_piece(mov.piece, mov.from, mov.to);
    }

    #[inline(always)]
    fn make_double_pawn(&mut self, mov: Move) {
        self.move_piece(mov.piece, mov.from, mov.to);
        let color = mov.piece.get_color();
        let en_passant = match color {
            White => mov.to - 8,
            Black => mov.to + 8,
        };
        self.set_en_passant(en_passant);
    }

    #[inline(always)]
    fn make_en_passant(&mut self, mov: Move) {
        self.move_piece(mov.piece, mov.from, mov.to);
        let capture = match mov.piece.get_color() {
            White => mov.to - 8,
            Black => mov.to + 8,
        };
        self.remove_piece(mov.capture.unwrap(), capture);
    }
    #[inline(always)]
    fn make_promotion(&mut self, mov: Move, promoted_to: BoardPiece) {
        if mov.capture.is_some() {
            self.remove_piece(mov.capture.unwrap(), mov.to);
        }
        self.remove_piece(mov.piece, mov.from);
        self.add_piece(promoted_to, mov.to);
    }

    #[inline(always)]
    fn make_castle(&mut self, mov: Move, castle: CastlingRights) {
        let color = mov.piece.get_color();
        let (king_from, king_to, rook_from, rook_to) = match castle {
            CastlingRights::WhiteKingSide => (E1, G1, H1, F1),
            CastlingRights::WhiteQueenSide => (E1, C1, A1, D1),
            CastlingRights::BlackKingSide => (E8, G8, H8, F8),
            CastlingRights::BlackQueenSide => (E8, C8, A8, D8),
        };
        let rook = match color {
            White => WhiteRook,
            Black => BlackRook,
        };
        self.move_piece(mov.piece, king_from, king_to);
        self.move_piece(rook, rook_from, rook_to);
        self.info.castle.remove(castle);
    }

    #[inline(always)]
    fn unmake_normal(&mut self, mov: Move) {
        self.move_piece(mov.piece, mov.to, mov.from);
        if mov.capture.is_some() {
            self.add_piece(mov.capture.unwrap(), mov.to);
        }
    }

    #[inline(always)]
    fn unmake_double_pawn(&mut self, mov: Move) {
        self.move_piece(mov.piece, mov.to, mov.from);
    }

    #[inline(always)]
    fn unmake_en_passant(&mut self, mov: Move) {
        self.move_piece(mov.piece, mov.to, mov.from);
        let capture = match mov.piece.get_color() {
            White => mov.to - 8,
            Black => mov.to + 8,
        };
        self.add_piece(mov.capture.unwrap(), capture);
    }

    #[inline(always)]
    fn unmake_promotion(&mut self, mov: Move, promoted_to: BoardPiece) {
        self.add_piece(mov.piece, mov.from);
        self.remove_piece(promoted_to, mov.to);
        if mov.capture.is_some() {
            self.add_piece(mov.capture.unwrap(), mov.to);
        }
    }

    #[inline(always)]
    fn unmake_castle(&mut self, mov: Move, castle: CastlingRights) {
        let color = mov.piece.get_color();
        let (king_from, king_to, rook_from, rook_to) = match castle {
            CastlingRights::WhiteKingSide => (E1, G1, H1, F1),
            CastlingRights::WhiteQueenSide => (E1, C1, A1, D1),
            CastlingRights::BlackKingSide => (E8, G8, H8, F8),
            CastlingRights::BlackQueenSide => (E8, C8, A8, D8),
        };
        let rook = match color {
            White => WhiteRook,
            Black => BlackRook,
        };
        self.move_piece(mov.piece, king_to, king_from);
        self.move_piece(rook, rook_to, rook_from);
        self.info.castle.add(castle);
    }

    // makes move and pushes it to history also logs it, shouldn't be used for search or perft testing because of performance overhead by logging
    #[inline(always)]
    pub fn make(&mut self, mov: Move) {
        let history = self.make_move(mov, false);
        if history.is_some() {
            // log::info!("pushing {:?} to history", history.unwrap());
            self.move_history.push(history.unwrap());
        }
    }

    // unmake move from history
    #[inline(always)]
    pub fn unmake(&mut self) {
        let mov = self.move_history.pop();
        if mov.is_none() {
            return;
        }
        // log::info!("unmaking {:?} from history", mov.unwrap());
        self.unmake_move(mov.unwrap());
    }

    #[inline(always)]
    pub fn make_move(&mut self, mov: Move, only_captures: bool) -> Option<MoveHistory> {
        if only_captures && mov.capture.is_none() {
            return None;
        }

        use BoardPiece::*;
        use CastlingRights::*;
        use Color::*;
        use MoveType::*;

        let old_en_passant = self.info.en_passant;
        let old_castle = self.info.castle;
        let old_half_move_clock = self.info.halfmove_clock;
        let old_hash = self.hash;

        match mov.move_type {
            Normal => {
                self.make_normal(mov);
            }
            DoublePawnPush => {
                self.make_double_pawn(mov);
            }

            EnPassant => {
                self.make_en_passant(mov);
            }

            Promotion(promoted_to) => {
                self.make_promotion(mov, promoted_to.unwrap());
            }

            Castle(castle) => {
                self.make_castle(mov, castle);
            }
        };

        // State Updates

        // Update fullmove number
        if self.info.turn == Black {
            self.info.fullmove_clock += 1;
        }

        // Update turn
        self.toggle_turn();

        // Update halfmove clock
        if mov.capture.is_some() || mov.piece == WhitePawn || mov.piece == BlackPawn {
            self.info.halfmove_clock = 0;
        } else {
            self.info.halfmove_clock += 1;
        }

        // Update en passant
        if mov.move_type != DoublePawnPush {
            self.clear_en_passant();
        }

        // Update castling rights
        if mov.from == A1 || mov.to == A1 {
            self.info.castle.remove(WhiteQueenSide);
        } else if mov.from == H1 || mov.to == H1 {
            self.info.castle.remove(WhiteKingSide);
        } else if mov.from == A8 || mov.to == A8 {
            self.info.castle.remove(BlackQueenSide);
        } else if mov.from == H8 || mov.to == H8 {
            self.info.castle.remove(BlackKingSide);
        } else if mov.from == E1 || mov.to == E1 {
            self.info.castle.remove(WhiteKingSide);
            self.info.castle.remove(WhiteQueenSide);
        } else if mov.from == E8 || mov.to == E8 {
            self.info.castle.remove(BlackKingSide);
            self.info.castle.remove(BlackQueenSide);
        }

        // Update Castle hash
        self.hash ^= self.zobrist.get_castle_key(old_castle.0);
        self.hash ^= self.zobrist.get_castle_key(self.info.castle.0);

        Some(MoveHistory {
            mov: mov,
            castle_rights: old_castle,
            en_passant: old_en_passant,
            hash: old_hash,
            half_move_clock: old_half_move_clock,
        })
    }

    #[inline(always)]
    pub fn unmake_move(&mut self, history: MoveHistory) {
        let color = history.mov.piece.get_color();
        let mov = history.mov;

        use MoveType::*;
        match mov.move_type {
            Normal => {
                self.unmake_normal(mov);
            }
            DoublePawnPush => {
                self.unmake_double_pawn(mov);
            }
            EnPassant => {
                self.unmake_en_passant(mov);
            }
            Promotion(promoted_to) => {
                self.unmake_promotion(mov, promoted_to.unwrap());
            }
            Castle(castle) => {
                self.unmake_castle(mov, castle);
            }
        };

        self.info.turn = !self.info.turn;
        self.info.castle = history.castle_rights;
        self.info.en_passant = history.en_passant;
        self.info.halfmove_clock = history.half_move_clock;
        self.hash = history.hash;
        if color == Black && self.info.fullmove_clock > 1 {
            self.info.fullmove_clock -= 1;
        }

        // update hash in draw table
        // self.remove_repetition();
    }

    #[inline(always)]
    pub fn make_null_move(&mut self) -> (Option<Squares>, u64) {
        let mut old_en_passant = self.info.en_passant;
        let old_hash = self.hash;
        self.clear_en_passant();
        self.toggle_turn();

        // self.add_repetition();

        (old_en_passant, old_hash)
    }

    #[inline(always)]
    pub fn unmake_null_move(&mut self, en_passant: Option<Squares>, hash: u64) {
        if en_passant.is_some() {
            self.set_en_passant(en_passant.unwrap());
        }
        self.toggle_turn();
        self.hash = hash;

        // self.remove_repetition();
    }
}

// Trait implementations

impl FenParse<Fen, Board, FenParseError> for Board {
    fn parse(fen: Fen) -> Result<Board, FenParseError> {
        let mut bitboards: BitBoardArray = [BitBoard::empty(); 12];

        let is_valid = fen.is_valid();
        match is_valid {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let parts: Vec<&str> = fen.0.split_whitespace().collect::<Vec<&str>>();
        let mut board = parts[0].split("/");
        let mut number_of_pieces = 0;

        for rank in (0..8).rev() {
            let mut file = 0;
            for c in board.next().unwrap().chars() {
                if c.is_alphabetic() {
                    let piece = BoardPiece::from_str(c.to_string().as_str());
                    if piece.is_err() {
                        return Err(FenParseError::InvalidPiece(format!("Invalid piece: {}", c)));
                    }
                    let piece = piece.unwrap();
                    bitboards[piece as usize].set_bit(rank * 8 + file);
                    number_of_pieces += 1;
                    file += 1;
                } else if c.is_numeric() {
                    file += (c.to_digit(10).unwrap()) as u8;
                }
            }
        }

        let turn = match parts[1] {
            "w" => White,
            "b" => Black,
            _ => {
                return Err(FenParseError::InvalidTurn(format!(
                    "Invalid turn: {}, must be 'w' or 'b'",
                    parts[1]
                )))
            }
        };

        let mut castling_rights: Castle = Castle(0);
        for c in parts[2].chars() {
            match c {
                'K' => castling_rights.add(CastlingRights::WhiteKingSide),
                'Q' => castling_rights.add(CastlingRights::WhiteQueenSide),
                'k' => castling_rights.add(CastlingRights::BlackKingSide),
                'q' => castling_rights.add(CastlingRights::BlackQueenSide),
                '-' => {}
                _ => {
                    return Err(FenParseError::InvalidCastlingRights(format!(
                        "Invalid castling rights: {}, must be 'K', 'Q', 'k', 'q', or '-'",
                        parts[2]
                    )))
                }
            }
        }

        let en_passant = match parts[3] {
            "-" => None,
            _ => Some({
                let enp = Squares::from_str(parts[3]);
                if enp.is_err() {
                    return Err(FenParseError::InvalidEnPassant(format!(
                        "Invalid en passant square: {}",
                        parts[3]
                    )));
                }
                enp.unwrap()
            }),
        };

        let mut halfmove_clock = 0;
        let mut fullmove_clock = 0;

        if parts.len() > 4 {
            halfmove_clock = parts[4].parse::<u8>().unwrap();
            fullmove_clock = parts[5].parse::<u16>().unwrap();
        }

        let game_phase = {
            if number_of_pieces <= 12 {
                GamePhase::EndGame
            } else if number_of_pieces <= 24 {
                GamePhase::MiddleGame
            } else {
                GamePhase::Opening
            }
        };

        let game_state = {
            if game_phase == GamePhase::EndGame {
                if halfmove_clock >= 100 {
                    GameState::Draw
                } else {
                    GameState::Playing
                }
            } else {
                GameState::Playing
            }
        };

        let mut board = Board {
            bitboards,
            hash: 0, // TODO
            state: game_state,
            phase: game_phase,
            move_history: MoveArray::new(),
            info: BoardInfo {
                turn,
                castle: castling_rights,
                en_passant,
                halfmove_clock,
                fullmove_clock,
            },
            zobrist: Zobrist::new(),
        };

        board.update_hash();

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        board.push_str("+---+---+---+---+---+---+---+---+\n");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mut piece: Option<BoardPiece> = None;
                for i in 0..12 {
                    if self.bitboards[i].get_bit(rank * 8 + file) {
                        piece = Some(BoardPiece::from(i as u8));
                        break;
                    }
                }
                board.push_str(&format!(
                    "| {} ",
                    match piece {
                        Some(p) => p.to_string(),
                        None => " ".to_string(),
                    }
                ));
            }
            board.push_str(&format!("| {}\n", rank + 1));
            board.push_str("+---+---+---+---+---+---+---+---+\n");
        }
        board.push_str("  a   b   c   d   e   f   g   h\n\n");
        board.push_str(&format!("Fen: {}\n", self.to_fen()));
        board.push_str(&format!("Key: {:016X}\n", self.hash));
        write!(f, "{}", board)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        board.push_str("+---+---+---+---+---+---+---+---+\n");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mut piece: Option<BoardPiece> = None;
                for i in 0..12 {
                    if self.bitboards[i].get_bit(rank * 8 + file) {
                        piece = Some(BoardPiece::from(i as u8));
                        break;
                    }
                }
                board.push_str(&format!(
                    "| {} ",
                    match piece {
                        Some(p) => p.to_string(),
                        None => " ".to_string(),
                    }
                ));
            }
            board.push_str(&format!("| {}\n", rank + 1));
            board.push_str("+---+---+---+---+---+---+---+---+\n");
        }

        board.push_str("  a   b   c   d   e   f   g   h\n\n");
        let fen = self.to_fen();
        board.push_str(format!("Fen: {}\n", fen).as_str());
        board.push_str(format!("Hash: {:016X}\n", self.hash).as_str());
        board.push_str(format!("State: {:?}\n", self.state).as_str());
        board.push_str(format!("Phase: {:?}\n", self.phase).as_str());
        let board_info = format!("{:?}", self.info);
        board.push_str(&board_info);
        write!(f, "{}", board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kelp::kelp_core::lookup_table::LookupTable;

    #[test]
    fn fen_to_board() {
        let FEN = Fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ".to_string(),
        );
        let ERR_FEN = Fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq a9 0 1 ".to_string(),
        ); // invalid en passant square

        let board = Board::parse(FEN);
        assert!(board.is_ok());
        let board = Board::parse(ERR_FEN);
        assert!(board.is_err());
    }

    #[test]
    fn board_to_fen() {
        use crate::kelp::kelp_core::bitboard::BitBoard;
        let mut board = Board::default();
        let test_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

        board.bitboards[0] = BitBoard(34628232960);
        board.bitboards[1] = BitBoard(68719738880);
        board.bitboards[2] = BitBoard(6144);
        board.bitboards[3] = BitBoard(129);
        board.bitboards[4] = BitBoard(2097152);
        board.bitboards[5] = BitBoard(16);
        board.bitboards[6] = BitBoard(12754334924144640);
        board.bitboards[7] = BitBoard(37383395344384);
        board.bitboards[8] = BitBoard(18015498021109760);
        board.bitboards[9] = BitBoard(9295429630892703744);
        board.bitboards[10] = BitBoard(4503599627370496);
        board.bitboards[11] = BitBoard(1152921504606846976);

        let fen = board.to_fen();
        assert_eq!(fen, test_fen);

        let fen = Fen::parse(&board);
        assert!(fen.is_ok());
        assert_eq!(fen.unwrap().to_string(), test_fen);
    }

    fn incremental_zobrist_test_driver(depth: u16, board: &mut Board, gen: &mut MovGen) {
        if depth == 0 {
            return;
        }
        gen.generate_moves(board);
        let move_list = gen.get_move_list();

        for mov in move_list.iter() {
            let history = board.make_move(*mov, false);
            if history.is_none() {
                continue;
            }
            let hash = board.generated_hash();
            assert_eq!(hash, board.hash);
            incremental_zobrist_test_driver(depth - 1, board, gen);
            board.unmake_move(history.unwrap());
            let hash = board.generated_hash();
            assert_eq!(hash, board.hash);
        }
    }

    #[test]
    fn incremental_zobrist_test() {
        let tricky_fen =
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ".to_string();
        let promotion_fen =
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ".to_string();

        let mut table = LookupTable::new();
        table.populate();
        let mut gen = MovGen::new(&table);

        let mut board = Board::parse(Fen(tricky_fen)).unwrap();
        incremental_zobrist_test_driver(3, &mut board, &mut gen);

        let mut board = Board::parse(Fen(promotion_fen)).unwrap();
        incremental_zobrist_test_driver(3, &mut board, &mut gen);
    }
}
