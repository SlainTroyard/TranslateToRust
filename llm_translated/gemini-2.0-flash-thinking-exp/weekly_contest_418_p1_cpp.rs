use std::io;

struct Solution {}

impl Solution {
    pub fn max_good_number(&self, nums: &mut Vec<i32>) -> i32 {
        nums.sort_by(|a, b| {
            let len_a = Self::integer_log2(*a) + 1;
            let len_b = Self::integer_log2(*b) + 1;
            let val_a = (*a as i64) << len_b | (*b as i64);
            let val_b = (*b as i64) << len_a | (*a as i64);
            val_b.cmp(&val_a) // Reverse order for descending sort
        });

        let mut ans: i32 = 0;
        for &x in nums {
            ans = ans << (Self::integer_log2(x) + 1) | x;
        }
        ans
    }

    fn integer_log2(n: i32) -> i32 {
        if n == 0 {
            return -1; // Or handle the error as appropriate for your use case
        }
        31 - n.leading_zeros() as i32
    }
}

fn main() {
    let mut num_size = String::new();
    io::stdin().read_line(&mut num_size).expect("Failed to read line");
    let num_size: usize = num_size.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let mut nums_mut = nums;
    let s = Solution {};
    println!("{}", s.max_good_number(&mut nums_mut));
}