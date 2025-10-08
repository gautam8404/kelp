# Quick Implementation Guide

This guide provides code snippets you can use to implement the top optimizations.

## 1. Move Struct Bit-Packing

### Step 1: Define the packed Move struct

```rust
// File: kelp_engine/src/kelp/board/moves.rs

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Move(u32);

impl Move {
    // Bit layout constants
    const FROM_MASK: u32 = 0x0000003F;        // Bits 0-5
    const TO_MASK: u32 = 0x00000FC0;          // Bits 6-11
    const PIECE_MASK: u32 = 0x0000F000;       // Bits 12-15
    const CAPTURE_MASK: u32 = 0x000F0000;     // Bits 16-19
    const MOVETYPE_MASK: u32 = 0x00700000;    // Bits 20-22
    const PROMOTION_MASK: u32 = 0x07800000;   // Bits 23-26
    const GENTYPE_MASK: u32 = 0x08000000;     // Bit 27
    
    const TO_SHIFT: u32 = 6;
    const PIECE_SHIFT: u32 = 12;
    const CAPTURE_SHIFT: u32 = 16;
    const MOVETYPE_SHIFT: u32 = 20;
    const PROMOTION_SHIFT: u32 = 23;
    const GENTYPE_SHIFT: u32 = 27;
    
    const CAPTURE_NONE: u32 = 15;  // Sentinel value for no capture
    
    pub fn new(
        from: Squares,
        to: Squares,
        piece: BoardPiece,
        capture: Option<BoardPiece>,
        move_type: MoveType,
        gen_type: GenType,
    ) -> Self {
        let mut bits = 0u32;
        
        // Pack from square
        bits |= (from as u32) & Self::FROM_MASK;
        
        // Pack to square
        bits |= ((to as u32) << Self::TO_SHIFT) & Self::TO_MASK;
        
        // Pack piece
        bits |= ((piece as u32) << Self::PIECE_SHIFT) & Self::PIECE_MASK;
        
        // Pack capture
        let capture_val = capture.map(|p| p as u32).unwrap_or(Self::CAPTURE_NONE);
        bits |= (capture_val << Self::CAPTURE_SHIFT) & Self::CAPTURE_MASK;
        
        // Pack move type and promotion
        let (type_bits, promo_bits) = Self::encode_move_type(move_type);
        bits |= (type_bits << Self::MOVETYPE_SHIFT) & Self::MOVETYPE_MASK;
        bits |= (promo_bits << Self::PROMOTION_SHIFT) & Self::PROMOTION_MASK;
        
        // Pack gen type
        bits |= ((gen_type as u32) << Self::GENTYPE_SHIFT) & Self::GENTYPE_MASK;
        
        Move(bits)
    }
    
    #[inline(always)]
    pub fn from(&self) -> Squares {
        Squares::from_repr((self.0 & Self::FROM_MASK) as u8).unwrap()
    }
    
    #[inline(always)]
    pub fn to(&self) -> Squares {
        Squares::from_repr(((self.0 & Self::TO_MASK) >> Self::TO_SHIFT) as u8).unwrap()
    }
    
    #[inline(always)]
    pub fn piece(&self) -> BoardPiece {
        BoardPiece::from(((self.0 & Self::PIECE_MASK) >> Self::PIECE_SHIFT) as u8)
    }
    
    #[inline(always)]
    pub fn capture(&self) -> Option<BoardPiece> {
        let capture_val = (self.0 & Self::CAPTURE_MASK) >> Self::CAPTURE_SHIFT;
        if capture_val == Self::CAPTURE_NONE {
            None
        } else {
            Some(BoardPiece::from(capture_val as u8))
        }
    }
    
    #[inline(always)]
    pub fn move_type(&self) -> MoveType {
        let type_bits = (self.0 & Self::MOVETYPE_MASK) >> Self::MOVETYPE_SHIFT;
        let promo_bits = (self.0 & Self::PROMOTION_MASK) >> Self::PROMOTION_SHIFT;
        Self::decode_move_type(type_bits, promo_bits)
    }
    
    #[inline(always)]
    pub fn gen_type(&self) -> GenType {
        if (self.0 & Self::GENTYPE_MASK) != 0 {
            GenType::Capture
        } else {
            GenType::Quiet
        }
    }
    
    fn encode_move_type(move_type: MoveType) -> (u32, u32) {
        match move_type {
            MoveType::Normal => (0, 0),
            MoveType::DoublePawnPush => (1, 0),
            MoveType::EnPassant => (2, 0),
            MoveType::Castle(_) => (3, 0),  // Castle rights stored separately if needed
            MoveType::Promotion(promo) => {
                let promo_val = promo.map(|p| p as u32).unwrap_or(0);
                (4, promo_val)
            }
        }
    }
    
    fn decode_move_type(type_bits: u32, promo_bits: u32) -> MoveType {
        match type_bits {
            0 => MoveType::Normal,
            1 => MoveType::DoublePawnPush,
            2 => MoveType::EnPassant,
            3 => MoveType::Castle(CastlingRights::WhiteKingSide),  // Placeholder
            4 => MoveType::Promotion(Some(BoardPiece::from(promo_bits as u8))),
            _ => MoveType::Normal,
        }
    }
    
    // Convenience methods
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.capture().is_some()
    }
    
    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        matches!(self.move_type(), MoveType::Promotion(_))
    }
    
    #[inline(always)]
    pub fn is_en_passant(&self) -> bool {
        matches!(self.move_type(), MoveType::EnPassant)
    }
    
    #[inline(always)]
    pub fn is_castle(&self) -> bool {
        matches!(self.move_type(), MoveType::Castle(_))
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move({}->{} {} {:?})",
            self.from(),
            self.to(),
            self.piece(),
            self.move_type()
        )
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.move_type() {
            MoveType::Promotion(Some(promotion)) => {
                write!(
                    f,
                    "{}{}{}",
                    self.from(),
                    self.to(),
                    promotion.to_string().to_lowercase()
                )
            }
            _ => write!(f, "{}{}", self.from(), self.to()),
        }
    }
}
```

### Step 2: Update all call sites

Use IDE refactoring to change:
- `mov.from` → `mov.from()`
- `mov.to` → `mov.to()`
- `mov.piece` → `mov.piece()`
- etc.

---

## 2. TranspositionTable Replacement

```rust
// File: kelp_engine/src/kelp/search/transposition.rs

use crate::kelp::board::moves::Move;

const SIZE_MB: usize = 64;
const SIZE_BYTES: usize = SIZE_MB * 1024 * 1024;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum EntryType {
    #[default]
    Exact,
    Alpha,
    Beta,
}

#[repr(C, align(16))]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct TTEntry {
    hash: u64,       // Verification hash
    data: u64,       // Packed: depth(8) | flag(8) | score(32) | generation(16)
    best_move: u32,  // Packed move
}

impl TTEntry {
    pub fn new(hash: u64, depth: u8, flag: EntryType, score: i32, best_move: Option<Move>) -> Self {
        let mut data = 0u64;
        data |= depth as u64;
        data |= (flag as u64) << 8;
        data |= ((score as u32 as u64) << 16);
        
        let move_data = best_move.map(|m| m.0).unwrap_or(0);
        
        TTEntry {
            hash,
            data,
            best_move: move_data,
        }
    }
    
    #[inline(always)]
    pub fn hash(&self) -> u64 {
        self.hash
    }
    
    #[inline(always)]
    pub fn depth(&self) -> u8 {
        (self.data & 0xFF) as u8
    }
    
    #[inline(always)]
    pub fn flag(&self) -> EntryType {
        match (self.data >> 8) & 0xFF {
            0 => EntryType::Exact,
            1 => EntryType::Alpha,
            2 => EntryType::Beta,
            _ => EntryType::Exact,
        }
    }
    
    #[inline(always)]
    pub fn score(&self) -> i32 {
        ((self.data >> 16) & 0xFFFFFFFF) as u32 as i32
    }
    
    #[inline(always)]
    pub fn best_move(&self) -> Option<Move> {
        if self.best_move == 0 {
            None
        } else {
            Some(Move(self.best_move))
        }
    }
}

pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    mask: usize,
    hits: u64,
    misses: u64,
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl TranspositionTable {
    pub fn new() -> Self {
        log::info!("Initializing transposition table");
        
        // Round down to power of 2
        let entry_size = std::mem::size_of::<TTEntry>();
        let num_entries = SIZE_BYTES / entry_size;
        let num_entries = num_entries.next_power_of_two() / 2;  // Round down
        
        log::info!("TT size: {} entries ({:.2} MB)", 
                   num_entries, 
                   (num_entries * entry_size) as f64 / (1024.0 * 1024.0));
        
        TranspositionTable {
            entries: vec![TTEntry::default(); num_entries],
            mask: num_entries - 1,
            hits: 0,
            misses: 0,
        }
    }
    
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            *entry = TTEntry::default();
        }
    }
    
    #[inline(always)]
    pub fn get(&mut self, hash: u64) -> Option<&TTEntry> {
        let index = (hash as usize) & self.mask;
        let entry = &self.entries[index];
        
        if entry.hash == hash {
            self.hits += 1;
            Some(entry)
        } else {
            self.misses += 1;
            None
        }
    }
    
    #[inline(always)]
    pub fn insert(&mut self, hash: u64, depth: u8, flag: EntryType, score: i32, best_move: Option<Move>) {
        let index = (hash as usize) & self.mask;
        let entry = &self.entries[index];
        
        // Replacement strategy: replace if deeper or empty
        if entry.hash == 0 || depth >= entry.depth() {
            self.entries[index] = TTEntry::new(hash, depth, flag, score, best_move);
        }
    }
    
    pub fn get_size(&self) -> usize {
        self.entries.len()
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
        (self.entries.len() * std::mem::size_of::<TTEntry>()) as f64 / (1024.0 * 1024.0)
    }
    
    pub fn get_hash_full_percentage(&self) -> f64 {
        // Sample first 1000 entries
        let sample_size = 1000.min(self.entries.len());
        let filled = self.entries[..sample_size]
            .iter()
            .filter(|e| e.hash != 0)
            .count();
        (filled as f64 / sample_size as f64) * 100.0
    }
}
```

---

## 3. Fix MoveList Cloning

```rust
// File: kelp_engine/src/kelp/mov_gen/generator.rs

impl<'a> MovGen<'a> {
    // BEFORE (inefficient):
    // pub fn get_move_list(&self) -> MoveList {
    //     self.move_list.clone()
    // }
    
    // AFTER (efficient):
    pub fn get_move_list(&self) -> &MoveList {
        &self.move_list
    }
    
    // Or if you need ownership:
    pub fn take_move_list(&mut self) -> MoveList {
        std::mem::replace(&mut self.move_list, MoveList::new())
    }
}
```

---

## 4. LookupTable Array Conversion

```rust
// File: kelp_engine/src/kelp/kelp_core/lookup_table.rs

pub struct LookupTable {
    // BEFORE:
    // pawn_attacks: Vec<Vec<BitBoard>>,
    // knight_attacks: Vec<BitBoard>,
    // king_attacks: Vec<BitBoard>,
    // bishop_attacks: Vec<Vec<BitBoard>>,
    // rook_attacks: Vec<Vec<BitBoard>>,
    
    // AFTER:
    pawn_attacks: [[BitBoard; 64]; 2],
    knight_attacks: [BitBoard; 64],
    king_attacks: [BitBoard; 64],
    bishop_attacks: [[BitBoard; 512]; 64],
    rook_attacks: [[BitBoard; 4096]; 64],
    magic_table: MagicTable,
}

impl LookupTable {
    pub fn new() -> LookupTable {
        LookupTable {
            pawn_attacks: [[BitBoard(0); 64]; 2],
            knight_attacks: [BitBoard(0); 64],
            king_attacks: [BitBoard(0); 64],
            bishop_attacks: [[BitBoard(0); 512]; 64],
            rook_attacks: [[BitBoard(0); 4096]; 64],
            magic_table: MagicTable::new(),
        }
    }
    
    // All other methods stay the same!
}
```

---

## 5. Testing After Changes

### Test 1: Correctness
```bash
cargo test
```

### Test 2: Perft (move generation)
```bash
cargo run --release --bin kelp_perft
```

### Test 3: Size verification
```rust
#[test]
fn test_move_size() {
    assert_eq!(std::mem::size_of::<Move>(), 4);
}

#[test]
fn test_tt_entry_size() {
    assert_eq!(std::mem::size_of::<TTEntry>(), 24);
}
```

### Test 4: Benchmark
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_move_pack_unpack(c: &mut Criterion) {
    c.bench_function("move_creation", |b| {
        b.iter(|| {
            let m = Move::new(
                black_box(Squares::E2),
                black_box(Squares::E4),
                black_box(BoardPiece::WhitePawn),
                None,
                MoveType::DoublePawnPush,
                GenType::Quiet,
            );
            black_box(m);
        })
    });
    
    let m = Move::new(
        Squares::E2,
        Squares::E4,
        BoardPiece::WhitePawn,
        None,
        MoveType::DoublePawnPush,
        GenType::Quiet,
    );
    
    c.bench_function("move_access", |b| {
        b.iter(|| {
            black_box(m.from());
            black_box(m.to());
            black_box(m.piece());
        })
    });
}
```

---

## Migration Checklist

- [ ] Create feature branch
- [ ] Implement Move bit-packing
- [ ] Update all Move field accesses (`.from` → `.from()`)
- [ ] Run tests to verify correctness
- [ ] Implement TranspositionTable replacement
- [ ] Update TT usage in negamax
- [ ] Run tests again
- [ ] Fix MoveList cloning
- [ ] Convert LookupTable to arrays
- [ ] Run full test suite
- [ ] Run perft validation
- [ ] Benchmark before/after
- [ ] Merge to main

---

## Expected Issues & Solutions

### Issue 1: "field `from` is private"
**Solution**: Change `mov.from` to `mov.from()` everywhere

### Issue 2: "move occurs because `mov` has type `Move`, which does not implement the `Copy` trait"
**Solution**: Move is still Copy, but might need to derive traits explicitly

### Issue 3: TT insert signature changed
**Solution**: Update negamax.rs to use new signature:
```rust
// BEFORE:
// tt.insert(hash, Entry { hash, depth, flag, score, best_move });

// AFTER:
tt.insert(hash, depth, flag, score, best_move);
```

---

## Performance Validation

After implementing, measure:

1. **Memory usage**: `ps aux` or Activity Monitor
2. **Move generation speed**: perft benchmark
3. **Search speed**: tactical test suite
4. **Overall strength**: self-play or Lichess

Expected improvements:
- 40-50% less memory
- 2-3x faster search
- Same or slightly better playing strength

---

Good luck with the implementation! Start with the easy ones (MoveList cloning, LookupTable arrays) to build confidence before tackling Move bit-packing.
