use std::io::{self, BufRead};
use std::cmp;

// Compute the greatest common divisor using the Euclidean algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

// Compute the least common multiple.
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Compute the maximum score as per the algorithm in the C code.
fn max_score(nums: &[i64]) -> i64 {
    let nums_size = nums.len();
    // Special case: if only one number then result is that number squared.
    if nums_size == 1 {
        return nums[0] * nums[0];
    }
    
    // Create vectors for lcms and gcds with capacity nums_size.
    let mut lcms = vec![0i64; nums_size];
    let mut gcds = vec![0i64; nums_size];
    
    // Initialize the last element with the last number.
    lcms[nums_size - 1] = nums[nums_size - 1];
    gcds[nums_size - 1] = nums[nums_size - 1];
    
    // Compute cumulative lcm and gcd arrays from right to left.
    for i in (0..nums_size - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }
    
    // Initialize answer with the product from first element.
    let mut ans = lcms[0] * gcds[0];
    ans = cmp::max(ans, lcms[1] * gcds[1]);
    
    // Pre-calculate cumulative values from left side.
    let mut pre_lcm = nums[0];
    let mut pre_gcd = nums[0];
    
    // Loop through the array, considering splits at different positions.
    for i in 1..nums_size - 1 {
        let candidate = lcm(pre_lcm, lcms[i + 1]) * gcd(pre_gcd, gcds[i + 1]);
        ans = cmp::max(candidate, ans);
        pre_lcm = lcm(pre_lcm, nums[i]);
        pre_gcd = gcd(pre_gcd, nums[i]);
    }
    ans = cmp::max(ans, pre_lcm * pre_gcd);
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read from standard input, handling multiple lines and potential multiple values per line.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first non-empty line and parse the number of elements.
    let nums_size: usize = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            // Skip empty lines.
            if line.trim().is_empty() {
                continue;
            }
            // Attempt to parse the first token.
            let mut parts = line.split_whitespace();
            if let Some(token) = parts.next() {
                let parsed = token.parse::<usize>()?;
                break parsed;
            }
        } else {
            // If no input is provided, return an error.
            return Err("No input provided".into());
        }
    };

    // Read nums_size numbers.
    let mut nums: Vec<i64> = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        // Get the next available line.
        if let Some(line) = lines.next() {
            let line = line?;
            // Split the line by whitespace and parse each number.
            for token in line.split_whitespace() {
                if nums.len() >= nums_size {
                    break;
                }
                let number = token.parse::<i64>()?;
                nums.push(number);
            }
        } else {
            break;
        }
    }

    // Compute the maximum score.
    let result = max_score(&nums);

    // Print the result exactly as in the original C code.
    println!("{}", result);
    
    Ok(())
}