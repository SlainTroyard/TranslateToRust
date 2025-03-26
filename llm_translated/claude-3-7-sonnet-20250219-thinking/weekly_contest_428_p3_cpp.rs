use std::io::{self, BufRead};

fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    
    // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i:] and s[j:]
    let mut lcp = vec![vec![0; n + 1]; n + 1];
    
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }
    
    let mut ans = 0;
    for i in 1..n-1 {
        for j in i+1..n {
            // Check if the split satisfies the beautiful condition
            if i <= j - i && (lcp[0][i] >= i || lcp[i][j] >= j - i) {
                ans += 1;
            }
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse array size");
    
    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    let mut count = 0;
    
    while count < n {
        if let Some(Ok(line)) = lines.next() {
            for num in line.split_whitespace() {
                if let Ok(value) = num.parse::<i32>() {
                    nums.push(value);
                    count += 1;
                    if count == n {
                        break;
                    }
                }
            }
        }
    }
    
    // Calculate and print the result
    let result = beautiful_splits(&nums);
    println!("{}", result);
}