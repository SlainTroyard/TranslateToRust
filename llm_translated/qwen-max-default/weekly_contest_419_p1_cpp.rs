use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i32> {
    let mut mp = HashMap::new();
    let mut res = Vec::new();

    let (mut l, mut r) = (0, 0);
    while r < nums.len() {
        *mp.entry(nums[r]).or_insert(0) += 1;

        if r - l + 1 == k {
            let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&num, &cnt)| (num, cnt)).collect();
            vec.sort_by(|a, b| {
                if a.1 == b.1 {
                    b.0.cmp(&a.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });

            let sum: i32 = vec.iter().take(x).map(|&(num, cnt)| num * cnt).sum();
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    let third_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let s = Solution;
    let res = s.find_x_sum(&nums, k, x);

    for num in res {
        print!("{} ", num);
    }
    println!();
}