use crate::kelp::board::moves::Move;
use std::collections::HashMap;

const SIZE_MB: usize = 64;
const SIZE_BYTES: usize = SIZE_MB * 1024 * 1024;
const BYTES_PER_ENTRY: usize = std::mem::size_of::<Entry>();
const BYTES_PER_KB: usize = 1024;
const BYTES_PER_MB: usize = BYTES_PER_KB * 1024;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum EntryType {
    #[default]
    Exact,
    Alpha,
    Beta,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Entry {
    pub hash: u64,
    pub depth: u8,
    pub flag: EntryType,
    pub score: i32,
    pub best_move: Option<Move>,
}
#[derive(Debug, Default)]
pub struct TranspositionTable {
    table: HashMap<u64, Entry>,
    size: usize,
    hits: u64,
    misses: u64,
}

impl TranspositionTable {
    pub fn new() -> Self {
        log::info!("Initializing transposition table");
        let num_entries = SIZE_BYTES / std::mem::size_of::<Entry>();
        TranspositionTable {
            table: HashMap::with_capacity(num_entries),
            size: num_entries,
            hits: 0,
            misses: 0,
        }
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    #[inline(always)]
    pub fn get(&mut self, hash: u64) -> Option<&Entry> {
        let entry = self.table.get(&hash);
        if entry.is_some() {
            if entry.unwrap().hash == hash {
                self.hits += 1;
                return entry;
            }
        } else {
            self.misses += 1;
        }
        None
    }

    #[inline(always)]
    pub fn insert(&mut self, hash: u64, entry: Entry) {
        if self.table.len() >= self.size {
            self.table.clear(); // idk if this is the best way to do this
        }
        self.table.insert(hash, entry);
    }

    pub fn get_size(&self) -> usize {
        self.table.len()
    }

    pub fn get_hits(&self) -> u64 {
        self.hits
    }

    pub fn get_misses(&self) -> u64 {
        self.misses
    }

    pub fn reset_hits_and_misses(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }

    pub fn get_hashmap_size_mb(&self) -> f64 {
        self.table.len() as f64 * BYTES_PER_ENTRY as f64 / BYTES_PER_MB as f64
    }

    // TODO: remove this function
    pub fn get_total_entries(&self) -> usize {
        self.table.len()
    }

    pub fn get_hash_full_percentage(&self) -> f64 {
        self.table.len() as f64 / self.size as f64 * 100.0
    }
}
