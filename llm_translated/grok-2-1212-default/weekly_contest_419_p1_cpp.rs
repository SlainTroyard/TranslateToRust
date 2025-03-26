use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i64> {
    let mut mp: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i64> = Vec::new();

    let mut l = 0;
    for r in 0..nums.len() {
        *mp.entry(nums[r]).or_insert(0) += 1;
        if r - l + 1 == k {
            let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&k, &v)| (k, v)).collect();
            vec.sort_by(|a, b| {
                if a.1 == b.1 {
                    b.0.cmp(&a.0) // When counts are equal, sort by number in descending order
                } else {
                    b.1.cmp(&a.1) // Sort by count in descending order
                }
            });

            let mut sum = 0;
            for i in 0..x.min(vec.len()) {
                sum += vec[i].0 as i64 * vec[i].1 as i64;
            }
            res.push(sum);

            *mp.get_mut(&nums[l]).unwrap() -= 1;
            if *mp.get(&nums[l]).unwrap() == 0 {
                mp.remove(&nums[l]);
            }
            l += 1;
        }
    }
    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let k = first_line[0];
    let x = first_line[1];

    // Read nums size
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read nums
    let nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate result
    let res = find_x_sum(&nums, k, x);

    // Print result
    for num in res {
        print!("{} ", num);
    }
    println!();

    Ok(())
}