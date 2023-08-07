use super::super::{board::bitboard::BitBoard, BitBoardArray, Move, MoveType, board::piece::{Color, BoardPiece}, CastlingRights, BoardInfo};
use super::fen::Fen;


pub struct Board {
    pub bitboards: BitBoardArray,
    pub info: BoardInfo,
}

impl From<Fen> for Board {
    fn from(fen: Fen) -> Self {
        let mut bitboards: BitBoardArray = [BitBoard::empty(); 12];
        let mut index = 0;

        todo!("Implement From<Fen> for Board")
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboards: [BitBoard::empty(); 12],
            info: BoardInfo {
                turn: Color::White,
                castling_rights: CastlingRights::WhiteKingSide | CastlingRights::WhiteQueenSide | CastlingRights::BlackKingSide | CastlingRights::BlackQueenSide,
                en_passant: None,
                halfmove_clock: 0,
                fullmove_number: 1,
            },
        }
    }
}