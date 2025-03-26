use std::io::{self, BufRead};

fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut f = vec![vec![0; 9]; 10]; // Equivalent to array<array<int, 9>, 10> in C++

    for c in s.chars() {
        let d = c.to_digit(10).unwrap() as usize;
        for m in 1..10 {
            let mut nf = vec![0; 9];
            nf[d % m] = 1;
            for rem in 0..m {
                nf[(rem * 10 + d) % m] += f[m][rem];
            }
            f[m] = nf;
        }
        ans += f[d][0] as i64;
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap(); // Read the first line as the input string

    let result = count_substrings(&s);
    println!("{}", result); // Print the result
}