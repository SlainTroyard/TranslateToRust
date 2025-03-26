use std::io;

struct Solution;

impl Solution {
    fn max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let m = *nums.iter().max().unwrap();
        if m == 0 {
            return 0;
        }
        // Precompute the smallest prime factors for each number up to m
        let mut factors = vec![Vec::new(); (m + 1) as usize];
        for i in 2..=m {
            if factors[i as usize].is_empty() {
                for j in (i..=m).step_by(i as usize) {
                    factors[j as usize].push(i);
                }
            }
        }
        let n = nums.len();
        let mut ans = 0;
        let mut vis = vec![false; (m + 1) as usize];
        let mut j = 0;
        for i in 0..n {
            while j < n {
                let num = nums[j];
                let factors_of_num = &factors[num as usize];
                let mut valid = true;
                for &p in factors_of_num {
                    if vis[p as usize] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    for &p in factors_of_num {
                        vis[p as usize] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            ans = ans.max(j - i);
            // Unmark the factors of nums[i]
            let factors_of_i = &factors[nums[i] as usize];
            for &p in factors_of_i {
                vis[p as usize] = false;
            }
        }
        ans as i32
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        println!("0");
        return Ok(());
    }
    let num_size: usize = tokens[0].parse().unwrap();
    let nums: Vec<i32> = tokens[1..num_size + 1]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let solution = Solution;
    let result = solution.max_length(nums);
    println!("{}", result);
    Ok(())
}