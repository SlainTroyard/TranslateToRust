use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn find_x_sum(nums: &Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut res: Vec<i64> = Vec::new();
        let mut l: usize = 0;
        let mut r: usize = 0;

        while r < nums.len() {
            *mp.entry(nums[r]).or_insert(0) += 1;
            if (r as i32 - l as i32 + 1) == k {
                let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&k, &v)| (k, v)).collect();
                vec.sort_by(|a, b| {
                    if a.1 == b.1 {
                        b.0.cmp(&a.0) // 出现次数相同，num大的放前边
                    } else {
                        b.1.cmp(&a.1) // 出现次数不同，cnt大的在前边
                    }
                });

                let mut sum: i64 = 0;
                for i in 0..(x as usize).min(vec.len()) {
                    sum += vec[i].0 as i64 * vec[i].1 as i64;
                }
                res.push(sum);

                *mp.get_mut(&nums[l]).unwrap() -= 1;
                if *mp.get(&nums[l]).unwrap() == 0 {
                    mp.remove(&nums[l]);
                }
                l += 1;
            }
            r += 1;
        }
        res
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap()?;
    let nums_size: usize = second_line.parse().unwrap();

    let third_line = lines.next().unwrap()?;
    let nums: Vec<i32> = third_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let res = Solution::find_x_sum(&nums, k, x);
    for num in res {
        print!("{} ", num);
    }
    println!();

    Ok(())
}