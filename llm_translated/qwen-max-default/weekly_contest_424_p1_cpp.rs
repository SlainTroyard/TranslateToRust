use std::io::{self, BufRead, Write};

fn count_valid_selections(nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    // Compute prefix sums from the left
    for i in 1..n {
        left[i] = left[i - 1] + nums[i - 1];
    }

    // Compute prefix sums from the right
    for i in 1..n {
        right[n - i - 1] = right[n - i] + nums[n - i];
    }

    // Iterate through each element to count valid selections
    for i in 0..n {
        if nums[i] != 0 {
            continue;
        }
        if left[i] == right[i] {
            res += 2;
        }
        if (left[i] - right[i]).abs() == 1 {
            res += 1;
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    // Read the number of elements in the nums array
    let mut input = String::new();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements of the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Call the count_valid_selections function and store the result
    let result = count_valid_selections(&nums);

    // Print the result
    writeln!(stdout_lock, "{}", result).expect("Failed to write to stdout");
}