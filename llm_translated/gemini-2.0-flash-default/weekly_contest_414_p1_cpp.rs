// Problem: Weekly Contest 414 Problem 1
use std::io;

struct Solution {}

impl Solution {
    fn convert_date_to_binary(&self, date: String) -> String {
        let bin = |x: i32| -> String {
            let s = format!("{:b}", x);
            s
        };

        let year = date[0..4].parse::<i32>().unwrap();
        let month = date[5..7].parse::<i32>().unwrap();
        let day = date[8..10].parse::<i32>().unwrap();

        format!("{}-{}-{}", bin(year), bin(month), bin(day))
    }
}

fn main() {
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = date.trim().to_string(); // Remove trailing newline

    let sol = Solution {};
    println!("{}", sol.convert_date_to_binary(date));
}