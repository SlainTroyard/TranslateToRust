use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();
    let result = number_of_subsequences(&nums);
    println!("{}", result);
}

fn number_of_subsequences(nums: &[i32]) -> i64 {
    let mut hash_map = HashMap::new();
    let nums_size = nums.len();
    let mut ans = 0i64;

    // Populate hash map with (d/gcd(c,d), c/gcd(c,d)) pairs from valid (c,d) pairs
    for i in 4..(nums_size - 2) {
        let c = nums[i];
        for j in (i + 2)..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) as u32) << 16 | (c / g) as u32;
            *hash_map.entry(key).or_insert(0) += 1;
        }
    }

    // Calculate valid subsequences and adjust hash map for subsequent iterations
    for i in 2..(nums_size - 4) {
        let b = nums[i];
        // Query for (a/gcd(a,b), b/gcd(a,b)) pairs and accumulate results
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) as u32) << 16 | (b / g) as u32;
            ans += hash_map.get(&key).copied().unwrap_or(0);
        }

        // Remove (d/gcd(c,d), c/gcd(c,d)) pairs that are no longer valid
        let c = nums[i + 2];
        for j in (i + 4)..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) as u32) << 16 | (c / g) as u32;
            *hash_map.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}

/// Helper function to compute greatest common divisor using Euclidean algorithm
fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}