use crate::kelp::board::board::Board;
use crate::kelp::board::moves::MoveType;
use crate::kelp::kelp_core::lookup_table::LookupTable;
use crate::kelp::mov_gen::generator::MovGen;

#[derive(Debug, Default)]
pub struct Perft {
    pub nodes: u64,
    pub captures: u64,
    pub en_passants: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64, // TODO: Add checks
    checkmates: u64, //TODO: Add checkmate
}

impl Perft {
    pub fn check(
        &self,
        nodes: u64,
        captures: u64,
        en_passants: u64,
        castles: u64,
        promotions: u64,
    ) {
        if self.nodes != nodes
            || self.captures != captures
            || self.en_passants != en_passants
            || self.castles != castles
            || self.promotions != promotions
        {
            panic!(
                "\n Perft Test Failed! Details: \n
            Nodes: {} Expected: {} Difference: {} \n
            Captures: {} Expected: {} Difference: {} \n
            En Passants: {} Expected: {} Difference: {} \n
            Castles: {} Expected: {} Difference: {} \n
            Promotions: {} Expected: {} Difference: {} \n",
                self.nodes,
                nodes,
                self.nodes - nodes,
                self.captures,
                captures,
                self.captures - captures,
                self.en_passants,
                en_passants,
                self.en_passants - en_passants,
                self.castles,
                castles,
                self.castles - castles,
                self.promotions,
                promotions,
                self.promotions - promotions
            );
        }
        assert_eq!(self.nodes, nodes);
        assert_eq!(self.captures, captures);
        assert_eq!(self.en_passants, en_passants);
        assert_eq!(self.castles, castles);
        assert_eq!(self.promotions, promotions);
    }
}

// pub fn perft(depth: u16, board: &mut Board, gen: &mut MovGen, pft: &mut Perft) {
//     if depth == 0 {
//         pft.nodes += 1;
//         return ;
//     }
//
//     let side = board.info.turn;
//     gen.generate_moves(side, board);
//     let moves_list = gen.move_list.clone();
//     for moves in moves_list.iter() {
//         let a = board.make_move(*moves, true);
//         if !board.is_king_checked(side, gen) {
//             pft.nodes += 1;
//             perft(depth - 1, board, gen, pft);
//         }
//         if a.is_none() {
//             continue;
//         }
//         board.unmake_move(a.unwrap());
//     }
// }
// pub fn perft(depth: u16, board: &mut Board, gen: &mut MovGen, pft: & mut Perft) -> u32 {
//     if depth == 0 {
//         pft.nodes += 1;
//         return 1;
//     }
//     use MoveType::*;
//
//     let side = board.get_side_to_move();
//     gen.generate_moves( board);
//     let moves_list = gen.move_list.clone();
//
//     let mut nodes = 0;
//     let mut checkmates = 0;
//
//     for &mov in moves_list.iter() {
//         let game_state = board.get_game_state();
//         let a = board.simple_make_move(mov, false);
//         // if a.is_none() {
//         //     continue;
//         // }
//
//         if !board.is_king_checked(side, gen) {
//             if mov.is_capture() {
//                 pft.captures += 1;
//             }
//             if mov.is_en_passant() {
//                 pft.en_passants += 1;
//             }
//             if mov.is_castle() {
//                 pft.castles += 1;
//             }
//
//             if mov.is_promotion() {
//                 pft.promotions += 1;
//             }
//             nodes += perft(depth - 1, board, gen, pft);
//
//         } else {
//             pft.checks += 1;
//         }
//         board.simple_unmake_move(mov, game_state);
//     }
//     nodes
// }

pub fn perft_driver(depth: u16, board: &mut Board, gen: &mut MovGen, nodes: &mut u64) {
    if depth == 0 {
        *nodes += 1;
        return;
    }

    let side = board.get_side_to_move();
    gen.generate_moves(board);
    let moves_list = gen.move_list.clone();

    for &move_to_make in moves_list.iter() {
        let undo_info = board.make_move(move_to_make);
        if undo_info.is_none() {
            continue;
        }

        if !board.is_king_checked(side, gen) {
            perft_driver(depth - 1, board, gen, nodes);
        }

        board.unmake_move(undo_info.unwrap());
    }
}

pub fn perft_test(depth: u16, board: &mut Board, gen: &mut MovGen, nodes: &mut u64) {
    *nodes = 0;

    // println!("Starting Perft Test to depth: {depth}");
    gen.generate_moves(board);
    let moves_list = gen.move_list.clone();
    let time = std::time::Instant::now();

    for moves in moves_list.iter() {
        let a = board.make_move(*moves);
        if a.is_none() {
            continue;
        }
        let side = board.get_side_to_move();
        if board.is_king_checked(!side, gen) {
            board.unmake_move(a.unwrap());
            continue;
        }

        let cummutative_nodes = *nodes;

        perft_driver(depth - 1, board, gen, nodes);

        let old_nodes = *nodes - cummutative_nodes;

        board.unmake_move(a.unwrap());

        println!("{} {}", moves, old_nodes);
    }
    println!("\n{nodes}");
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kelp::board::fen::{Fen, FenParse};
    use crate::kelp::kelp_core::lookup_table::LookupTable;

    #[test]
    fn start_pos_test() {
        let mut board = Board::parse(Fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ".to_string(),
        ))
        .unwrap();
        let mut table = LookupTable::new();
        table.populate();
    }
}
