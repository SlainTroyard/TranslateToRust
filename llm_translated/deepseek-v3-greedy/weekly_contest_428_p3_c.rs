use std::cmp::min;
use std::io::{self, Write};

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
        if i % 2 == 1 && 0 == ((i + 1) / 2) % (i + 1 - kmpcnt) {
            res += (nums_size - i - 1) as i32;
        }
    }

    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        let mut end = nums_size;
        kmpcnt = 0;

        if 2 * i < nums_size && 0 == i % (2 * i - cnt0[2 * i - 1]) {
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
            if (j - i + 1) % 2 == 0 && 0 == ((j - i + 1) / 2) % (j - i + 1 - kmpcnt) {
                let len = j - i + 1;
                let div = i - 1 + len / 2;
                if len == i * 2 && 0 == i % (div + 1 - cnt0[div]) {
                    break;
                }
                res += 1;
            }
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    let result = beautiful_splits(&nums);
    println!("{}", result);
}