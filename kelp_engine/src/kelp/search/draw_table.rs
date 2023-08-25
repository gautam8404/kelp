use crate::kelp::ZobristKey;
const MAX_DRAW_TABLE_SIZE: usize = 128;

pub struct DrawTable {
    index: usize,
    table: [ZobristKey; MAX_DRAW_TABLE_SIZE],
}

impl DrawTable {
    pub fn new() -> Self {
        Self {
            index: 0,
            table: [0; MAX_DRAW_TABLE_SIZE],
        }
    }

    #[inline(always)]
    pub fn push(&mut self, key: ZobristKey) {
        self.table[self.index] = key;
        self.index += 1;
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<ZobristKey> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;
        Some(self.table[self.index])
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.index = 0;
    }

    #[inline(always)]
    pub fn is_repeat(&self, key: ZobristKey) -> bool {
        for i in 0..self.index {
            if self.table[i] == key {
                return true;
            }
        }

        false
    }
}
