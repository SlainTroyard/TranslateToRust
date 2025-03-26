use std::collections::HashMap;

struct Solution;

impl Solution {
    fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n < 4 {
            return 0;
        }

        let mut suf = HashMap::new();

        // First loop to build the suf map
        for i in 4..(n - 2) {
            let c = nums[i];
            for j in (i + 2)..n {
                let d = nums[j];
                let g = gcd(c, d);
                let c_reduced = c / g;
                let d_reduced = d / g;
                let key = (d_reduced, c_reduced);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans = 0;

        // Second loop to count and update
        for i in 2..(n - 4) {
            let b = nums[i];

            // Process a's
            for j in 0..(i - 1) {
                let a = nums[j];
                let g = gcd(a, b);
                let a_reduced = a / g;
                let b_reduced = b / g;
                let key = (a_reduced, b_reduced);
                ans += suf.get(&key).unwrap_or(&0);
            }

            // Process c and d's to update the map
            let c = nums[i + 2];
            for j in (i + 4)..n {
                let d = nums[j];
                let g = gcd(c, d);
                let c_reduced = c / g;
                let d_reduced = d / g;
                let key = (d_reduced, c_reduced);
                if let Some(count) = suf.get_mut(&key) {
                    *count -= 1;
                }
            }
        }

        ans
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().expect("No input").parse().expect("Invalid n");
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens.next().expect("Not enough numbers").parse().expect("Invalid number");
        nums.push(num);
    }

    let solution = Solution;
    let result = solution.number_of_subsequences(nums);
    println!("{}", result);
}