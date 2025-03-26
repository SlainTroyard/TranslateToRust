use std::io;

/// Computes the number of valid selections based on prefix sums.
pub fn count_valid_selections(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    // Compute prefix sums from the left
    for i in 1..n {
        left[i] = left[i - 1] + nums[i - 1];
    }

    // Compute prefix sums from the right
    for i in 1..n {
        let pos = n - i - 1;
        right[pos] = right[pos + 1] + nums[pos + 1];
    }

    let mut res = 0;
    for i in 0..n {
        if nums[i] != 0 {
            continue;
        }
        let l = left[i];
        let r = right[i];
        if l == r {
            res += 2;
        }
        if (l - r).abs() == 1 {
            res += 1;
        }
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut tokens = input.split_whitespace();

    let n: usize = tokens
        .next()
        .expect("No elements for n")
        .parse()
        .expect("Invalid n");

    let nums: Vec<i32> = tokens
        .take(n)
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let result = count_valid_selections(&nums);
    println!("{}", result);
}