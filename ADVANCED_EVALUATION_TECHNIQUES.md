# Advanced Evaluation Techniques for Chess Engines

This document provides detailed specifications for implementing advanced evaluation techniques in the Kelp chess engine. These techniques go beyond basic material counting to assess complex positional factors that distinguish strong engines from weak ones.

## Table of Contents

1. [King Safety](#king-safety)
2. [Pawn Shield](#pawn-shield)
3. [Pawn Structure](#pawn-structure)
4. [Piece Mobility](#piece-mobility)
5. [Piece Coordination](#piece-coordination)
6. [Space Evaluation](#space-evaluation)
7. [Threat Detection](#threat-detection)
8. [Endgame Recognition](#endgame-recognition)
9. [Tempo and Initiative](#tempo-and-initiative)
10. [Implementation Priorities](#implementation-priorities)

---

## 1. King Safety

King safety is one of the most critical evaluation factors in chess. A vulnerable king can lead to immediate tactical losses, making this evaluation crucial during the opening and middlegame.

### 1.1 Attack Zone Definition

Define the king's attack zone (typically a 3x3 or 5x5 square around the king):

```rust
// King attack zones
const KING_ZONE_INNER: [[BitBoard; 64]; 2] = ...; // 3x3 area
const KING_ZONE_OUTER: [[BitBoard; 64]; 64]; 2] = ...; // 5x5 area for long-range attacks

pub fn init_king_zones() {
    for sq in 0..64 {
        // Inner zone: king attacks + king square
        king_zone_inner[White][sq] = get_king_attacks(sq) | (1 << sq);
        
        // Outer zone: extended area for bishops, rooks, queens
        king_zone_outer[White][sq] = expand_zone(king_zone_inner[White][sq], 1);
    }
}
```

### 1.2 Attack Weights by Piece

Different pieces contribute different levels of danger when attacking the king:

```rust
const ATTACK_WEIGHT: [i32; 12] = [
    0,   // WhitePawn (handled separately)
    2,   // WhiteKnight
    2,   // WhiteBishop
    3,   // WhiteRook
    5,   // WhiteQueen
    0,   // WhiteKing
    0,   // BlackPawn
    2,   // BlackKnight
    2,   // BlackBishop
    3,   // BlackRook
    5,   // BlackQueen
    0,   // BlackKing
];

// Attack unit calculation
pub fn calculate_king_attackers(board: &Board, gen: &MovGen, king_sq: Squares, attacking_side: Color) -> i32 {
    let mut attack_units = 0;
    let mut attacker_count = 0;
    
    let king_zone = get_king_zone(king_sq);
    
    // Check each attacking piece type
    for piece in get_pieces_for_color(attacking_side) {
        let piece_bb = board.get_piece_occ(piece);
        
        for sq in piece_bb {
            let attacks = get_attacks(piece, sq, board.get_occ());
            
            if (attacks & king_zone).is_not_empty() {
                attack_units += ATTACK_WEIGHT[piece as usize];
                attacker_count += 1;
            }
        }
    }
    
    attack_units
}
```

### 1.3 King Safety Scoring

Convert attack units to a safety score using a non-linear scale:

```rust
// King safety score table (indexed by attack units)
// The more attackers, the exponentially worse it gets
const KING_SAFETY_TABLE: [i32; 100] = [
    0, 0, 1, 2, 3, 5, 7, 9, 12, 15,           // 0-9 attack units
    18, 22, 26, 30, 35, 39, 44, 50, 56, 62,   // 10-19
    68, 75, 82, 85, 89, 97, 105, 113, 122, 131, // 20-29
    140, 150, 169, 180, 191, 202, 213, 225, 237, 248, // 30-39
    260, 272, 283, 295, 307, 319, 330, 342, 354, 366, // 40-49
    377, 389, 401, 412, 424, 436, 448, 459, 471, 483, // 50-59
    494, 500, 500, 500, 500, 500, 500, 500, 500, 500, // 60+ (capped)
    // ... rest capped at 500
];

pub fn king_safety_score(attack_units: i32) -> i32 {
    let index = min(attack_units as usize, 99);
    KING_SAFETY_TABLE[index]
}
```

### 1.4 King Tropism (Piece Proximity)

Penalize enemy pieces being too close to our king:

```rust
const KING_TROPISM_WEIGHT: [i32; 12] = [
    0,   // Pawn
    3,   // Knight (dangerous when close)
    1,   // Bishop
    2,   // Rook
    4,   // Queen (very dangerous)
    0,   // King
    // ... mirror for black
];

pub fn calculate_king_tropism(board: &Board, our_king_sq: Squares, enemy_color: Color) -> i32 {
    let mut tropism_score = 0;
    
    for piece in get_pieces_for_color(enemy_color) {
        let piece_bb = board.get_piece_occ(piece);
        
        for sq in piece_bb {
            let distance = chebyshev_distance(our_king_sq, sq);
            let penalty = KING_TROPISM_WEIGHT[piece as usize] * (7 - distance);
            tropism_score += penalty;
        }
    }
    
    tropism_score
}

fn chebyshev_distance(sq1: Squares, sq2: Squares) -> i32 {
    let rank_dist = (sq1.rank() as i32 - sq2.rank() as i32).abs();
    let file_dist = (sq1.file() as i32 - sq2.file() as i32).abs();
    max(rank_dist, file_dist)
}
```

### 1.5 Open Files Near King

Open or semi-open files near the king are dangerous for rook/queen attacks:

```rust
pub fn open_files_near_king(board: &Board, king_sq: Squares, our_color: Color) -> i32 {
    let mut penalty = 0;
    let king_file = king_sq.file();
    
    // Check files near the king (king file ± 1)
    for file_offset in -1..=1 {
        let file = king_file as i32 + file_offset;
        if file < 0 || file > 7 {
            continue;
        }
        
        let file_mask = get_file_mask(file as u8);
        let our_pawns = board.get_piece_occ(get_pawn(our_color)) & file_mask;
        let enemy_pawns = board.get_piece_occ(get_pawn(!our_color)) & file_mask;
        
        if our_pawns.is_empty() {
            if enemy_pawns.is_empty() {
                // Open file - very dangerous
                penalty += 20;
            } else {
                // Semi-open file - somewhat dangerous
                penalty += 10;
            }
        }
    }
    
    penalty
}
```

### 1.6 Castling Rights and King Position

```rust
pub fn evaluate_king_position(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    let king_sq = board.get_king_square(color);
    let phase = board.phase;
    
    // Castling rights bonus (in opening/middlegame)
    if phase != EndGame {
        if board.info.castle.can_castle_king_side(color) {
            score += 30;
        }
        if board.info.castle.can_castle_queen_side(color) {
            score += 25;
        }
    }
    
    // King centralization in endgame
    if phase == EndGame {
        let centralization = CENTER_MANHATTAN_DISTANCE[king_sq as usize];
        score += (14 - centralization) * 10; // Closer to center is better
    } else {
        // King should be on back rank or castled in middlegame
        let rank = king_sq.rank();
        let ideal_rank = if color == White { 0 } else { 7 };
        
        if rank == ideal_rank {
            score += 15;
        } else {
            score -= (rank as i32 - ideal_rank as i32).abs() * 5;
        }
    }
    
    score
}
```

---

## 2. Pawn Shield

A pawn shield protects the king from direct attacks. This is crucial in the opening and middlegame.

### 2.1 Pawn Shield Structure

```rust
// Pawn shield templates for different king positions
// These define ideal pawn formations in front of the king

// King on g1 (white kingside castled)
const IDEAL_SHIELD_KS_WHITE: [Squares; 3] = [F2, G2, H2];
const SECONDARY_SHIELD_KS_WHITE: [Squares; 3] = [F3, G3, H3];

// King on c1 (white queenside castled)
const IDEAL_SHIELD_QS_WHITE: [Squares; 3] = [A2, B2, C2];
const SECONDARY_SHIELD_QS_WHITE: [Squares; 3] = [A3, B3, C3];

// Pawn shield evaluation structure
pub struct PawnShield {
    pub pawns_in_front: u8,      // Count of pawns directly shielding
    pub pawn_distance: [u8; 3],  // Distance each shield pawn moved
    pub missing_pawns: u8,        // Count of missing shield pawns
    pub pawn_storms: u8,          // Enemy pawns attacking the shield
}

impl PawnShield {
    pub fn evaluate(&self, phase: GamePhase) -> i32 {
        let mut score = 0;
        
        // Bonus for each shield pawn
        score += (self.pawns_in_front as i32) * 15;
        
        // Penalty for missing shield pawns
        score -= (self.missing_pawns as i32) * 25;
        
        // Penalty for each square a shield pawn has advanced
        for &distance in &self.pawn_distance {
            score -= (distance as i32) * 10;
        }
        
        // Penalty for enemy pawn storms
        score -= (self.pawn_storms as i32) * 15;
        
        // Phase-dependent scaling
        match phase {
            Opening | MiddleGame => score,
            EndGame => score / 2, // Less important in endgame
        }
    }
}
```

### 2.2 Pawn Shield Detection

```rust
pub fn analyze_pawn_shield(board: &Board, king_sq: Squares, color: Color) -> PawnShield {
    let mut shield = PawnShield {
        pawns_in_front: 0,
        pawn_distance: [0; 3],
        missing_pawns: 0,
        pawn_storms: 0,
    };
    
    let king_file = king_sq.file();
    let our_pawns = board.get_piece_occ(get_pawn(color));
    let enemy_pawns = board.get_piece_occ(get_pawn(!color));
    
    // Define shield squares based on king position
    let shield_files = if king_file >= 5 {
        // Kingside
        [max(0, king_file - 1), king_file, min(7, king_file + 1)]
    } else if king_file <= 2 {
        // Queenside
        [max(0, king_file - 1), king_file, min(7, king_file + 1)]
    } else {
        // King in center (not ideal)
        [max(0, king_file - 1), king_file, min(7, king_file + 1)]
    };
    
    // Check each shield file
    for (i, &file) in shield_files.iter().enumerate() {
        let file_mask = get_file_mask(file);
        let our_pawns_on_file = our_pawns & file_mask;
        let enemy_pawns_on_file = enemy_pawns & file_mask;
        
        if our_pawns_on_file.is_empty() {
            shield.missing_pawns += 1;
        } else {
            shield.pawns_in_front += 1;
            
            // Calculate how far the pawn has moved
            let pawn_sq = our_pawns_on_file.get_lsb();
            let ideal_rank = if color == White { 1 } else { 6 };
            let current_rank = Squares::from_repr(pawn_sq).unwrap().rank();
            shield.pawn_distance[i] = (current_rank as i32 - ideal_rank as i32).abs() as u8;
        }
        
        // Check for enemy pawn storms
        if !enemy_pawns_on_file.is_empty() {
            let enemy_pawn_sq = enemy_pawns_on_file.get_lsb();
            let enemy_rank = Squares::from_repr(enemy_pawn_sq).unwrap().rank();
            
            // Enemy pawn is advanced and attacking our king
            let distance_to_king = if color == White {
                7 - enemy_rank
            } else {
                enemy_rank
            };
            
            if distance_to_king <= 3 {
                shield.pawn_storms += 1;
            }
        }
    }
    
    shield
}
```

### 2.3 Fianchetto Detection

Fianchetto (bishop behind pawn on g2/b2/g7/b7) is a special shield structure:

```rust
pub fn evaluate_fianchetto(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    
    let our_bishops = board.get_piece_occ(get_bishop(color));
    let our_pawns = board.get_piece_occ(get_pawn(color));
    
    // Check for kingside fianchetto
    let ks_bishop_sq = if color == White { Squares::G2 } else { Squares::G7 };
    let ks_pawn_sq = if color == White { Squares::G3 } else { Squares::G6 };
    
    if our_bishops.get_bit(ks_bishop_sq as u8) && our_pawns.get_bit(ks_pawn_sq as u8) {
        score += 25; // Bonus for fianchetto structure
        
        // Extra bonus if king is castled kingside
        let king_sq = board.get_king_square(color);
        if (color == White && king_sq.file() >= 5) || 
           (color == Black && king_sq.file() >= 5) {
            score += 15;
        }
    }
    
    // Check for queenside fianchetto
    let qs_bishop_sq = if color == White { Squares::B2 } else { Squares::B7 };
    let qs_pawn_sq = if color == White { Squares::B3 } else { Squares::B6 };
    
    if our_bishops.get_bit(qs_bishop_sq as u8) && our_pawns.get_bit(qs_pawn_sq as u8) {
        score += 25;
        
        let king_sq = board.get_king_square(color);
        if (color == White && king_sq.file() <= 2) || 
           (color == Black && king_sq.file() <= 2) {
            score += 15;
        }
    }
    
    score
}
```

---

## 3. Pawn Structure

Advanced pawn structure evaluation beyond basic doubled/isolated pawns.

### 3.1 Pawn Chains

Pawn chains provide structural strength:

```rust
pub fn evaluate_pawn_chains(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    let our_pawns = board.get_piece_occ(get_pawn(color));
    
    for sq in our_pawns {
        let pawn_sq = Squares::from_repr(sq).unwrap();
        
        // Check if this pawn is part of a chain (protected by another pawn)
        let protecting_pawns = get_pawn_attacks(!color, sq) & our_pawns;
        
        if protecting_pawns.count_bits() > 0 {
            score += 5; // Bonus for being in a chain
            
            // Extra bonus for being at the base of the chain
            let rank = pawn_sq.rank();
            let is_base = if color == White {
                rank <= 3
            } else {
                rank >= 4
            };
            
            if is_base {
                score += 3;
            }
        }
    }
    
    score
}
```

### 3.2 Backward Pawns

Pawns that can't be defended by other pawns and are behind their neighbors:

```rust
pub fn evaluate_backward_pawns(board: &Board, color: Color) -> i32 {
    let mut penalty = 0;
    let our_pawns = board.get_piece_occ(get_pawn(color));
    let enemy_pawns = board.get_piece_occ(get_pawn(!color));
    
    for sq in our_pawns {
        let pawn_sq = Squares::from_repr(sq).unwrap();
        let file = pawn_sq.file();
        
        // Check adjacent files for friendly pawns
        let left_file = if file > 0 { file - 1 } else { file };
        let right_file = if file < 7 { file + 1 } else { file };
        
        let adjacent_pawns = our_pawns & 
            (get_file_mask(left_file) | get_file_mask(right_file));
        
        // Check if all adjacent pawns are ahead of this one
        let mut is_backward = true;
        for adj_sq in adjacent_pawns {
            let adj_pawn = Squares::from_repr(adj_sq).unwrap();
            
            if color == White {
                if adj_pawn.rank() <= pawn_sq.rank() {
                    is_backward = false;
                    break;
                }
            } else {
                if adj_pawn.rank() >= pawn_sq.rank() {
                    is_backward = false;
                    break;
                }
            }
        }
        
        if is_backward {
            // Check if the square in front is controlled by enemy pawns
            let square_in_front = if color == White {
                pawn_sq as u8 + 8
            } else {
                pawn_sq as u8 - 8
            };
            
            let attacked_by_enemy = get_pawn_attacks(color, square_in_front) & enemy_pawns;
            
            if attacked_by_enemy.is_not_empty() {
                penalty += 15; // Backward pawn penalty
            }
        }
    }
    
    penalty
}
```

### 3.3 Pawn Islands

The number of pawn groups separated by empty files:

```rust
pub fn count_pawn_islands(board: &Board, color: Color) -> i32 {
    let our_pawns = board.get_piece_occ(get_pawn(color));
    let mut islands = 0;
    let mut in_island = false;
    
    for file in 0..8 {
        let file_mask = get_file_mask(file);
        let pawns_on_file = our_pawns & file_mask;
        
        if pawns_on_file.is_not_empty() {
            if !in_island {
                islands += 1;
                in_island = true;
            }
        } else {
            in_island = false;
        }
    }
    
    // Penalty: more islands = weaker pawn structure
    // Ideal is 1 island, each additional island costs 10 centipawns
    (islands - 1) * 10
}
```

---

## 4. Piece Mobility

Enhanced mobility evaluation with context-aware bonuses.

### 4.1 Safe Mobility

Only count squares that aren't attacked by enemy pawns:

```rust
pub fn calculate_safe_mobility(
    board: &Board, 
    gen: &MovGen, 
    piece: BoardPiece, 
    sq: u8,
    enemy_pawn_attacks: BitBoard
) -> i32 {
    let attacks = gen.get_attacks(piece, sq, board.get_occ());
    let our_pieces = board.get_color_occ(piece.get_color());
    
    // Available squares (not occupied by our pieces)
    let available = attacks & !our_pieces;
    
    // Safe squares (not attacked by enemy pawns)
    let safe = available & !enemy_pawn_attacks;
    
    safe.count_bits() as i32
}
```

### 4.2 Mobility Bonuses by Piece Type

```rust
const KNIGHT_MOBILITY_BONUS: [i32; 9] = [
    -20, -10, 0, 5, 5, 10, 10, 10, 10
];

const BISHOP_MOBILITY_BONUS: [i32; 14] = [
    -15, -10, -5, 0, 5, 10, 15, 15, 15, 15, 15, 15, 15, 15
];

const ROOK_MOBILITY_BONUS: [i32; 15] = [
    -10, -5, 0, 2, 4, 6, 8, 10, 12, 14, 14, 14, 14, 14, 14
];

const QUEEN_MOBILITY_BONUS: [i32; 28] = [
    -10, -5, 0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14
];

pub fn mobility_bonus(piece: BoardPiece, mobility: usize) -> i32 {
    match piece {
        WhiteKnight | BlackKnight => {
            KNIGHT_MOBILITY_BONUS[min(mobility, 8)]
        }
        WhiteBishop | BlackBishop => {
            BISHOP_MOBILITY_BONUS[min(mobility, 13)]
        }
        WhiteRook | BlackRook => {
            ROOK_MOBILITY_BONUS[min(mobility, 14)]
        }
        WhiteQueen | BlackQueen => {
            QUEEN_MOBILITY_BONUS[min(mobility, 27)]
        }
        _ => 0
    }
}
```

---

## 5. Piece Coordination

Evaluate how well pieces work together.

### 5.1 Rook on Open File

```rust
pub fn rook_on_open_file(board: &Board, rook_sq: Squares, color: Color) -> i32 {
    let file = rook_sq.file();
    let file_mask = get_file_mask(file);
    
    let our_pawns = board.get_piece_occ(get_pawn(color)) & file_mask;
    let enemy_pawns = board.get_piece_occ(get_pawn(!color)) & file_mask;
    
    if our_pawns.is_empty() && enemy_pawns.is_empty() {
        return 20; // Open file
    } else if our_pawns.is_empty() {
        return 10; // Semi-open file
    }
    
    0
}
```

### 5.2 Connected Rooks

```rust
pub fn connected_rooks(board: &Board, color: Color) -> i32 {
    let rooks = board.get_piece_occ(get_rook(color));
    
    if rooks.count_bits() < 2 {
        return 0;
    }
    
    let rook_squares: Vec<u8> = rooks.into_iter().collect();
    let r1 = Squares::from_repr(rook_squares[0]).unwrap();
    let r2 = Squares::from_repr(rook_squares[1]).unwrap();
    
    // Rooks are connected if they're on the same rank or file with no pieces between
    if r1.rank() == r2.rank() {
        let between = get_between_mask(r1, r2);
        if (between & board.get_occ()).is_empty() {
            return 20; // Connected on rank
        }
    }
    
    if r1.file() == r2.file() {
        let between = get_between_mask(r1, r2);
        if (between & board.get_occ()).is_empty() {
            return 20; // Connected on file
        }
    }
    
    0
}
```

### 5.3 Bishop Pair

```rust
pub fn bishop_pair_bonus(board: &Board, color: Color) -> i32 {
    let bishops = board.get_piece_occ(get_bishop(color));
    
    if bishops.count_bits() >= 2 {
        // Check if we have both light and dark square bishops
        let light_squares = bishops & LIGHT_SQUARES_MASK;
        let dark_squares = bishops & DARK_SQUARES_MASK;
        
        if light_squares.is_not_empty() && dark_squares.is_not_empty() {
            return 50; // Bishop pair bonus
        }
    }
    
    0
}
```

---

## 6. Space Evaluation

Control of the board center and advanced squares.

### 6.1 Center Control

```rust
const CENTER_SQUARES: BitBoard = BitBoard(0x0000001818000000); // d4, e4, d5, e5
const EXTENDED_CENTER: BitBoard = BitBoard(0x00003C3C3C3C0000); // c3-f3 to c6-f6

pub fn center_control(board: &Board, gen: &MovGen, color: Color) -> i32 {
    let mut score = 0;
    
    // Pawn center control
    let our_pawns = board.get_piece_occ(get_pawn(color));
    score += (our_pawns & CENTER_SQUARES).count_bits() as i32 * 10;
    score += (our_pawns & EXTENDED_CENTER).count_bits() as i32 * 5;
    
    // Piece attacks on center
    let our_attacks = gen.get_all_attacks(color, board);
    score += (our_attacks & CENTER_SQUARES).count_bits() as i32 * 2;
    
    score
}
```

### 6.2 Space Advantage

```rust
pub fn space_evaluation(board: &Board, color: Color) -> i32 {
    let our_pawns = board.get_piece_occ(get_pawn(color));
    let our_pieces = board.get_color_occ(color);
    
    // Define "our territory" based on pawn advancement
    let our_half = if color == White {
        BitBoard(0xFFFFFFFF00000000) // Ranks 5-8 for white
    } else {
        BitBoard(0x00000000FFFFFFFF) // Ranks 1-4 for black
    };
    
    // Count squares in our half controlled by pawns or occupied by pieces
    let pawn_controlled = gen.get_pawn_controlled_squares(our_pawns, color);
    let controlled_space = (pawn_controlled | our_pieces) & our_half;
    
    controlled_space.count_bits() as i32
}
```

---

## 7. Threat Detection

Identify and penalize threats against our pieces.

### 7.1 Hanging Pieces

```rust
pub fn evaluate_hanging_pieces(board: &Board, gen: &MovGen, color: Color) -> i32 {
    let mut penalty = 0;
    
    let our_pieces = board.get_color_occ(color);
    let enemy_attacks = gen.get_all_attacks(!color, board);
    let our_attacks = gen.get_all_attacks(color, board);
    
    // Check each of our pieces
    for piece in BoardPiece::iter() {
        if piece.get_color() != color {
            continue;
        }
        
        let piece_bb = board.get_piece_occ(piece);
        
        for sq in piece_bb {
            // Is this piece attacked?
            if enemy_attacks.get_bit(sq) {
                // Is it defended?
                if !our_attacks.get_bit(sq) {
                    // Hanging piece!
                    penalty += piece.get_value() / 2;
                } else {
                    // Attacked and defended - check if it's a bad trade
                    let attacker_value = get_least_valuable_attacker(!color, sq, board);
                    let defender_value = get_least_valuable_attacker(color, sq, board);
                    
                    if attacker_value < piece.get_value() && defender_value > attacker_value {
                        // We'd lose material in the exchange
                        penalty += (defender_value - attacker_value) / 4;
                    }
                }
            }
        }
    }
    
    penalty
}
```

### 7.2 Forks and Pins

```rust
pub fn detect_forks(board: &Board, gen: &MovGen, color: Color) -> i32 {
    let mut score = 0;
    
    // Check our knights for fork opportunities
    let our_knights = board.get_piece_occ(get_knight(color));
    let enemy_pieces = board.get_color_occ(!color);
    
    for sq in our_knights {
        let attacks = gen.table.get_knight_attacks(sq);
        let attacked_pieces = attacks & enemy_pieces;
        
        if attacked_pieces.count_bits() >= 2 {
            // This knight is forking!
            let mut fork_value = 0;
            for target in attacked_pieces {
                let target_sq = Squares::from_repr(target).unwrap();
                if let Some(piece) = board.get_piece(target_sq) {
                    fork_value += piece.get_value();
                }
            }
            
            score += fork_value / 10; // Bonus for fork potential
        }
    }
    
    score
}

pub fn detect_pins(board: &Board, gen: &MovGen, color: Color) -> i32 {
    let mut score = 0;
    
    let enemy_king_sq = board.get_king_square(!color);
    let our_bishops = board.get_piece_occ(get_bishop(color));
    let our_rooks = board.get_piece_occ(get_rook(color));
    let our_queens = board.get_piece_occ(get_queen(color));
    
    // Check for bishop/queen pins on diagonals
    for sq in our_bishops | our_queens {
        let attacks = gen.get_bishop_attacks(sq, board.get_occ());
        
        if attacks.get_bit(enemy_king_sq as u8) {
            // There's a line to the king - check for pinned piece
            let between = get_between_mask(
                Squares::from_repr(sq).unwrap(),
                enemy_king_sq
            );
            
            let pieces_between = between & board.get_occ();
            
            if pieces_between.count_bits() == 1 {
                // Exactly one piece between - it's pinned!
                let pinned_sq = pieces_between.get_lsb();
                if let Some(piece) = board.get_piece(Squares::from_repr(pinned_sq).unwrap()) {
                    if piece.get_color() != color {
                        score += 25; // Pin bonus
                    }
                }
            }
        }
    }
    
    // Similar check for rook/queen pins on files/ranks
    // ... (code similar to above but for orthogonal lines)
    
    score
}
```

---

## 8. Endgame Recognition

Specialized evaluation for different endgame types.

### 8.1 Pawn Endgames

```rust
pub fn evaluate_pawn_endgame(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    
    let our_pawns = board.get_piece_occ(get_pawn(color));
    let enemy_pawns = board.get_piece_occ(get_pawn(!color));
    let our_king_sq = board.get_king_square(color);
    
    // King activity is crucial
    for sq in our_pawns {
        let pawn_sq = Squares::from_repr(sq).unwrap();
        let distance = manhattan_distance(our_king_sq, pawn_sq);
        score += (7 - distance) * 5; // King should support pawns
    }
    
    // Passed pawns are extremely valuable
    for sq in our_pawns {
        let pawn_sq = Squares::from_repr(sq).unwrap();
        if is_passed_pawn(pawn_sq, color, enemy_pawns) {
            let rank = pawn_sq.rank();
            let distance_to_promotion = if color == White { 7 - rank } else { rank };
            score += PASSED_PAWN_BONUS[distance_to_promotion as usize] * 2;
        }
    }
    
    score
}
```

### 8.2 Rook Endgames

```rust
pub fn evaluate_rook_endgame(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    
    let our_rooks = board.get_piece_occ(get_rook(color));
    let enemy_king_sq = board.get_king_square(!color);
    
    // Rook on 7th rank is very strong
    for sq in our_rooks {
        let rook_sq = Squares::from_repr(sq).unwrap();
        let rank = rook_sq.rank();
        
        if (color == White && rank == 6) || (color == Black && rank == 1) {
            score += 30;
            
            // Even stronger if enemy king is on back rank
            let enemy_rank = enemy_king_sq.rank();
            if (color == White && enemy_rank == 7) || (color == Black && enemy_rank == 0) {
                score += 20;
            }
        }
    }
    
    // Rook behind passed pawns
    for sq in our_rooks {
        let rook_sq = Squares::from_repr(sq).unwrap();
        // Check if there's a passed pawn in front of this rook
        // ... (implementation depends on passed pawn detection)
    }
    
    score
}
```

---

## 9. Tempo and Initiative

Evaluate who has the initiative in the position.

### 9.1 Piece Activity

```rust
pub fn piece_activity_score(board: &Board, gen: &MovGen, color: Color) -> i32 {
    let mut score = 0;
    
    // Count developed pieces (not on starting squares)
    let starting_rank = if color == White { 0 } else { 7 };
    
    for piece in [get_knight(color), get_bishop(color), get_rook(color), get_queen(color)] {
        let piece_bb = board.get_piece_occ(piece);
        
        for sq in piece_bb {
            let sq_obj = Squares::from_repr(sq).unwrap();
            
            if sq_obj.rank() != starting_rank {
                score += 10; // Piece is developed
                
                // Extra bonus for pieces in the center
                if CENTER_SQUARES.get_bit(sq) {
                    score += 5;
                }
            }
        }
    }
    
    score
}
```

### 9.2 Tempo Evaluation

```rust
pub fn tempo_evaluation(board: &Board) -> i32 {
    // Simple tempo bonus for side to move
    // In opening/middlegame, having the move is worth ~10-15 centipawns
    if board.phase != EndGame {
        if board.get_side_to_move() == White {
            10
        } else {
            -10
        }
    } else {
        // In endgame, tempo is less important
        0
    }
}
```

---

## 10. Implementation Priorities

### Phase 1: Immediate Impact (1-2 weeks)

**Critical for strength improvement:**

1. **Enhanced King Safety** (Priority: CRITICAL)
   - Implement attack zone calculation
   - Add attack weight system
   - Implement non-linear safety table
   - Expected impact: +100-200 Elo

2. **Improved Pawn Shield** (Priority: HIGH)
   - Implement shield structure detection
   - Add pawn storm detection
   - Add fianchetto recognition
   - Expected impact: +50-100 Elo

3. **Safe Mobility** (Priority: HIGH)
   - Modify existing mobility to exclude pawn-attacked squares
   - Add piece-specific mobility tables
   - Expected impact: +30-50 Elo

### Phase 2: Structural Improvements (2-4 weeks)

4. **Advanced Pawn Structure** (Priority: MEDIUM)
   - Pawn chains
   - Backward pawns
   - Pawn islands
   - Expected impact: +40-60 Elo

5. **Piece Coordination** (Priority: MEDIUM)
   - Connected rooks
   - Bishop pair
   - Rooks on open files
   - Expected impact: +30-50 Elo

6. **Space Evaluation** (Priority: LOW)
   - Center control
   - Territory evaluation
   - Expected impact: +20-30 Elo

### Phase 3: Tactical Awareness (4-6 weeks)

7. **Threat Detection** (Priority: MEDIUM)
   - Hanging pieces
   - Fork detection
   - Pin detection
   - Expected impact: +50-80 Elo

8. **Endgame Specialization** (Priority: MEDIUM)
   - Pawn endgame evaluation
   - Rook endgame evaluation
   - King activity in endgame
   - Expected impact: +40-60 Elo

### Implementation Template

```rust
// Add to eval.rs

impl Eval {
    pub fn evaluate_advanced(&self, board: &Board, gen: &MovGen) -> i32 {
        let mut score = 0;
        let phase = board.phase;
        
        // King safety evaluation
        let white_king_safety = self.evaluate_king_safety(board, gen, White);
        let black_king_safety = self.evaluate_king_safety(board, gen, Black);
        score += white_king_safety - black_king_safety;
        
        // Pawn shield evaluation
        let white_shield = self.evaluate_pawn_shield(board, White);
        let black_shield = self.evaluate_pawn_shield(board, Black);
        score += white_shield - black_shield;
        
        // Piece coordination
        let white_coordination = self.evaluate_piece_coordination(board, gen, White);
        let black_coordination = self.evaluate_piece_coordination(board, gen, Black);
        score += white_coordination - black_coordination;
        
        // ... more evaluations
        
        score
    }
    
    fn evaluate_king_safety(&self, board: &Board, gen: &MovGen, color: Color) -> i32 {
        let king_sq = board.get_king_square(color);
        let mut penalty = 0;
        
        // Attack units
        let attack_units = calculate_king_attackers(board, gen, king_sq, !color);
        penalty += king_safety_score(attack_units);
        
        // King tropism
        penalty += calculate_king_tropism(board, king_sq, !color);
        
        // Open files near king
        penalty += open_files_near_king(board, king_sq, color);
        
        // Phase-dependent scaling
        match board.phase {
            Opening | MiddleGame => -penalty,
            EndGame => -penalty / 3, // Less important in endgame
        }
    }
    
    // ... implement other evaluation functions
}
```

---

## Performance Considerations

### Caching Strategies

```rust
pub struct EvalCache {
    // Pawn structure is relatively static - cache it
    pawn_hash_table: HashMap<u64, PawnStructureEval>,
    
    // King safety zones can be precomputed
    king_zones: [[BitBoard; 64]; 2],
    
    // Common attack patterns
    attack_cache: HashMap<u64, BitBoard>,
}

#[derive(Clone, Copy)]
pub struct PawnStructureEval {
    hash: u64,
    score: i32,
    passed_pawns: [BitBoard; 2],
    weak_pawns: [BitBoard; 2],
}
```

### Incremental Updates

```rust
// Track changes to evaluation after each move
pub struct IncrementalEval {
    material_balance: i32,
    pst_score: i32, // Piece-square table score
    
    // Update these incrementally during make_move/unmake_move
    pub fn update_move(&mut self, mov: Move, board: &Board) {
        // Remove piece from old square
        self.pst_score -= PST[mov.piece()][mov.from()];
        
        // Add piece to new square
        self.pst_score += PST[mov.piece()][mov.to()];
        
        // Handle captures
        if let Some(captured) = mov.capture() {
            self.material_balance += captured.get_value();
            self.pst_score -= PST[captured][mov.to()];
        }
    }
}
```

---

## Testing and Tuning

### Evaluation Test Positions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_king_safety_recognition() {
        // Position with weak king
        let fen = "r1bq1rk1/ppp2ppp/2n5/3p4/3P4/2N5/PPP2PPP/R1BQ1RK1 w - - 0 1";
        let board = Board::from_fen(fen);
        
        let king_safety = evaluate_king_safety(&board, &gen, Black);
        assert!(king_safety < -50); // Black king should be unsafe
    }
    
    #[test]
    fn test_bishop_pair_detection() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KB1R w - - 0 1";
        let board = Board::from_fen(fen);
        
        let score = bishop_pair_bonus(&board, White);
        assert_eq!(score, 50);
    }
    
    // ... more tests
}
```

### Tuning Parameters

Use automated tuning (Texel tuning method):

```rust
pub struct TunableParams {
    pub king_safety_weights: [i32; 100],
    pub mobility_bonuses: [[i32; 28]; 4],
    pub pawn_structure_weights: [i32; 10],
    // ...
}

impl TunableParams {
    pub fn tune_with_positions(&mut self, positions: &[Position], results: &[f32]) {
        // Implement gradient descent or other optimization
        // to find parameter values that best match game results
    }
}
```

---

## Summary

This document provides comprehensive specifications for advanced evaluation techniques. Implementation priority:

1. **King Safety** - Most impactful (+100-200 Elo)
2. **Pawn Shield** - Critical for king safety (+50-100 Elo)
3. **Safe Mobility** - Quick win (+30-50 Elo)
4. **Pawn Structure** - Medium effort, good return (+40-60 Elo)
5. **Piece Coordination** - Adds strategic understanding (+30-50 Elo)

Total expected improvement from all techniques: **+300-500 Elo**

These techniques transform the engine from a tactical calculator to a positionally-aware player that understands chess strategy.
