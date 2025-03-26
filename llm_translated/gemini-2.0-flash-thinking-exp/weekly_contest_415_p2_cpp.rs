use std::io;
use std::str::SplitWhitespace;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_numbers_from_line(line: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut split = line.split_whitespace();
    while let Some(token) = split.next() {
        if let Ok(num) = token.parse::<i32>() {
            numbers.push(num);
        }
    }
    numbers
}

fn read_two_numbers_from_line(line: &str) -> (usize, usize) {
    let numbers = read_numbers_from_line(line);
    (numbers[0] as usize, numbers[1] as usize)
}

struct Solution {}

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut f: [i64; 5] = [0; 5];
        for i in 1..5 {
            f[i] = i64::min_value() / 2; // Equivalent to LLONG_MIN / 2 in C++
        }
        for &y in b.iter() {
            for j in (0..=3).rev() {
                f[j + 1] = f[j + 1].max(f[j] + (a[j] as i64) * (y as i64));
            }
        }
        f[4]
    }
}

fn main() {
    let line1 = read_line();
    let (a_size, b_size) = read_two_numbers_from_line(&line1);

    let line2 = read_line();
    let a = read_numbers_from_line(&line2);

    let line3 = read_line();
    let b = read_numbers_from_line(&line3);

    let solution = Solution {};
    println!("{}", solution.max_score(a, b));
}