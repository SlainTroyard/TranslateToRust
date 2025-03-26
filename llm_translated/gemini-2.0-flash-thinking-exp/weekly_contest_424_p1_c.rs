fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right = 0;
    let mut ret = 0;

    // Calculate initial sumRight
    for &num in nums.iter() {
        sum_right += num;
    }

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
    use std::io;

    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    if nums.len() != n as usize {
        eprintln!("Input number count does not match n");
        return;
    }

    let result = count_valid_selections(&nums);
    println!("{}", result);
}