use crate::kelp::board::board::Board;
use crate::kelp::board::moves::MoveType;
use crate::kelp::mov_gen::generator::MovGen;

pub struct Perft {
    pub nodes: u64,
    pub captures: u64,
    pub en_passants: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64, // TODO: Add checks
    // checkmates: u64, //TODO: Add checkmate
}

impl Default for Perft {
    fn default() -> Self {
        Perft {
            nodes: 0,
            captures: 0,
            en_passants: 0,
            castles: 0,
            promotions: 0,
            checks: 0,
        }
    }
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
pub fn perft(depth: u16, board: &mut Board, gen: &mut MovGen, pft: &mut Perft) {
    if depth == 0 {
        pft.nodes += 1;
        return;
    }
    use MoveType::*;

    let side = board.info.turn;
    gen.generate_moves(side, board);
    let moves_list = gen.move_list.clone();

    for &mov in moves_list.iter() {
        let move_history = board.make_move(mov, false);

        if board.is_king_checked(side, gen) {
            pft.checks += 1;
        }

        if !board.is_king_checked(side, gen) {
            if move_history.is_none() {
                continue;
            }
            if mov.capture.is_some() {
                pft.captures += 1;
            }
            let type_move = match mov.move_type {
                EnPassant(_) => {
                    1
                },
                Castle(_) => {
                    2
                },
                Promotion(_) => {
                    3
                },
                _ => 4,
            };
            if type_move == 1 {
                pft.en_passants += 1;
            } else if type_move == 2 {
                pft.castles += 1;
            } else if type_move == 3 {
                pft.promotions += 1;
            }
            perft(depth - 1, board, gen, pft);

        }
        if move_history.is_none() {
            continue;
        }
        board.unmake_move(move_history.unwrap());
    }
}


#[cfg(test)]
mod tests {
    use crate::kelp::board::fen::{FenParse, Fen};
    use crate::kelp::kelp_core::lookup_table::LookupTable;
    use super::*;

    #[test]
    fn start_pos_test() {
        maybe_grow(1024 * 1024, 1024 * 1024, || {
            let mut board = Board::parse(Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ".to_string())).unwrap();
            let mut table = LookupTable::new();
            table.populate();
            let mut gen = MovGen::new(&mut table);
            let mut pft = Perft::default();
            let time = std::time::Instant::now();
            perft(5, &mut board, &mut gen, &mut pft);
            println!("Time: {:?}", time.elapsed());
            println!("Nodes: {}", pft.nodes);
            println!("Captures: {}", pft.captures);
            println!("En Passants: {}", pft.en_passants);
            println!("Castles: {}", pft.castles);
            println!("Promotions: {}", pft.promotions);
            assert_eq!(1, 1); // just for now to see the output
        })
    }
}
