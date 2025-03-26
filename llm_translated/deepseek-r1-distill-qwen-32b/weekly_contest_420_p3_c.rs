use std::io;

fn max_factor(n: i32) -> Option<i32> {
    if n <= 1 {
        return None;
    }
    let mut max = None;
    let sqrt_n = (n as f64).sqrt() as i32;
    for j in 2..=sqrt_n + 1 {
        if n % j == 0 {
            let candidate = j;
            let other = n / j;
            if let Some(current_max) = max {
                if candidate > current_max {
                    max = Some(candidate);
                }
                if other > current_max {
                    max = Some(other);
                }
            } else {
                max = Some(candidate);
                if other > candidate {
                    max = Some(other);
                }
            }
        }
    }
    max
}

fn min_operations(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }
    let mut res = 0;
    for i in (0..n - 1).rev() {
        while nums[i] > nums[i + 1] {
            match max_factor(nums[i]) {
                None => return -1,
                Some(f) => {
                    nums[i] /= f;
                    res += 1;
                }
            }
        }
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    if parts.is_empty() {
        println!("0");
        return;
    }
    let nums_size = parts[0] as usize;
    let nums = if nums_size == 0 {
        Vec::new()
    } else {
        parts[1..=nums_size].to_vec()
    };
    let res = min_operations(nums);
    println!("{}", res);
}