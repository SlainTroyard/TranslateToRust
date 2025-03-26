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

fn number_of_subsequences(nums: &Vec<i32>) -> i64 {
    let mut hash_table: HashMap<i32, i32> = HashMap::new();

    // Helper function to insert or update the value in the hash table
    fn hash_insert(hash_table: &mut HashMap<i32, i32>, key: i32, value: i32) {
        *hash_table.entry(key).or_insert(0) += value;
    }

    // Helper function to get the value from the hash table
    fn hash_get(hash_table: &HashMap<i32, i32>, key: i32) -> i32 {
        *hash_table.get(&key).unwrap_or(&0)
    }

    let nums_size = nums.len();
    let mut ans: i64 = 0;

    for i in 4..nums_size - 2 {
        let c = nums[i];
        for j in (i + 2)..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            hash_insert(&mut hash_table, key, 1);
        }
    }

    for i in 2..nums_size - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            ans += hash_get(&hash_table, key) as i64;
        }

        let c = nums[i + 2];
        for j in (i + 4)..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            hash_insert(&mut hash_table, key, -1);
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().expect("Failed to parse n");
        let mut nums: Vec<i32> = Vec::with_capacity(n);

        for _ in 0..n {
            if let Some(Ok(line)) = lines.next() {
                let num: i32 = line.trim().parse().expect("Failed to parse number");
                nums.push(num);
            }
        }

        let result = number_of_subsequences(&nums);
        println!("{}", result);
    } else {
        eprintln!("No input provided");
    }
}