use std::io::{self, Read};

struct Solution;

impl Solution {
    fn max_good_number(&self, nums: &[i32]) -> i32 {
        let mut nums = nums.to_vec();
        nums.sort_by(|&a, &b| {
            let len_a = (a as u32).ilog2() as i32 + 1;
            let len_b = (b as u32).ilog2() as i32 + 1;
            // Compare concatenation in both orders
            // In C++: (a << len_b | b) > (b << len_a | a)
            let a_first = (a << len_b) | b;
            let b_first = (b << len_a) | a;
            // Reverse comparison to match C++'s descending order
            b_first.cmp(&a_first)
        });

        let mut ans = 0;
        for &x in &nums {
            ans = (ans << ((x as u32).ilog2() as i32 + 1)) | x;
        }
        ans
    }
}

fn main() {
    // Read all input as one string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut iter = input.split_whitespace();
    
    // Parse the size
    let num_size: usize = iter.next().unwrap().parse().expect("Invalid input");
    
    // Parse the numbers
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        nums.push(iter.next().unwrap().parse::<i32>().expect("Invalid input"));
    }
    
    // Solve and output
    let solution = Solution;
    println!("{}", solution.max_good_number(&nums));
}