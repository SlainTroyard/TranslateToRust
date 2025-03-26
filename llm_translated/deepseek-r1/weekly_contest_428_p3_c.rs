use std::io::{self, Read};

fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;

    // First part: compute cnt0 array using KMP-like preprocessing
    let mut cnt0 = vec![0; nums_size];
    let mut kmpcnt = 0;

    for i in 1..nums_size {
        // KMP failure function calculation
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;

        // Check condition for beautiful split
        if i % 2 == 1 { // i is odd
            let half = (i + 1) / 2;
            let divisor = (i + 1) - kmpcnt;
            if half % divisor == 0 {
                res += (nums_size - i - 1) as i32;
            }
        }
    }

    // Second part: process each possible i for additional splits
    for i in 1..nums_size {
        let nums_size_i = nums_size - i;
        let mut cnt = vec![0; nums_size_i];
        let mut end = nums_size;

        // Determine end based on prefix function properties
        if 2 * i < nums_size {
            let two_i_minus_1 = 2 * i - 1;
            let divisor = 2 * i - cnt0[two_i_minus_1];
            if i % divisor == 0 {
                end = std::cmp::min(end, 3 * i);
            }
        }

        let mut kmpcnt = 0;

        for j in (i + 1)..end {
            // KMP preprocessing for the substring starting at i
            while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                kmpcnt = cnt[kmpcnt - 1];
            }
            if nums[j] == nums[i + kmpcnt] {
                kmpcnt += 1;
            }
            let idx = j - i;
            cnt[idx] = kmpcnt;

            let len = j - i + 1;
            if len % 2 == 0 {
                let half_len = len / 2;
                let divisor = len - kmpcnt;
                if half_len % divisor == 0 {
                    let div = i - 1 + half_len;

                    // Check special case to break early
                    if len == 2 * i {
                        let div_val = (div + 1) - cnt0[div];
                        if i % div_val == 0 {
                            break;
                        }
                    }

                    res += 1;
                }
            }
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();

    let result = beautiful_splits(&nums);
    println!("{}", result);
}