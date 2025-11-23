use colored::Colorize;

const WHITE_KING_SYMBOL: char = '\u{2654}';
const WHITE_QUEEN_SYMBOL: char = '\u{2655}';
const WHITE_ROOK_SYMBOL: char = '\u{2656}';
const WHITE_BISHOP_SYMBOL: char = '\u{2657}';
const WHITE_KNIGHT_SYMBOL: char = '\u{2658}';
const WHITE_PAWN_SYMBOL: char = '\u{2659}';

const BLACK_KING_SYMBOL: char = '\u{265A}';
const BLACK_QUEEN_SYMBOL: char = '\u{265B}';
const BLACK_ROOK_SYMBOL: char = '\u{265C}';
const BLACK_BISHOP_SYMBOL: char = '\u{265D}';
const BLACK_KNIGHT_SYMBOL: char = '\u{265E}';
const BLACK_PAWN_SYMBOL: char = '\u{265F}';

pub fn print_possible_moves(possible_moves: u64, position_of_piece: u64) {
    // Start at the 64th square
    let mut current_square = 0b1u64;

    println!("   ---------------------------------");
    let mut board = String::new();
    for i in 1..=64 {
        if possible_moves & current_square != 0 {
            board = " * ".green().to_string() + "|" + board.as_str();
        } else if position_of_piece & current_square != 0 {
            board = " * ".red().to_string() + "|" + board.as_str();
        } else {
            board = "   |".to_string() + board.as_str();
        }

        // Print the row
        if i % 8 == 0 {
            let row_number = 8 - (i / 8 - 1);
            println!("{}  |{}", row_number, board);
            board = String::new();
        }

        current_square <<= 1;
    }

    println!("   ---------------------------------");
    println!("     a   b   c   d   e   f   g   h  ");
}
