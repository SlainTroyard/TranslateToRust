use std::io::{self, BufRead};
use std::cmp::min;

/// Function to compute the number of beautiful splits
fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; nums_size];
    let mut kmpcnt = 0;

    cnt0[0] = 0;

    // First pass to calculate the cnt0 array
    for i in 1..nums_size {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 == 1 && ((i + 1) / 2) % (i + 1 - kmpcnt) == 0 {
            res += nums_size - i - 1;
        }
    }

    // Second pass considering offset cases
    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        let mut end = nums_size;
        kmpcnt = 0;

        cnt[0] = 0;
        if 2 * i < nums_size && i % (2 * i - cnt0[2 * i - 1]) == 0 {
            end = min(end, 3 * i);
        }

        for j in i + 1..end {
            while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                kmpcnt = cnt[kmpcnt - 1];
            }
            if nums[j] == nums[i + kmpcnt] {
                kmpcnt += 1;
            }
            cnt[j - i] = kmpcnt;

            if (j - i + 1) % 2 == 0 && ((j - i + 1) / 2) % (j - i + 1 - kmpcnt) == 0 {
                let len = j - i + 1;
                let div = i - 1 + len / 2;
                if len == i * 2 && i % (div + 1 - cnt0[div]) == 0 {
                    break;
                }
                res += 1;
            }
        }
    }

    res
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read size of the array
    let n: usize = lines.next()
        .expect("Expected input for the size of the array")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse array size as usize");

    // Read array elements
    let nums: Vec<i32> = lines.next()
        .expect("Expected input for the array elements")
        .expect("Failed to read line")
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Failed to parse element as i32"))
        .collect();

    // Ensure the array size matches the provided n
    assert_eq!(nums.len(), n, "Array size does not match the input size");

    // Calculate and print the result
    let result = beautiful_splits(&nums);
    println!("{}", result);
}