use std::io;

/// Translated from C code of LeetCode Weekly Contest 424 Problem 1.
/// Counts valid selections where the sum of elements before and after a zero are equal or differ by 1.
///
/// # Arguments
/// * `nums` - Slice of integers to process
///
/// # Returns
/// The count of valid selections as per problem requirements
pub fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let sum_total: i32 = nums.iter().sum();
    let mut sum_right = sum_total;
    let mut ret = 0;

    for &num in nums.iter() {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid n value");
    let nums: Vec<i32> = iter
        .take(n)
        .map(|x| x.parse().expect("Invalid number in input"))
        .collect();

    let result = count_valid_selections(&nums);
    println!("{}", result);
}