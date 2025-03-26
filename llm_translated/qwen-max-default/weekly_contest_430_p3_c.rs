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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to read n");
    let mut nums: Vec<i32> = Vec::with_capacity(n);

    // Read the elements
    for line in lines.by_ref().take(n) {
        let num: i32 = line.unwrap().trim().parse().expect("Failed to read num");
        nums.push(num);
    }

    let result = number_of_subsequences(&nums);
    println!("{}", result);
}

fn number_of_subsequences(nums: &[i32]) -> i64 {
    let mut hash_table: HashMap<i32, i32> = HashMap::new();

    let mut ans: i64 = 0;

    for i in 4..nums.len() - 2 {
        let c = nums[i];
        for j in (i + 2)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash_table.entry(key).or_insert(0) += 1;
        }
    }

    for i in 2..nums.len() - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            if let Some(&count) = hash_table.get(&key) {
                ans += count as i64;
            }
        }

        let c = nums[i + 2];
        for j in (i + 4)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash_table.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}