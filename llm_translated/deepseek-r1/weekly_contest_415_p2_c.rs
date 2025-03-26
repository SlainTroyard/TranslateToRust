use std::io::{self, Read};

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    // Initialize DP table with i32::MIN (matching original C code's INT_MIN)
    let mut dp = vec![vec![i64::from(i32::MIN); 5]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=4 {
            // Inherit previous state
            dp[i][j] = dp[i-1][j];
            
            // Consider adding current element if we're building a sequence
            if j > 0 {
                let product = a[j-1] as i64 * b[i-1] as i64;
                let option = dp[i-1][j-1].saturating_add(product);
                dp[i][j] = dp[i][j].max(option);
            }
        }
    }

    dp[n][4]
}

fn main() {
    // Read entire input and parse as i32 tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    // Parse array sizes and elements following original C input format
    let a_size = tokens.next().unwrap() as usize;
    let b_size = tokens.next().unwrap() as usize;
    let a: Vec<_> = (0..a_size).map(|_| tokens.next().unwrap()).collect();
    let b: Vec<_> = (0..b_size).map(|_| tokens.next().unwrap()).collect();

    println!("{}", max_score(&a, &b));
}