use regex::Regex;

/// Helper function to create the binary representation of a specific coordinate. Mostly useful for
/// debugging and setting up positions.
pub fn coordinates_to_binary(coordinates: &str) -> Option<u64> {
    let regex = Regex::new(r"^[a-hA-H][1-8]$").unwrap();

    if !regex.is_match(coordinates) {
        println!("Coordinates are not in valid format. It should be [a-h][1-8]. For example: a2");
        return None;
    }

    let lowered = coordinates.to_lowercase();
    let mut chars = lowered.chars();
    let column = match chars.next() {
        Some(c) => {
            // The character 'a' starts at 97. So to make sure 'a' is the first column, subtract 97.
            // the following characters will have the correct column_number.
            u32::from(c) - 97
        }
        None => {
            println!("Unable to find column (a-h)");
            return None;
        }
    };
    let row = match chars.next() {
        Some(r) => match r.to_digit(10) {
            Some(digit) => digit,
            None => {
                println!("Row is not a digit");
                return None;
            }
        },
        None => {
            println!("Unable to find row");
            return None;
        }
    };

    // Shift to the right by 8 times the number of rows (which is subtracted by 8, as when the row
    // is 8, it shouldn't go down a single row). Then shift by the number of columns. Finally shift
    // one less time as we shift max 63 times as we already have the bit on the last square.
    let binary = 0b1u64 << (8 - row) * 8 + (8 - column) - 1;
    println!("{:#066b}", binary);
    Some(binary)
}
