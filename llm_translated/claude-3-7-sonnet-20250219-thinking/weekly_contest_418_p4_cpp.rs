use std::io::{self, Read};
use std::cmp;

struct Solution;

impl Solution {
    fn gcd_values(nums: &Vec<i32>, queries: &Vec<i64>) -> Vec<i32> {
        // Find maximum element in nums
        let mx = *nums.iter().max().unwrap();
        
        // Count occurrences of each number
        let mut cnt_x = vec![0; mx as usize + 1];
        for &x in nums {
            cnt_x[x as usize] += 1;
        }

        // Compute GCD counts
        let mut cnt_gcd = vec![0i64; mx as usize + 1];
        for i in (1..=mx as usize).rev() {
            let mut c = 0;
            for j in (i..=mx as usize).step_by(i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (c as i64 * (c as i64 - 1)) / 2;
        }
        
        // Compute prefix sum
        for i in 1..=mx as usize {
            cnt_gcd[i] += cnt_gcd[i-1];
        }

        // Process queries
        let mut ans = vec![0; queries.len()];
        for i in 0..queries.len() {
            ans[i] = cnt_gcd.partition_point(|&x| x <= queries[i]) as i32;
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut words = input.split_whitespace();
    
    // Read number of elements
    let n: usize = words.next().unwrap().parse().expect("Failed to parse n");
    
    // Read nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(words.next().unwrap().parse::<i32>().expect("Failed to parse nums element"));
    }
    
    // Read number of queries
    let q: usize = words.next().unwrap().parse().expect("Failed to parse q");
    
    // Read queries array
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        queries.push(words.next().unwrap().parse::<i64>().expect("Failed to parse query value"));
    }
    
    // Process queries and get answer
    let ans = Solution::gcd_values(&nums, &queries);
    
    // Output results
    for (i, &x) in ans.iter().enumerate() {
        print!("{}", x);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    println!();
    
    Ok(())
}