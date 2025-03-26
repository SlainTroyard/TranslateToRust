use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: Vec<usize> = Vec::new();
        for i in 0..n {
            let x = nums[i];
            while let Some(&top) = st.last() {
                if x >= nums[top] {
                    pos_r[top] = i;
                    st.pop();
                } else {
                    break;
                }
            }
            if let Some(parent) = st.last() {
                g[*parent].push(i);
            }
            st.push(i);
        }

        let mut ans: i64 = 0;
        let mut cnt: i64 = 0;
        let mut l: usize = 0;
        let mut q: VecDeque<usize> = VecDeque::new();
        for r in 0..n {
            let x = nums[r];
            while let Some(&last) = q.back() {
                if nums[last] <= x {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(r);
            let front = q[0];
            cnt += (nums[front] as i64) - (x as i64);
            while cnt > k as i64 {
                let out = nums[l];
                for &i in &g[l] {
                    if i > r {
                        break;
                    }
                    let upper = pos_r[i].min(r + 1);
                    let term = (out as i64 - nums[i] as i64) * ((upper - i) as i64);
                    cnt -= term;
                }
                l += 1;
                if let Some(front_val) = q.front().copied() {
                    if front_val < l {
                        q.pop_front();
                    }
                }
            }
            ans += (r - l + 1) as i64;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();
    let mut nums = Vec::new();
    for _ in 0..n {
        let num: i32 = tokens.next().unwrap().parse().unwrap();
        nums.push(num);
    }
    let solution = Solution;
    println!("{}", solution.count_non_decreasing_subarrays(nums, k));
}