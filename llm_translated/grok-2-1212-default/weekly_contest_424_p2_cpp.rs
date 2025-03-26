use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: &Vec<i32>, q: &Vec<Vec<i32>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        for query in q {
            v[query[0] as usize] += 1;
            if query[1] as usize + 1 < v.len() {
                v[query[1] as usize + 1] -= 1;
            }
        }
        for i in 1..nums.len() {
            v[i] += v[i - 1];
        }
        for i in 0..nums.len() {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        true
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let n: usize = lines.next().unwrap()?.parse().unwrap();

    // Read the nums array
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the number of queries
    let m: usize = lines.next().unwrap()?.parse().unwrap();

    // Read the queries
    let mut queries = Vec::new();
    for _ in 0..m {
        let query: Vec<i32> = lines.next().unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    if Solution::is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}