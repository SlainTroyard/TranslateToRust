use std::collections::HashMap;
use std::io::{self, BufRead};

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn number_of_subsequences(nums: &[i32]) -> i64 {
    let mut hash_map: HashMap<i32, i64> = HashMap::new();
    let mut ans = 0;

    // First pass: populate the hash map with (d/g, c/g) pairs
    for i in 4..nums.len() - 2 {
        let c = nums[i];
        for j in i + 2..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash_map.entry(key).or_insert(0) += 1;
        }
    }

    // Second pass: calculate the answer and update the hash map
    for i in 2..nums.len() - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            ans += hash_map.get(&key).unwrap_or(&0);
        }

        let c = nums[i + 2];
        for j in i + 4..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash_map.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line to get the array of numbers
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = number_of_subsequences(&nums);

    // Print the result
    println!("{}", result);
}