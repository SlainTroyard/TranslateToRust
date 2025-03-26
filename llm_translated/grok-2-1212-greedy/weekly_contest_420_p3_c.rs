use std::io::{self, BufRead};
use std::cmp::{max, min};
use std::i32;

fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let mut res = 0;
    let nums_size = nums.len();

    if nums_size == 1 {
        return res;
    }

    for i in (0..nums_size - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max_factor = 0;
            let mut j = 0;
            let mut max = i32::MIN;
            let mut count = 1;
            let original = nums[i];

            loop {
                max = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max = max(max, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max = max(max, nums[i] / j);
                        }
                    }
                }

                if max == i32::MIN || (count *= max) == original {
                    return -1;
                } else {
                    nums[i] /= max;
                    if nums[i] <= nums[i + 1] {
                        res += 1;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read nums
    let mut nums: Vec<i32> = lines
        .take(nums_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print result
    let result = min_operations(&mut nums);
    println!("{}", result);

    Ok(())
}