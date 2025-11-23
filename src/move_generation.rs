use crate::constants;

pub mod rook {
    use crate::constants;
    use crate::move_generation::Move;

    pub fn generate_moves(current_position: u64) -> u64 {
        let mut possible_moves = 0u64;

        for amount_to_move in 1..constants::BOARD_SIZE {
            possible_moves |= current_position.move_up_by(amount_to_move)
                | current_position.move_down_by(amount_to_move);
        }

        // Only generate moves to the right if it's not already on the h-file.
        if current_position & constants::H_FILE == 0 {
            for amount_to_move in 1..constants::BOARD_SIZE {
                let right = current_position.move_right_by(amount_to_move);

                possible_moves |= right;

                // Short circuit if the piece is now off of the board OR this the piece is now on the
                // h-file. Meaning it can't go further to the right.
                if right == 0 || right & constants::H_FILE != 0 {
                    break;
                }
            }
        }

        // Only generate moves to the left if it's not already on the a-file.
        if current_position & constants::A_FILE == 0 {
            for amount_to_move in 1..constants::BOARD_SIZE {
                let left = current_position.move_left_by(amount_to_move);

                possible_moves |= left;

                // Short circuit if the piece is now off of the board OR this is piece is now the
                // a-file. Meaning it can't go further to the left.
                if left == 0 || left & constants::A_FILE != 0 {
                    break;
                }
            }
        }

        possible_moves
    }
}

trait Move {
    fn move_up_by(self, amount: u8) -> Self;
    fn move_down_by(self, amount: u8) -> Self;
    fn move_left_by(self, amount: u8) -> Self;
    fn move_right_by(self, amount: u8) -> Self;
    fn move_diagonally_up_left_by(self, amount: u8) -> Self;
    fn move_diagonally_up_right_by(self, amount: u8) -> Self;
    fn move_diagonally_down_left_by(self, amount: u8) -> Self;
    fn move_diagonally_down_right_by(self, amount: u8) -> Self;
}

impl Move for u64 {
    fn move_up_by(self, amount: u8) -> Self {
        self >> constants::BOARD_SIZE * amount
    }

    fn move_down_by(self, amount: u8) -> Self {
        self << constants::BOARD_SIZE * amount
    }

    fn move_left_by(self, amount: u8) -> Self {
        self << 1 * amount
    }

    fn move_right_by(self, amount: u8) -> Self {
        self >> 1 * amount
    }

    fn move_diagonally_up_left_by(self, amount: u8) -> Self {
        self >> constants::BOARD_SIZE * amount - amount
    }

    fn move_diagonally_up_right_by(self, amount: u8) -> Self {
        self >> constants::BOARD_SIZE * amount + amount
    }

    fn move_diagonally_down_left_by(self, amount: u8) -> Self {
        self << constants::BOARD_SIZE * amount - amount
    }

    fn move_diagonally_down_right_by(self, amount: u8) -> Self {
        self << constants::BOARD_SIZE * amount + amount
    }
}
