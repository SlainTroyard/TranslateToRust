use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut freq_map: HashMap<i32, usize> = HashMap::new();
        let k = k as usize;
        let x = x as usize;
        let mut left = 0;
        for right in 0..nums.len() {
            let num = nums[right];
            *freq_map.entry(num).or_insert(0) += 1;

            // When window reaches size k
            if right >= k - 1 {
                // Collect entries and sort
                let mut vec: Vec<_> = freq_map.iter().map(|(&n, &c)| (n, c)).collect();
                vec.sort_by(|a, b| {
                    if a.1 == b.1 {
                        // Higher number first if counts are equal
                        b.0.cmp(&a.0)
                    } else {
                        // Higher count first
                        b.1.cmp(&a.1)
                    }
                });

                // Calculate sum of top x elements
                let mut sum = 0;
                let limit = std::cmp::min(x, vec.len());
                for i in 0..limit {
                    sum += vec[i].0 * vec[i].1 as i32;
                }
                res.push(sum);

                // Move left pointer and update frequency map
                let left_num = nums[left];
                let count = freq_map.get_mut(&left_num).unwrap();
                *count -= 1;
                if *count == 0 {
                    freq_map.remove(&left_num);
                }
                left += 1;
            }
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let k = iter.next().unwrap();
    let x = iter.next().unwrap();
    let nums_size = iter.next().unwrap() as usize;
    let nums: Vec<i32> = iter.take(nums_size).collect();
    let res = Solution::find_x_sum(&nums, k, x);
    for num in &res {
        print!("{} ", num);
    }
    println!();
}