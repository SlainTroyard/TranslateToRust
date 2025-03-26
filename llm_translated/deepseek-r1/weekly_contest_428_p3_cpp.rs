use std::io::{self, Read};

fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }

    // Initialize LCP table with (n+1)x(n+1) dimensions
    let mut lcp = vec![vec![0; n + 1]; n + 1];
    
    // Populate LCP table from bottom-up
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
            // Else remains 0 as initialized
        }
    }

    let mut ans = 0;
    // Iterate all possible split points (i,j)
    for i in 1..n-1 {
        for j in i+1..n {
            // Check both beautiful split conditions
            let condition1 = i <= j - i && lcp[0][i] >= i;
            let condition2 = lcp[i][j] >= j - i;
            if condition1 || condition2 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse input size and numbers
    let n: usize = tokens
        .next()
        .expect("No input provided")
        .parse()
        .expect("Invalid number for n");
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid number in nums"))
        .collect();
    
    // Compute and print result
    let result = beautiful_splits(&nums);
    println!("{}", result);
}