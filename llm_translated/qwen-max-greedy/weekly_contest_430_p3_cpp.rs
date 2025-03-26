use std::collections::HashMap;
use std::io::{self, BufRead};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    fn number_of_subsequences(nums: &Vec<usize>) -> usize {
        let n = nums.len();
        let mut suf: HashMap<usize, isize> = HashMap::new();

        for i in 4..n - 2 {
            let c = nums[i];
            for j in (i + 2)..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans = 0;
        for i in 2..n - 4 {
            let b = nums[i];
            for j in 0..i - 1 {
                let a = nums[j];
                let g = gcd(a, b);
                let key = (a / g) << 16 | (b / g);
                if let Some(&count) = suf.get(&key) {
                    ans += count as usize;
                }
            }
            let c = nums[i + 2];
            for j in (i + 4)..n {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the elements of the array
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Call the number_of_subsequences function
    let result = Solution::number_of_subsequences(&nums);

    // Output the result
    println!("{}", result);
}