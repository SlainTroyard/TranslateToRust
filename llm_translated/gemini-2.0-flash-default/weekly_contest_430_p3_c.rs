use std::collections::HashMap;
use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    let mut a_mut = a;
    let mut b_mut = b;
    while b_mut != 0 {
        let temp = b_mut;
        b_mut = a_mut % b_mut;
        a_mut = temp;
    }
    a_mut
}

fn number_of_subsequences(nums: &[i32]) -> i64 {
    let mut hash_map: HashMap<i64, i32> = HashMap::new();

    let mut ans: i64 = 0;

    for i in 4..(nums.len() - 2) {
        let c = nums[i];
        for j in (i + 2)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = (((d / g) as i64) << 16) | ((c / g) as i64);
            *hash_map.entry(key).or_insert(0) += 1;
        }
    }

    for i in 2..(nums.len() - 4) {
        let b = nums[i];
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            let key = (((a / g) as i64) << 16) | ((b / g) as i64);
            ans += hash_map.get(&key).copied().unwrap_or(0) as i64;
        }

        let c = nums[i + 2];
        for j in (i + 4)..nums.len() {
            let d = nums[j];
            let g = gcd(c, d);
            let key = (((d / g) as i64) << 16) | ((c / g) as i64);
            *hash_map.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let result = number_of_subsequences(&nums);
    println!("{}", result);
}