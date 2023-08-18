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
