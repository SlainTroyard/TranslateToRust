use std::io::{self, BufRead};

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0;
    let mut f = [[0; 9]; 10];
    
    for c in s.chars() {
        let d = c.to_digit(10).unwrap() as usize;
        for m in 1..10 {
            let mut nf = [0; 9];
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
    // Read the input string from stdin
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Trim any leading or trailing whitespace

    // Create an instance of the Solution and call the count_substrings method
    let sol = count_substrings(input);
    
    // Output the result to stdout
    println!("{}", sol);
}