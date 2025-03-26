use std::io;

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut v = vec![0; n + 1];
        
        for q in queries {
            let start = q[0] as usize;
            let end = q[1] as usize;
            v[start] += 1;
            if end + 1 <= n {
                v[end + 1] -= 1;
            }
        }
        
        for i in 1..n {
            v[i] += v[i - 1];
        }
        
        for i in 0..n {
            if nums[i] - v[i] > 0 {
                return false;
            }
        }
        
        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty());
    
    // Read n
    let n: usize = lines.next().expect("Missing n").parse().expect("Invalid n");
    
    // Read nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Missing nums")
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid num"))
        .collect();
    
    // Read m
    let m: usize = lines.next().expect("Missing m").parse().expect("Invalid m");
    
    // Read queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().expect("Missing query");
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid query"))
            .collect();
        queries.push(parts);
    }
    
    let sol = Solution;
    if sol.is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}