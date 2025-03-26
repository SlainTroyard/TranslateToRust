use std::io::{self, BufRead};
use std::cmp;

fn max_good_number(nums: &[i32]) -> i32 {
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;

    for (i, &num) in nums.iter().enumerate() {
        for j in 0..7 {
            let s = (num << j) & 64;
            if s == 0 {
                match i {
                    0 => nums1 += 1,
                    1 => num2 += 1,
                    2 => num3 += 1,
                    _ => {}
                }
            } else {
                break;
            }
        }
    }

    let times = vec![nums1, num2, num3];
    let mut store = vec![0, 1, 2];

    for _ in 1..=2 {
        for j in 0..2 {
            let value1 = (nums[store[j]] << (7 - times[store[j + 1]])) + nums[store[j + 1]];
            let value2 = (nums[store[j + 1]] << (7 - times[store[j]])) + nums[store[j]];
            if value2 >= value1 {
                store.swap(j, j + 1);
            }
        }
    }

    (nums[store[0]] << (14 - times[store[1]] - times[store[2]]))
        + (nums[store[1]] << (7 - times[store[2]]))
        + nums[store[2]]
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read numSize
    let num_size: usize = lines
        .next()
        .expect("Expected input size line.")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse numSize");

    // Read nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input numbers line.")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    if nums.len() != num_size {
        panic!("The number of elements in `nums` does not match `numSize`.");
    }

    // Compute the result and print it
    let result = max_good_number(&nums);
    println!("{}", result);
}