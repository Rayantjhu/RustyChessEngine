mod constants;
mod move_generation;
mod print_board;
mod read_board;

fn main() {
    print_all_piece_moves(move_generation::bishop::generate_moves);
    print_all_piece_moves(move_generation::rook::generate_moves);
    print_all_piece_moves(move_generation::queen::generate_moves);
}

fn print_all_piece_moves(move_generator: fn(u64) -> u64) {
    let starting_position = 0b1u64;

    for i in 0..64 {
        let current_position = starting_position << i;
        let piece_possible_moves = move_generator(current_position);
        print_board::print_possible_moves(piece_possible_moves, current_position);
    }
}
