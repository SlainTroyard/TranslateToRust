use std::io;
use std::io::BufRead;

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
    let stdin = io::stdin();
    let mut line = String::new();

    // Read the size of the array
    stdin.lock().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    line.clear();

    // Read the array elements
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    stdin.lock().read_line(&mut line).unwrap();
    let nums_str: Vec<&str> = line.trim().split_whitespace().collect();

    for num_str in nums_str {
        nums.push(num_str.parse().unwrap());
    }

    let result = count_valid_selections(&nums);
    println!("{}", result);
}