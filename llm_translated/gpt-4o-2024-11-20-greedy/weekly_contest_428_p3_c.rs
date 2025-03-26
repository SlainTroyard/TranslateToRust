use std::io::{self, Write};
use std::cmp::min;

fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; nums_size];
    let mut kmpcnt = 0;

    // First loop to calculate cnt0 and partial results
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

    // Second loop to calculate additional results
    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        let mut end = nums_size;
        kmpcnt = 0;

        if 2 * i < nums_size && i % (2 * i - cnt0[2 * i - 1]) == 0 {
            end = min(end, 3 * i);
        }

        for j in (i + 1)..end {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n);

    // Calculate the result
    let result = beautiful_splits(&nums);

    // Print the result to stdout
    println!("{}", result);
}