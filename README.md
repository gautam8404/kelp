# Kelp

![Rust-CI](https://github.com/gautam8404/kelp/actions/workflows/rust-ci.yml/badge.svg)

<p align="center">
    <img src="img/kelp.png" alt="Kelp" width="200">
</p>

Kelp is a UCI compatible chess engine written in Rust Using standard chess algorithms.

Kelp is work in progress. Currently, it can be used as a UCI engine But evaluation needs a lot of work to be done especially the endgame evaluation.

## Play Against It


Lichess:- https://lichess.org/@/KelpBot

Kelp is UCI compatible it should work with any UCI compatible gui, so far kelp has been tested on [Pychess](https://github.com/pychess/pychess), [BanskaiGUI](https://banksiagui.com/), [Arena](http://www.playwitharena.de/) and [cutechess](https://cutechess.com/).


## Example

![Example](img/example.png)  

## Installation

Kelp Binary can be downloaded from [Releases](https://github.com/gautam8404/kelp/releases) page for Windows and Linux.

## Build

```bash
cargo build --release
```

## About

### Board
- A bitboard[12] array is used to represent the board. Each piece is mapped to a bitboard.
- standard make/unmake move functions are used, unmake move use move history to unmake instead of copy/take approach.

### Search
- Iterative deepening with aspiration windows.
- Negamax with alpha-beta pruning.
- Principal Variation Search.
- Late Move Reduction.
- Null Move Pruning.
- Quiescence Search.
- Transposition Table.
- Move Ordering
  - MVV-LVA
  - Killer Moves
  - History Heuristic
  - PV Table

### Evaluation
- Piece Square Tables
- Tapered Eval
- Basic King Safety
- Mobility & Basic Mop Up Evaluation
- Passed Pawns, Isolated Pawns, Doubled Pawns

### TODO
- [ ] Better King Safety and Pawn Shield
- [ ] Better Mobility
- [ ] Better Mop Up Evaluation
- [ ] Opening Book
- [ ] Integrate Syzygy Endgame Tablebases
- [ ] Integrate Stockfish's NNUE

## Tests

Kelp Implements some basic tests suchs as perft test, fen parsing and incremental update of zobrist hash.
Perft results are compared with [Perft Results](https://www.chessprogramming.org/Perft_Results) and incremental update of zobrist hash is compared with scratch generation of zobrist hash.

```bash
cargo test
```

## References & Resources

Resources that helped me a lot in making this engine.

- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Vice by Bluefever Software](https://github.com/bluefeversoft/vice) and corresponfing [playlist](https://www.youtube.com/playlist?list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg)
- [BBC by Code Monkey King](https://github.com/maksimKorzh/bbc) and the correspponding [playlist](https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs)
- [Pleco Chess Engine](https://github.com/pleco-rs/Pleco)
- [Chess-rs Chess Engine](https://github.com/ParthPant/chess-rs)
- [Fruit Chess Engine](https://github.com/Warpten/Fruit-2.1)
- [codefish Chess Engine](https://github.com/jsilll/codfish)
- [TSCP Chess Engine](http://www.tckerrigan.com/Chess/TSCP/)

## License
Kelp is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
