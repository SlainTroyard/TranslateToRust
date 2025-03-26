use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn convert_date_to_binary(date: &str) -> String {
        let bin = |x: i32| -> String {
            let s = format!("{:b}", x);
            s.trim_start_matches('0').to_string()
        };

        let year = &date[0..4];
        let month = &date[5..7];
        let day = &date[8..10];

        format!(
            "{}-{}-{}",
            bin(year.parse().unwrap()),
            bin(month.parse().unwrap()),
            bin(day.parse().unwrap())
        )
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(date)) = lines.next() {
        let result = Solution::convert_date_to_binary(&date);
        println!("{}", result);
    }

    Ok(())
}