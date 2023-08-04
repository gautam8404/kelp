use std::fmt::Display;

#[derive(Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn get_bit(&self, index: u8) -> bool {
        let bit = self.0 & (1 << index);
        bit != 0
    }

    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub fn toggle_bit(&mut self, index: u8) {
        self.0 ^= 1 << index;
    }

    pub fn count_bits(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn get_lsb(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    pub fn get_msb(&self) -> u8 {
        self.0.leading_zeros() as u8
    }

    pub fn pop_lsb(&mut self) -> u8 {
        let lsb = self.get_lsb();
        self.clear_bit(lsb);
        lsb
    }

    pub fn pop_msb(&mut self) -> u8 {
        let msb = self.get_msb();
        self.clear_bit(msb);
        msb
    }
}

impl Display for BitBoard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        let mut rank = 9;

        for i in 0..64 {
            if i % 8 == 0 {
                if rank != 9 {
                    board.push_str(&format!("\t{} ", rank));
                }
                rank -= 1;
                board.push_str("\n");
            }
            let bit = self.0 & (1 << i);
            if bit == 0 {
                board.push_str("0  ");
            } else {
                board.push_str("1  ");
            }

        }
        board.push_str(&format!("\t{} ", rank));

        board.push_str("\n\na  b  c  d  e  f  g  h\n");
        write!(f, "{}", board)
    }
}