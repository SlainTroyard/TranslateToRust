use std::io;

struct Solution {}

impl Solution {
    pub fn beautiful_splits(&self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return 0;
        }
        let mut lcp = vec![vec![0; n + 1]; n + 1];
        for i in (0..n).rev() {
            for j in (i..n).rev() {
                if nums[i] == nums[j] {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }

        let mut ans = 0;
        for i in 1..n - 1 {
            for j in i + 1..n {
                // Check if the split satisfies the beautiful condition
                if i as i32 <= (j - i) as i32 && lcp[0][i] >= i as i32 || lcp[i][j] >= (j - i) as i32 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let solution = Solution {};

    // Input: size of array and array elements
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    // Calculate and print the result
    let result = solution.beautiful_splits(nums);
    println!("{}", result);
}