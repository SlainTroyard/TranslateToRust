use std::io;

pub struct Solution;

impl Solution {
    pub fn max_length(nums: &[i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let m = *nums.iter().max().unwrap() as usize;
        let mut fac = vec![Vec::new(); m + 1];

        for i in 2..=m {
            if fac[i].is_empty() {
                let mut j = i;
                while j <= m {
                    fac[j].push(i);
                    j += i;
                }
            }
        }

        let n = nums.len();
        let mut ans = 2;
        let mut j = 0;
        let mut vis = vec![false; m + 1];

        for i in 0..n {
            while j < n {
                let current_num = nums[j] as usize;
                let check = fac[current_num].iter().all(|&p| !vis[p]);
                if check {
                    for &p in &fac[current_num] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            ans = ans.max((j - i) as i32);
            let current_i_num = nums[i] as usize;
            for &p in &fac[current_i_num] {
                vis[p] = false;
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let num_size: usize = parts.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = parts
        .take(num_size)
        .map(|s| s.parse().unwrap())
        .collect();
    let solution = Solution;
    let result = solution.max_length(&nums);
    println!("{}", result);
}