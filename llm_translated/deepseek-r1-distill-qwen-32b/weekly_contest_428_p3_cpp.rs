use std::io;

struct Solution;

impl Solution {
    fn beautiful_splits(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        
        // Initialize LCP table with (n+1) x (n+1) zeros
        let mut lcp = vec![vec![0; n + 1]; n + 1];
        
        // Compute LCP table
        for i in (0..n).rev() {
            for j in (i..n).rev() {
                if nums[i] == nums[j] {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }
        
        let mut ans = 0;
        
        // Iterate over all possible splits
        for i in 1..n - 1 {
            for j in i + 1..n {
                if i <= j - i {
                    if lcp[0][i] >= i || lcp[i][j] >= (j - i) {
                        ans += 1;
                    }
                }
            }
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut tokens = input.split_whitespace();
    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let nums: Vec<i32> = tokens.map(|t| t.parse().unwrap()).collect();
    
    let solution = Solution;
    let result = solution.beautiful_splits(&nums);
    println!("{}", result);
    
    Ok(())
}