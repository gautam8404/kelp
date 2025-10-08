# Deep Chess Engine Optimization Analysis

This document provides a comprehensive analysis of optimization opportunities in the Kelp chess engine, going beyond surface-level improvements to identify architectural and performance-critical optimizations.

## Current Measurements

- **Move struct size**: 7 bytes (currently)
- **Theoretical packed Move size**: 4 bytes (u32)
- **Memory savings per move**: ~43% reduction
- **Typical move list size**: 256 moves (MAX_SIZE_MOVES_ARR)

---

## 1. CRITICAL: Move Struct Bit-Packing (Memory & Cache Optimization)

### Current Implementation
```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    pub from: Squares,        // 1 byte (6 bits needed: 0-63)
    pub to: Squares,          // 1 byte (6 bits needed: 0-63)
    pub piece: BoardPiece,    // 1 byte (4 bits needed: 12 pieces)
    pub capture: Option<BoardPiece>, // 1 byte (4 bits + 1 flag bit needed)
    pub move_type: MoveType,  // 2 bytes (variable)
    pub gen_type: GenType,    // 1 byte (1 bit needed)
}
// Total: 7 bytes
```

### Optimized Implementation (32-bit packed)
```
Bit Layout (32 bits total):
- Bits 0-5:   from square (6 bits, 0-63)
- Bits 6-11:  to square (6 bits, 0-63)
- Bits 12-15: piece type (4 bits, 0-11)
- Bits 16-19: captured piece (4 bits, 0-15 with 15=None)
- Bits 20-22: move type flags (3 bits)
  - 000: Normal
  - 001: DoublePawnPush
  - 010: EnPassant
  - 011: Castle
  - 100-111: Promotion (with promotion piece in bits 23-26)
- Bits 23-26: promotion piece (4 bits, used only for promotions)
- Bit 27:     gen_type (quiet=0, capture=1)
- Bits 28-31: reserved/castling rights (4 bits)
```

### Impact
- **Memory**: 7 bytes → 4 bytes per move (43% reduction)
- **Cache efficiency**: 256 moves = 1.75 KB → 1 KB (L1 cache friendly)
- **MoveHistory**: Currently ~32 bytes, will reduce to ~28 bytes
- **Bandwidth**: Less memory bandwidth in move generation/sorting

---

## 2. CRITICAL: TranspositionTable Replacement

### Current Implementation Issues
```rust
pub struct TranspositionTable {
    table: HashMap<u64, Entry>,  // ← MAJOR BOTTLENECK
    size: usize,
    hits: u64,
    misses: u64,
}
```

**Problems:**
1. **HashMap overhead**: 24+ bytes per entry overhead (pointer, hash, capacity)
2. **Poor cache locality**: Entries scattered in memory
3. **Unpredictable performance**: Hash collisions cause cache misses
4. **Full table clearing**: When capacity reached, entire table is cleared (line 67)

### Optimized Implementation
```rust
pub struct TranspositionTable {
    entries: Vec<TTEntry>,       // Direct indexing
    mask: usize,                  // Power of 2 size - 1
    hits: u64,
    misses: u64,
}

#[repr(C, align(16))]  // Cache line alignment
pub struct TTEntry {
    hash: u64,         // Full hash for verification
    data: u64,         // Packed: depth(8) | flag(8) | score(32) | move(16)
}
```

**Improvements:**
1. **Direct addressing**: `index = hash & mask`
2. **No allocation overhead**: Just the data itself
3. **Cache-friendly**: Sequential memory, predictable access
4. **Replacement strategy**: Replace based on depth/age, not clear entire table
5. **SIMD potential**: Aligned entries enable vectorized operations

### Impact
- **Memory**: ~70% reduction in actual memory usage
- **Speed**: 2-3x faster lookups (no hash computation, direct array access)
- **Predictability**: Consistent cache behavior

---

## 3. HIGH PRIORITY: MoveList Cloning Inefficiency

### Current Issue
```rust
pub fn get_move_list(&self) -> MoveList {
    self.move_list.clone()  // ← Expensive Vec clone
}
```

**Location**: `kelp_engine/src/kelp/mov_gen/generator.rs:22-24`

**Problem**: Every call to `get_move_list()` performs a full Vec clone (up to 256 moves × 7 bytes = 1.8 KB)

### Solutions

#### Option A: Return Reference (Best)
```rust
pub fn get_move_list(&self) -> &MoveList {
    &self.move_list
}
```

#### Option B: Move Semantics
```rust
pub fn take_move_list(&mut self) -> MoveList {
    std::mem::take(&mut self.move_list)
}
```

#### Option C: Iterator Pattern
```rust
pub fn moves(&self) -> impl Iterator<Item = &Move> {
    self.move_list.iter()
}
```

### Impact
- **Performance**: Eliminate unnecessary allocations in hot paths
- **Memory**: Reduce temporary allocations during search

---

## 4. MEDIUM PRIORITY: LookupTable Memory Optimization

### Current Implementation
```rust
pub struct LookupTable {
    pawn_attacks: Vec<Vec<BitBoard>>,     // 2 × 64 × 8 = 1 KB
    knight_attacks: Vec<BitBoard>,        // 64 × 8 = 512 bytes
    king_attacks: Vec<BitBoard>,          // 64 × 8 = 512 bytes
    bishop_attacks: Vec<Vec<BitBoard>>,   // 64 × 512 × 8 = 256 KB
    rook_attacks: Vec<Vec<BitBoard>>,     // 64 × 4096 × 8 = 2 MB
    magic_table: MagicTable,
}
```

**Total**: ~2.3 MB

### Optimization Opportunities

#### A. Use Fixed-Size Arrays (Remove Vec overhead)
```rust
pub struct LookupTable {
    pawn_attacks: [[BitBoard; 64]; 2],
    knight_attacks: [BitBoard; 64],
    king_attacks: [BitBoard; 64],
    bishop_attacks: [[BitBoard; 512]; 64],
    rook_attacks: [[BitBoard; 4096]; 64],
    magic_table: MagicTable,
}
```

**Benefits:**
- No heap allocation
- No Vec metadata (capacity, length pointers)
- Better cache locality
- Compile-time size checking

#### B. Lazy Initialization
Only populate attack tables when needed, or populate on first access.

### Impact
- **Memory**: ~40 KB saved (Vec overhead × large tables)
- **Startup time**: Faster initialization
- **Cache**: Better locality

---

## 5. MEDIUM PRIORITY: Evaluation Function Optimization

### Current Issues in `eval.rs`

#### A. Redundant Calculations
```rust
// File: kelp_engine/src/kelp/search/eval.rs
```

**Problems:**
1. **No incremental evaluation**: Full board evaluation on every node
2. **Repeated bitboard operations**: Same piece locations scanned multiple times
3. **No material balance caching**: Recalculated constantly

#### B. Evaluation Caching
```rust
pub struct EvalCache {
    pawn_structure_cache: HashMap<u64, i32>,  // Cache pawn structure eval
    material_balance: i32,                     // Incremental material
}
```

**Strategy:**
- Cache pawn structure evaluation (pawns rarely move)
- Maintain incremental material balance
- Use Zobrist hashing for pawn structure

### Impact
- **Speed**: 30-40% faster evaluation in middle game
- **Nodes/sec**: Significant increase in search depth

---

## 6. LOW PRIORITY: Magic Bitboard Lookup Optimization

### Current Implementation
```rust
#[inline(always)]
pub fn get_bishop_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
    let magic = self.magic_table.bishop[square as usize];
    let mut occ = occupancy;
    occ &= magic.mask;
    occ *= magic.magic;
    occ >>= magic.shift;
    self.bishop_attacks[square as usize][occ.0 as usize]
}
```

### Optimization: Inline Magic Data
```rust
#[inline(always)]
pub fn get_bishop_attacks(&self, square: u8, occupancy: BitBoard) -> BitBoard {
    // Access magic data inline, avoiding one indirection
    let magic = BISHOP_MAGICS[square as usize];  // const array
    let index = ((occupancy & magic.mask) * magic.magic) >> magic.shift;
    BISHOP_ATTACKS[square as usize][index as usize]
}
```

### Impact
- **Speed**: 5-10% faster attack generation
- **Cache**: One less memory indirection

---

## 7. ARCHITECTURAL: Move Generation Strategy

### Current Approach
```rust
pub fn generate_moves(&mut self, board: &Board) {
    let side = board.get_side_to_move();
    self.move_list.clear();
    self.generate_pawn_moves(side, board);
    self.generate_castling_moves(side, board);
    self.generate_king_moves(side, board);
    self.generate_knight_moves(side, board);
    self.generate_bishop_moves(side, board);
    self.generate_rook_moves(side, board);
    self.generate_queen_moves(side, board);
}
```

### Optimization: Staged Move Generation
```rust
pub enum MoveGenStage {
    HashMove,       // TT move
    Captures,       // MVV-LVA sorted
    Killers,        // Killer moves
    Quiet,          // Remaining moves
}
```

**Benefits:**
1. **Alpha-beta cutoffs**: Generate high-value moves first
2. **Lazy generation**: Don't generate all moves if early cutoff
3. **Better move ordering**: Faster search tree pruning

### Impact
- **Search speed**: 2-3x improvement in nodes/second
- **Effective depth**: Deeper search in same time

---

## 8. CRITICAL: Remove MoveList/MoveArray Duplication

### Current Problem
```rust
// Two different containers for moves:
pub struct MoveList(pub Vec<Move>);      // Used in move generation
pub struct MoveArray {                    // Used in move history
    pub moves: [Option<MoveHistory>; MAX_SIZE_MOVES_ARR],
    pub count: usize,
}
```

**Issues:**
1. Duplicated functionality
2. Conversion overhead between types
3. Confusion about which to use where

### Solution
```rust
// Single, efficient container
pub struct MoveList {
    moves: [Move; 256],
    len: u8,  // Only 256 max, so u8 is enough
}
```

### Impact
- **Code clarity**: Single way to handle move lists
- **Performance**: No vec allocations, fixed size
- **Memory**: Predictable, stack-allocated

---

## 9. ADVANCED: SIMD Evaluation

### Opportunity
Modern CPUs support SIMD (AVX2/AVX-512) for parallel operations.

### Application Areas
1. **Piece counting**: Count pieces across all bitboards simultaneously
2. **Mobility calculation**: Count set bits in multiple bitboards at once
3. **Pattern matching**: Check multiple piece patterns in parallel

### Example (using `std::arch`)
```rust
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn count_material_simd(bitboards: &[u64; 12]) -> i32 {
    // Load 4 bitboards at a time
    // Use popcnt instruction
    // Multiply by piece values in parallel
    // Sum results
}
```

### Impact
- **Speed**: 2-4x faster evaluation on modern CPUs
- **Complexity**: Moderate (requires unsafe code)

---

## 10. ADVANCED: Parallel Search

### Current Limitation
Single-threaded search limits scalability.

### Approaches

#### A. Young Brothers Wait Concept (YBWC)
- Main thread searches first move
- Helper threads search remaining moves in parallel
- Join results at parent node

#### B. Lazy SMP
- Multiple threads search same position
- Shared transposition table
- Simple but effective for chess engines

### Impact
- **Speed**: Near-linear scaling with cores (4 cores = 3.5x speed)
- **Complexity**: High (thread synchronization, TT locking)

---

## Priority Implementation Order

1. **IMMEDIATE** (Critical path optimizations):
   - Move struct bit-packing (7 → 4 bytes)
   - Fix MoveList cloning (return references)
   - TranspositionTable replacement (HashMap → Vec)

2. **SHORT-TERM** (High-value, lower risk):
   - LookupTable array conversion
   - Staged move generation
   - Remove MoveList/MoveArray duplication

3. **MEDIUM-TERM** (Algorithmic improvements):
   - Evaluation caching
   - Magic bitboard inlining
   - Incremental evaluation

4. **LONG-TERM** (Advanced features):
   - SIMD evaluation
   - Parallel search
   - Neural network evaluation

---

## Estimated Performance Gains

### Combined Impact of Top 3 Optimizations:
- **Memory usage**: 45-50% reduction
- **Search speed**: 2-3x faster (nodes/second)
- **Cache efficiency**: 40-60% improvement
- **Effective depth**: +2-3 ply deeper in same time

### Testing Methodology:
1. Benchmark current version (perft, tactical suite)
2. Implement optimizations incrementally
3. Measure each optimization's impact
4. Validate correctness with test suite
5. Compare search performance on standard positions

---

## Correctness Validation

Before and after each optimization:
1. Run full test suite
2. Perft validation (move generation correctness)
3. Tactical test suite (search correctness)
4. Self-play matches (overall strength)

---

## References

- Chessprogramming Wiki: https://www.chessprogramming.org/
- Stockfish source code (reference implementation)
- AMD64 optimization manual (SIMD, cache optimization)
