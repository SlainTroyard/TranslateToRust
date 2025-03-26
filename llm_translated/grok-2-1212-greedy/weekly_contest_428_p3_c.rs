use std::io::{self, BufRead};

fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; nums_size];
    let mut kmpcnt = 0;

    cnt0[0] = 0;
    for i in 1..nums_size {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 != 0 && (i + 1) / 2 % (i + 1 - kmpcnt) == 0 {
            res += (nums_size - i - 1) as i32;
        }
    }

    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        let mut end = nums_size;
        kmpcnt = 0;
        cnt[0] = 0;
        if 2 * i < nums_size && i % (2 * i - cnt0[2 * i - 1]) == 0 {
            end = end.min(3 * i);
        }
        for j in (i + 1)..end {
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
                if len == i * 2 && i % (div + 1 - cnt0[div]) == 0 {
                    break;
                }
                res += 1;
            }
        }
    }

    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input: array elements
    let nums: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = beautiful_splits(&nums);
    println!("{}", result);

    Ok(())
}