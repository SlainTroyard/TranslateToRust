use std::collections::HashMap;
use std::io;
use std::str::SplitWhitespace;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution {}

impl Solution {
    pub fn number_of_subsequences(&self, nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf: HashMap<i32, i32> = HashMap::new();

        // First loop to populate 'suf' map
        for i in 4..(n - 2) {
            let c = nums[i];
            for j in (i + 2)..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans: i64 = 0;

        // Second loop to calculate 'ans' and update 'suf'
        for i in 2..(n - 4) {
            let b = nums[i];
            for j in 0..(i - 1) {
                let a = nums[j];
                let g = gcd(a, b);
                let key = (a / g) << 16 | (b / g);
                ans += suf.get(&key).unwrap_or(&0) as i64;
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

fn parse_input(input: SplitWhitespace) -> Vec<i32> {
    input.map(|s| s.parse::<i32>().unwrap()).collect()
}

fn main() {
    let solution = Solution {};
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nums: Vec<i32> = parse_input(input_line.split_whitespace());

    let result = solution.number_of_subsequences(nums);
    println!("{}", result);
}