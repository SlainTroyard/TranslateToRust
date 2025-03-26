use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;

struct Solution;

impl Solution {
    pub fn number_of_subsequences(nums: &[i32]) -> i64 {
        let n = nums.len();
        let mut suf: HashMap<i32, i32> = HashMap::new();
        
        // Build suffix map for pairs (c, d)
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
            
            // Count valid subsequences with current b
            for j in 0..i - 1 {
                let a = nums[j];
                let g = gcd(a, b);
                let key = ((a / g) << 16) | (b / g);
                ans += *suf.get(&key).unwrap_or(&0) as i64;
            }
            
            // Update suffix map by removing pairs that will no longer be valid
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

// Greatest common divisor function
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn parse_int(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn main() -> io::Result<()> {
    // Input the size of the array
    let n = parse_int(&read_line()).expect("Invalid input for array size");
    
    // Input the elements of the array
    let nums_input = read_line();
    let nums: Vec<i32> = nums_input
        .split_whitespace()
        .map(|s| parse_int(s).expect("Invalid input for array element"))
        .collect();
    
    // Call the numberOfSubsequences function
    let result = Solution::number_of_subsequences(&nums);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}