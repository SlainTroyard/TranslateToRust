use std::io;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut pos_r = vec![n; n];
        let mut stack = Vec::new();
        
        // Preprocess to find next greater elements and build the graph
        for i in 0..n {
            while !stack.is_empty() && nums[i] >= nums[stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                pos_r[idx] = i;
            }
            if !stack.is_empty() {
                let top = stack.last().unwrap();
                stack.last_mut().unwrap().push(i);
            }
            stack.push(i);
        }
        
        let mut ans = 0;
        let mut cnt = 0;
        let mut l = 0;
        let mut q = VecDeque::new();
        
        for r in 0..n {
            while !q.is_empty() && nums[q.back().unwrap()] <= nums[r] {
                q.pop_back();
            }
            q.push_back(r);
            cnt += nums[q[0]] - nums[r];
            
            while cnt > k {
                let out = nums[l];
                for &i in g[l].iter() {
                    if i > r {
                        break;
                    }
                    let end = pos_r[i].min(r + 1);
                    cnt -= (out - nums[i]) * (end - i);
                }
                l += 1;
                if !q.is_empty() && q[0] < l {
                    q.pop_front();
                }
            }
            
            ans += (r - l + 1) as i64;
        }
        
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let k = iter.next().unwrap().parse::<i32>().unwrap();
    let mut nums = Vec::with_capacity(n);
    
    for _ in 0..n {
        nums.push(iter.next().unwrap().parse::<i32>().unwrap());
    }
    
    let solution = Solution;
    println!("{}", solution.count_non_decreasing_subarrays(nums, k));
}