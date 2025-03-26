use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let mut mp = HashMap::new();
    let mut res = Vec::new();
    let mut l = 0;

    for r in 0..nums.len() {
        *mp.entry(nums[r]).or_insert(0) += 1;

        if r - l + 1 == k {
            let mut vec: Vec<_> = mp.iter().map(|(&num, &cnt)| (num, cnt)).collect();
            vec.sort_by(|&(num1, cnt1), &(num2, cnt2)| {
                if cnt1 == cnt2 {
                    num2.cmp(&num1) // If counts are equal, sort by num in descending order
                } else {
                    cnt2.cmp(&cnt1) // Otherwise, sort by count in descending order
                }
            });

            let sum: i32 = vec.iter().take(x).map(|&(num, cnt)| num * cnt).sum();
            res.push(sum);

            if let Some(cnt) = mp.get_mut(&nums[l]) {
                *cnt -= 1;
                if *cnt == 0 {
                    mp.remove(&nums[l]);
                }
            }
            l += 1;
        }
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let x: usize = parts.next().unwrap().parse().unwrap();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read nums
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute the result
    let res = find_x_sum(&nums, k, x);

    // Print the result
    for num in res {
        print!("{} ", num);
    }
    println!();
}