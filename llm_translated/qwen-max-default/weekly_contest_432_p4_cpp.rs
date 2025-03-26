use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: &Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: Vec<usize> = Vec::new();

        for (i, &x) in nums.iter().enumerate() {
            while !st.is_empty() && x >= nums[st[st.len() - 1]] {
                pos_r[st[st.len() - 1]] = i;
                st.pop();
            }
            if !st.is_empty() {
                g[st[st.len() - 1]].push(i);
            }
            st.push(i);
        }

        let mut ans: i64 = 0;
        let mut cnt: i32 = 0;
        let mut l: usize = 0;
        let mut q: VecDeque<usize> = VecDeque::new();

        for r in 0..n {
            let x = nums[r];
            while !q.is_empty() && nums[q.back().unwrap()] <= x {
                q.pop_back();
            }
            q.push_back(r);
            cnt += nums[q.front().unwrap()] - x;
            while cnt > k {
                let out = nums[l];
                for &i in &g[l] {
                    if i > r {
                        break;
                    }
                    cnt -= (out - nums[i]) * ((pos_r[i].min(r + 1)) as i32 - i as i32);
                }
                l += 1;
                if !q.is_empty() && q.front().unwrap() < l {
                    q.pop_front();
                }
            }
            ans += (r - l + 1) as i64;
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the first line to get n and k
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    let n = input[0] as usize;
    let k = input[1];

    // Read the second line to get the array of integers
    buffer.clear();
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(&nums, k);

    writeln!(stdout, "{}", result).expect("Failed to write result");
}