use std::collections::HashMap;
use std::io::{self, BufRead};
use num::integer::gcd;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    // Call the numberOfSubsequences function
    let result = numberOf_subsequences(&nums);

    // Output the result
    println!("{}", result);
}

fn number_of_subsequences(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut suf: HashMap<u32, i32> = HashMap::new();

    // Populate the suf map
    for i in 4..n - 2 {
        let c = nums[i];
        for j in i + 2..n {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) as u32) << 16 | (c / g) as u32;
            *suf.entry(key).or_insert(0) += 1;
        }
    }

    let mut ans: i64 = 0;

    // Calculate the answer
    for i in 2..n - 4 {
        let b = nums[i];
        for j in 0..i - 1 {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) as u32) << 16 | (b / g) as u32;
            ans += *suf.get(&key).unwrap_or(&0) as i64;
        }

        let c = nums[i + 2];
        for j in i + 4..n {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) as u32) << 16 | (c / g) as u32;
            *suf.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}