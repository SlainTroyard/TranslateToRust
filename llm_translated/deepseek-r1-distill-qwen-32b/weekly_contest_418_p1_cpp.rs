use std::io;
use std::cmp::Ordering;

struct Solution;

impl Solution {
    fn max_good_number(mut nums: Vec<i32>) -> u128 {
        nums.sort_by(|a, b| {
            let a_val = *a as u64;
            let b_val = *b as u64;
            let len_a = a_val.ilog2() + 1;
            let len_b = b_val.ilog2() + 1;
            let ab = (a_val << len_b) | b_val;
            let ba = (b_val << len_a) | a_val;
            if ab > ba {
                Ordering::Less
            } else if ab < ba {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let mut ans: u128 = 0;
        for x in nums {
            let len_x = (x as u64).ilog2() + 1;
            ans = (ans << len_x) | (x as u128);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let num_size = iter.next().unwrap().parse::<usize>().unwrap();
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        nums.push(x);
    }
    let solution = Solution;
    let result = solution.max_good_number(nums);
    println!("{}", result);
}