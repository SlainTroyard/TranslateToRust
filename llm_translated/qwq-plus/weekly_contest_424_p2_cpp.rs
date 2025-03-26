use std::io;

struct Solution;

impl Solution {
    fn is_zero_array(&self, nums: &[i32], queries: &[Vec<i32>]) -> bool {
        let n = nums.len();
        let mut v = vec![0; n + 1];

        for query in queries {
            let start = query[0] as usize;
            let end = query[1] as usize;
            v[start] += 1;
            v[end + 1] -= 1;
        }

        // Compute prefix sums
        for i in 1..n {
            v[i] += v[i - 1];
        }

        // Check if all nums[i] - v[i] <= 0
        for i in 0..n {
            if nums[i] > v[i] {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid number"));

    // Read n and nums
    let n = tokens.next().expect("No n found");
    let n_usize = n as usize;
    let mut nums = Vec::with_capacity(n_usize);
    for _ in 0..n {
        nums.push(tokens.next().expect("Not enough numbers for nums"));
    }

    // Read m and queries
    let m = tokens.next().expect("No m found");
    let m_usize = m as usize;
    let mut queries = Vec::with_capacity(m_usize);
    for _ in 0..m {
        let a = tokens.next().expect("Missing query start");
        let b = tokens.next().expect("Missing query end");
        queries.push(vec![a, b]);
    }

    let sol = Solution {};
    let result = sol.is_zero_array(&nums, &queries);
    println!("{}", if result { "true" } else { "false" });
}