use std::collections::HashMap;
use std::io::{self, BufRead};

// Function to calculate gcd (greatest common divisor)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Function to solve the number of subsequences problem
fn number_of_subsequences(nums: &[i32]) -> i64 {
    const HASH_SIZE: usize = 100003;
    let mut hash_table: Vec<HashMap<i32, i32>> = vec![HashMap::new(); HASH_SIZE];

    // Clear hash table
    fn hash_clear(hash_table: &mut Vec<HashMap<i32, i32>>) {
        for map in hash_table.iter_mut() {
            map.clear();
        }
    }

    // Get the hash key for a given value
    fn hash_key(key: i32) -> usize {
        (key as usize) % HASH_SIZE
    }

    // Insert into hash table
    fn hash_insert(hash_table: &mut Vec<HashMap<i32, i32>>, key: i32, value: i32) {
        let hash_index = hash_key(key);
        let entry = hash_table[hash_index].entry(key).or_insert(0);
        *entry += value;
    }

    // Get value from hash table
    fn hash_get(hash_table: &Vec<HashMap<i32, i32>>, key: i32) -> i32 {
        let hash_index = hash_key(key);
        *hash_table[hash_index].get(&key).unwrap_or(&0)
    }

    hash_clear(&mut hash_table);

    let mut ans: i64 = 0;

    // Fill hash table with values
    for i in 4..nums.len() - 2 {
        let c = nums[i];
        for j in (i + 2)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            hash_insert(&mut hash_table, key, 1);
        }
    }

    // Match keys from hash table to calculate result
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

    hash_clear(&mut hash_table);

    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let n: usize = lines
        .next()
        .expect("Expected first line of input")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse integer");

    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected second line of input")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Failed to parse integer"))
        .collect();

    assert_eq!(n, nums.len(), "Input size mismatch");

    // Calculate the number of subsequences
    let result = number_of_subsequences(&nums);

    // Print the result
    println!("{}", result);
}