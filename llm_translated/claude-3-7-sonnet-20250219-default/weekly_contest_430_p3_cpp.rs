use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;

// Reads a line from stdin and parses it into the target type
fn read_line<T: std::str::FromStr>() -> Result<T, ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<T>().map_err(|_| "Parse error".parse::<i32>().unwrap_err())
}

// Reads a line and parses it into a vector of integers
fn read_vec() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

struct Solution;

impl Solution {
    pub fn number_of_subsequences(nums: &Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf: HashMap<i32, i32> = HashMap::new();
        
        // Build suffix map
        for i in 4..n - 2 {
            let c = nums[i];
            for j in i + 2..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans: i64 = 0;
        for i in 2..n - 4 {
            let b = nums[i];
            
            // Calculate answers for current position
            for j in 0..i - 1 {
                let a = nums[j];
                let g = gcd(a, b);
                let key = ((a / g) << 16) | (b / g);
                ans += *suf.get(&key).unwrap_or(&0) as i64;
            }
            
            // Update suffix map
            let c = nums[i + 2];
            for j in i + 4..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                if let Some(count) = suf.get_mut(&key) {
                    *count -= 1;
                }
            }
        }
        
        ans
    }
}

fn main() {
    // Input the size of the array
    let n: usize = read_line().expect("Failed to read n");
    
    // Input the elements of the array
    let nums: Vec<i32> = read_vec();
    
    // Call the number_of_subsequences function
    let result = Solution::number_of_subsequences(&nums);
    
    // Output the result
    println!("{}", result);
}