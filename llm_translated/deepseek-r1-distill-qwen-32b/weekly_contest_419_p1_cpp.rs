use std::collections::HashMap;
use std::cmp::Ordering;

struct Solution;

impl Solution {
    fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
        let mut freq = HashMap::new();
        let mut res = Vec::new();
        let mut l = 0;
        for r in 0..nums.len() {
            let num_r = nums[r];
            *freq.entry(num_r).or_insert(0) += 1;

            if (r - l + 1) as i32 == k {
                let mut vec: Vec<(i32, i32)> = freq.iter().map(|(&num, &count)| (num, count)).collect();

                vec.sort_by(|a, b| {
                    if a.1 > b.1 {
                        Ordering::Less
                    } else if a.1 == b.1 {
                        if a.0 > b.0 {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        Ordering::Greater
                    }
                });

                let x = x as usize;
                let mut sum = 0;
                for i in 0..x {
                    if i >= vec.len() {
                        break;
                    }
                    sum += vec[i].0 * vec[i].1;
                }
                res.push(sum);

                let num_l = nums[l];
                let count = freq.get_mut(&num_l).unwrap();
                *count -= 1;
                if *count == 0 {
                    freq.remove(&num_l);
                }
                l += 1;
            }
        }
        res
    }
}

fn main() {
    use std::io::stdin;
    let k: i32;
    let x: i32;
    {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        k = parts[0];
        x = parts[1];
    }

    let nums_size: usize;
    {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        nums_size = input.trim().parse().unwrap();
    }

    let mut nums = Vec::with_capacity(nums_size);
    {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        nums = parts;
    }

    let s = Solution;
    let res = s.find_x_sum(&nums, k, x);

    for num in res {
        print!("{} ", num);
    }
    println!();
}