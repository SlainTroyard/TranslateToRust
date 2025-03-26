use std::collections::HashMap;
use std::io;
use std::num::Wrapping;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf: HashMap<i32, i32> = HashMap::new();

        // Populate the suf map
        for i in 4..n - 2 {
            let c = nums[i];
            for j in i + 2..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = (d / g) << 16 | (c / g);
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
                let key = (a / g) << 16 | (b / g);
                ans += *suf.get(&key).unwrap_or(&0) as i64;
            }

            let c = nums[i + 2];
            for j in i + 4..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                *suf.entry(key).or_insert(0) -= 1;
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    let result = Solution::number_of_subsequences(nums);
    println!("{}", result);
}