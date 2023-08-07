use std::fmt::Display;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Mul, MulAssign, Not, Shl, ShlAssign, Shr, ShrAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    #[inline(always)]
    pub fn get_bit(&self, index: u8) -> bool {
        ((self.0 >> index) & 1) == 1
    }

    #[inline(always)]
    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    #[inline(always)]
    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    #[inline(always)]
    pub fn toggle_bit(&mut self, index: u8) {
        self.0 ^= 1 << index;
    }

    #[inline(always)]
    pub fn count_bits(&self) -> u8 {
        self.0.count_ones() as u8
    }

    #[inline(always)]
    pub fn get_lsb(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    #[inline(always)]
    pub fn get_msb(&self) -> u8 {
        self.0.leading_zeros() as u8
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> u8 {
        let lsb = self.get_lsb();
        self.clear_bit(lsb);
        lsb
    }
    #[inline(always)]
    pub fn pop_msb(&mut self) -> u8 {
        let msb = self.get_msb();
        self.clear_bit(msb);
        msb
    }

    pub fn empty() -> BitBoard {
        BitBoard(0)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.get_bit(square as u8) {
                    board.push_str("1  ");
                } else {
                    board.push_str("0  ");
                }
            }
            board.push_str(&format!("\t{} ", rank + 1));
            board.push_str("\n");
        }

        board.push_str("\n\na  b  c  d  e  f  g  h\n");
        board.push_str(&format!("BitBoard: {}", self.0));
        write!(f, "{}", board)
    }
}

impl Mul for BitBoard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.wrapping_mul(rhs.0))
    }
}

impl MulAssign for BitBoard {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_mul(rhs.0);
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Shr<BitBoard> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 >> rhs.0)
    }
}

impl Shr<u8> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl ShrAssign<BitBoard> for BitBoard {
    fn shr_assign(&mut self, rhs: BitBoard) {
        self.0 >>= rhs.0;
    }
}

impl ShrAssign<u8> for BitBoard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

impl Shl<BitBoard> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 << rhs.0)
    }
}

impl Shl<u8> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl ShlAssign<BitBoard> for BitBoard {
    fn shl_assign(&mut self, rhs: BitBoard) {
        self.0 <<= rhs.0;
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard(value)
    }
}

impl From<BitBoard> for u64 {
    fn from(value: BitBoard) -> Self {
        value.0
    }
}
