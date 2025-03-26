use std::io::{self, Read};

/// Count valid selections based on the rules:
/// - For each 0 in the array, it can be filled with a value
/// - If the sum of elements to the left equals the sum of elements to the right, add 2 to the count
/// - If the absolute difference between left and right sums is 1, add 1 to the count
fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;
    
    for &num in nums {
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
    
    let mut tokens = input.split_whitespace();
    
    // Read n (array size)
    let n: usize = tokens.next().unwrap().parse().expect("Invalid input for n");
    
    // Read nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(tokens.next().unwrap().parse::<i32>().expect("Invalid input for nums"));
    }
    
    // Compute and print result
    let result = count_valid_selections(&nums);
    println!("{}", result);
}