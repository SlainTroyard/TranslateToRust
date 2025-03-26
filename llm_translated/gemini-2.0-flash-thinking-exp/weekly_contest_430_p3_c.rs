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
    let mut hash_table: HashMap<i32, i32> = HashMap::new();
    let nums_size = nums.len();
    let mut ans: i64 = 0;

    for i in 4..nums_size - 2 {
        let c = nums[i];
        for j in i + 2..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash_table.entry(key).or_insert(0) += 1;
        }
    }

    for i in 2..nums_size - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            ans += hash_table.get(&key).copied().unwrap_or(0) as i64;
        }

        let c = nums[i + 2];
        for j in i + 4..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            *hash_table.entry(key).or_insert(0) -= 1;
        }
    }
    ans
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let n: usize = input_line.trim().parse().expect("Invalid input for n");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    if nums.len() != n {
        panic!("Input nums length does not match n");
    }

    let result = number_of_subsequences(&nums);
    println!("{}", result);
}