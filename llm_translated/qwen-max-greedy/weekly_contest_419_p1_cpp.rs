use std::collections::{HashMap, BTreeMap};
use std::io::{self, BufRead};

fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i32> {
    let mut mp = HashMap::new();
    let mut res = Vec::new();

    let (mut l, mut r) = (0, 0);
    while r < nums.len() {
        *mp.entry(nums[r]).or_insert(0) += 1;

        if r - l + 1 == k {
            let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&k, &v)| (k, v)).collect();
            vec.sort_by(|a, b| {
                match a.1.cmp(&b.1) {
                    std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                    other => other.reverse(),
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

    // Read k and x
    let line1 = lines.next().unwrap().unwrap();
    let mut iter = line1.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    // Read the size of nums
    let line2 = lines.next().unwrap().unwrap();
    let nums_size: usize = line2.parse().unwrap();

    // Read the nums array
    let line3 = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = line3.split_whitespace()
                              .map(|s| s.parse().unwrap())
                              .collect();

    // Create an instance of Solution and call the function
    let res = find_x_sum(&nums, k, x);

    // Print the result
    for num in res {
        print!("{} ", num);
    }
    println!();
}