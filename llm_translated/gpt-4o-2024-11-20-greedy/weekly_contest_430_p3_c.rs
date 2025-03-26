use std::collections::HashMap;
use std::io::{self, BufRead};

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Function to calculate the number of subsequences
fn number_of_subsequences(nums: &[i32]) -> i64 {
    const HASH_SIZE: usize = 100_003;

    // Hash table to store key-value pairs
    let mut hash_table: HashMap<i32, i32> = HashMap::new();

    // Helper function to insert into the hash table
    fn hash_insert(hash_table: &mut HashMap<i32, i32>, key: i32, value: i32) {
        *hash_table.entry(key).or_insert(0) += value;
    }

    // Helper function to get a value from the hash table
    fn hash_get(hash_table: &HashMap<i32, i32>, key: i32) -> i32 {
        *hash_table.get(&key).unwrap_or(&0)
    }

    let mut ans: i64 = 0;

    // First pass: populate the hash table
    for i in 4..nums.len() - 2 {
        let c = nums[i];
        for j in (i + 2)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            hash_insert(&mut hash_table, key, 1);
        }
    }

    // Second pass: calculate the result and update the hash table
    for i in 2..nums.len() - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            ans += hash_get(&hash_table, key) as i64;
        }

        let c = nums[i + 2];
        for j in (i + 4)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            hash_insert(&mut hash_table, key, -1);
        }
    }

    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected a number");

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected a number"))
        .collect();

    // Ensure the input size matches the provided size
    assert_eq!(nums.len(), n, "Input size does not match the provided size");

    // Calculate the result
    let result = number_of_subsequences(&nums);

    // Print the result
    println!("{}", result);
}