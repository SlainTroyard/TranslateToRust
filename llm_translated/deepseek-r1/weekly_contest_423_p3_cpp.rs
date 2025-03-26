use std::collections::HashMap;
use std::io::{self, Read};

fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
    let mod_val = 1_000_000_007i64;
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    let mut sum: HashMap<i32, i64> = HashMap::new();

    for &num in nums {
        let i = num;
        // Get previous counts from adjacent numbers
        let prev_cnt_iminus1 = cnt.get(&(i - 1)).copied().unwrap_or(0);
        let prev_cnt_iplus1 = cnt.get(&(i + 1)).copied().unwrap_or(0);
        let temp_cnt = (prev_cnt_iminus1 + prev_cnt_iplus1 + 1) % mod_val;

        // Update count for current number
        let current_cnt = cnt.entry(i).or_insert(0);
        *current_cnt = (*current_cnt + temp_cnt) % mod_val;

        // Get previous sums from adjacent numbers
        let prev_sum_iminus1 = sum.get(&(i - 1)).copied().unwrap_or(0);
        let prev_sum_iplus1 = sum.get(&(i + 1)).copied().unwrap_or(0);
        let sum_part = (prev_sum_iminus1 + prev_sum_iplus1) % mod_val;

        // Calculate contribution from current subsequences
        let sum_contribution = ((prev_cnt_iminus1 + prev_cnt_iplus1 + 1) % mod_val) * i as i64 % mod_val;

        // Update sum for current number
        let current_sum = sum.entry(i).or_insert(0);
        *current_sum = (*current_sum + sum_part + sum_contribution) % mod_val;
    }

    // Sum all values and apply final modulus
    let total: i64 = sum.values().sum();
    (total % mod_val) as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens.next().unwrap().parse::<i32>().unwrap();
        nums.push(num);
    }

    let result = sum_of_good_subsequences(&nums);
    println!("{}", result);
}