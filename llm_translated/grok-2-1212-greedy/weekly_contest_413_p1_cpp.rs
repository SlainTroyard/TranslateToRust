use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        let col1 = coordinate1.chars().next().unwrap() as i32 - 'a' as i32;
        let row1 = coordinate1.chars().nth(1).unwrap() as i32 - '1' as i32;
        let col2 = coordinate2.chars().next().unwrap() as i32 - 'a' as i32;
        let row2 = coordinate2.chars().nth(1).unwrap() as i32 - '1' as i32;

        (col1 - col2 + row1 - row2) % 2 == 0
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let coordinate1 = lines.next().unwrap()?;
    let coordinate2 = lines.next().unwrap()?;

    let sol = Solution;
    if sol.check_two_chessboards(&coordinate1, &coordinate2) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}