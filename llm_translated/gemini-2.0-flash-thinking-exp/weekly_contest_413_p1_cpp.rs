use std::io;

struct Solution {}

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        // Convert coordinate characters to integers for calculation
        let c1_file = coordinate1.as_bytes()[0] as i32; // 'a' is 97, 'b' is 98, etc.
        let c1_rank = coordinate1.as_bytes()[1] as i32; // '1' is 49, '2' is 50, etc.
        let c2_file = coordinate2.as_bytes()[0] as i32;
        let c2_rank = coordinate2.as_bytes()[1] as i32;

        // Calculate the difference and check if the sum of differences is even
        (c1_file - c2_file + c1_rank - c2_rank) % 2 == 0
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");

    // Split the input line by whitespace to get two coordinates
    let coordinates: Vec<&str> = input_line.trim().split_whitespace().collect();

    // Ensure we have exactly two coordinates
    if coordinates.len() != 2 {
        eprintln!("Error: Expected two coordinates as input.");
        return;
    }

    let coordinate1 = coordinates[0].to_string();
    let coordinate2 = coordinates[1].to_string();

    let sol = Solution {};
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}