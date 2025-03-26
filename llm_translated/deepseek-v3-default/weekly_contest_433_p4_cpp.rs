use std::io;
use std::collections::VecDeque;

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let k: i32 = iter.next().unwrap().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.split_whitespace() {
        nums.push(num.parse::<i32>().expect("Invalid input"));
    }

    // Calculate and print the result
    let result = min_max_subarray_sum(&mut nums, k);
    println!("{}", result);
}

fn min_max_subarray_sum(nums: &mut Vec<i32>, k: i32) -> i64 {
    // Helper function to calculate the count
    let count = |m: i32| -> i64 {
        if m > k {
            (m * 2 - k + 1) as i64 * k as i64 / 2
        } else {
            (m + 1) as i64 * m as i64 / 2
        }
    };

    // Helper function to calculate the sum of subarray minimums
    let sum_subarray_mins = |nums: &Vec<i32>| -> i64 {
        let mut res = 0;
        let mut stack = VecDeque::new();
        stack.push_back(-1);
        for r in 0..nums.len() {
            while stack.len() > 1 && nums[*stack.back().unwrap() as usize] >= nums[r] {
                let i = stack.pop_back().unwrap();
                let l = *stack.back().unwrap();
                let cnt = count((r as i32) - l - 1) - count(i - l - 1) - count((r as i32) - i - 1);
                res += nums[i as usize] as i64 * cnt;
            }
            stack.push_back(r as i32);
        }
        res
    };

    // Append a very small number to the end of the array
    nums.push(i32::MIN / 2);
    let mut ans = sum_subarray_mins(nums);

    // Negate all numbers except the last one
    for x in nums.iter_mut().take(nums.len() - 1) {
        *x = -*x;
    }
    *nums.last_mut().unwrap() *= -1;

    // Subtract the sum of subarray minimums of the negated array
    ans -= sum_subarray_mins(nums);

    ans
}