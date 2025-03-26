use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: &Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g = vec![vec![]; n];
        let mut pos_r = vec![n; n];
        let mut st = Vec::new();

        for (i, &x) in nums.iter().enumerate() {
            while let Some(&top) = st.last() {
                if x >= nums[top] {
                    pos_r[top] = i;
                    st.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = st.last() {
                g[top].push(i);
            }
            st.push(i);
        }

        let mut ans = 0;
        let mut cnt = 0;
        let mut l = 0;
        let mut q = Vec::new();

        for r in 0..n {
            let x = nums[r];
            while let Some(&back) = q.last() {
                if nums[back] <= x {
                    q.pop();
                } else {
                    break;
                }
            }
            q.push(r);
            cnt += nums[q[0]] - x;
            while cnt > k as i64 {
                let out = nums[l];
                for &i in &g[l] {
                    if i > r {
                        break;
                    }
                    cnt -= (out - nums[i]) as i64 * (std::cmp::min(pos_r[i], r + 1) - i) as i64;
                }
                l += 1;
                if let Some(&front) = q.first() {
                    if front < l {
                        q.remove(0);
                    }
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
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    buffer.clear();

    // Read the second line to get the array
    stdin.lock().read_line(&mut buffer).unwrap();
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    buffer.clear();

    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(&nums, k);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}