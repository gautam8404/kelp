mod kelp;
use kelp::board::bitboard::BitBoard;

fn main() {
    let board = BitBoard(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001);
    println!("{}", board);
}
