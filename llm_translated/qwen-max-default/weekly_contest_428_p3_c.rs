use std::io::{self, BufRead, Write};

fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; nums_size];
    let mut kmpcnt = 0;

    for i in 1..nums_size {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 == 1 && (i + 1) / 2 % (i + 1 - kmpcnt) == 0 {
            res += (nums_size - i - 1) as i32;
        }
    }

    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        kmpcnt = 0;
        if 2 * i < nums_size && i % (2 * i - cnt0[2 * i - 1]) == 0 {
            let end = std::cmp::min(3 * i, nums_size);
            for j in i + 1..end {
                while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                    kmpcnt = cnt[kmpcnt - 1];
                }
                if nums[j] == nums[i + kmpcnt] {
                    kmpcnt += 1;
                }
                cnt[j - i] = kmpcnt;
                if (j - i + 1) % 2 == 0 && (j - i + 1) / 2 % (j - i + 1 - kmpcnt) == 0 {
                    let len = j - i + 1;
                    let div = i - 1 + len / 2;
                    if len == 2 * i && i % (div + 1 - cnt0[div]) == 0 {
                        break;
                    }
                    res += 1;
                }
            }
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();

    // Input: size of the array
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse integer");

    // Input: array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read line");
        nums.push(input.trim().parse().expect("Failed to parse integer"));
    }

    // Calculate and print the result
    let result = beautiful_splits(&nums);
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}