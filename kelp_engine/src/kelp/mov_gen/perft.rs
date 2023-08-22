use crate::kelp::board::board::Board;
use crate::kelp::mov_gen::generator::MovGen;

pub fn perft_driver(depth: u16, board: &mut Board, gen: &mut MovGen, nodes: &mut u64) {
    if depth == 0 {
        *nodes += 1;
        return;
    }

    let side = board.get_side_to_move();
    gen.generate_moves(board);
    let moves_list = gen.move_list.clone();

    for &move_to_make in moves_list.iter() {
        let undo_info = board.make_move(move_to_make, false);
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
        let a = board.make_move(*moves, false);
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
    use super::perft_driver;
    use crate::kelp::board::board::Board;
    use crate::kelp::board::fen::{Fen, FenParse};
    use crate::kelp::kelp_core::lookup_table::LookupTable;
    use crate::kelp::mov_gen::generator::MovGen;

    fn test_by_depth(depth: u16, fen: String, expected: u64) {
        let mut board = Board::parse(Fen(fen)).unwrap();
        let mut table = LookupTable::new();
        table.populate();
        let mut gen = MovGen::new(&table);
        let mut nodes = 0;
        perft_driver(depth, &mut board, &mut gen, &mut nodes);
        assert_eq!(nodes, expected);
    }

    #[test]
    fn start_pos_test() {
        test_by_depth(
            1,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            20,
        );
        test_by_depth(
            2,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            400,
        );
        test_by_depth(
            3,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            8902,
        );
        test_by_depth(
            4,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            197281,
        );
        test_by_depth(
            5,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            4865609,
        );
    }

    #[test]
    fn kiwipete_pos_test() {
        test_by_depth(
            1,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ".to_string(),
            48,
        );
        test_by_depth(
            2,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ".to_string(),
            2039,
        );
        test_by_depth(
            3,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ".to_string(),
            97862,
        );
        test_by_depth(
            4,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ".to_string(),
            4085603,
        );
        #[cfg(feature = "long")]
        test_by_depth(
            5,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ".to_string(),
            193690690,
        );
    }

    #[test]
    fn pos_3_pos_test() {
        test_by_depth(1, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".to_string(), 14);
        test_by_depth(2, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".to_string(), 191);
        test_by_depth(
            3,
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".to_string(),
            2812,
        );
        test_by_depth(
            4,
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".to_string(),
            43238,
        );
        test_by_depth(
            5,
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".to_string(),
            674624,
        );
    }

    #[test]
    fn pos_4_pos_test() {
        test_by_depth(
            1,
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
            6,
        );
        test_by_depth(
            2,
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
            264,
        );
        test_by_depth(
            3,
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
            9467,
        );
        test_by_depth(
            4,
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
            422333,
        );
    }

    #[test]
    fn pos_4_mirror_pos_test() {
        test_by_depth(
            1,
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ".to_string(),
            6,
        );
        test_by_depth(
            2,
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ".to_string(),
            264,
        );
        test_by_depth(
            3,
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ".to_string(),
            9467,
        );
        test_by_depth(
            4,
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ".to_string(),
            422333,
        );
    }

    fn pos_5_pos_test() {
        test_by_depth(
            1,
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
            44,
        );
        test_by_depth(
            2,
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
            1486,
        );
        test_by_depth(
            3,
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
            62379,
        );
        test_by_depth(
            4,
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
            2103487,
        );
        test_by_depth(
            5,
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
            89941194,
        );
    }

    #[test]
    fn steven_edwards_pos_test() {
        // pos 6
        test_by_depth(
            1,
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
            46,
        );
        test_by_depth(
            2,
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
            2079,
        );
        test_by_depth(
            3,
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
            89890,
        );
        test_by_depth(
            4,
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
            3894594,
        );
    }
}
