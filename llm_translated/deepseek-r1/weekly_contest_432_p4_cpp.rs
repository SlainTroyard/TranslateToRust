use std::collections::VecDeque;
use std::io::{self, Read};

fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut pos_r = vec![n; n];
    let mut st = Vec::new();

    // Build the graph (g) and pos_r array using a stack
    for i in 0..n {
        let x = nums[i];
        // Pop elements from the stack while current element is >= top
        while let Some(&top) = st.last() {
            if x >= nums[top] {
                pos_r[top] = i;
                st.pop();
            } else {
                break;
            }
        }
        // Link current index to the last element in the stack
        if let Some(&top) = st.last() {
            g[top].push(i);
        }
        st.push(i);
    }

    let mut ans = 0i64;
    let mut cnt = 0i32;
    let mut l = 0;
    let mut q = VecDeque::new();

    // Sliding window with deque to maintain maximum
    for r in 0..n {
        let x = nums[r];
        // Maintain deque in decreasing order
        while let Some(&back) = q.back() {
            if nums[back] <= x {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(r);
        let front = *q.front().unwrap();
        cnt += (nums[front] - x) as i32;

        // Adjust window left boundary if cnt exceeds k
        while cnt > k {
            let out = nums[l];
            // Subtract contributions from elements in g[l]
            for &i in &g[l] {
                if i > r {
                    break;
                }
                let delta = (out - nums[i]) as i32;
                let min_val = std::cmp::min(pos_r[i], r + 1);
                let len = (min_val - i) as i32;
                cnt -= delta * len;
            }
            l += 1;
            // Remove elements from deque that are out of window
            if let Some(&front_q) = q.front() {
                if front_q < l {
                    q.pop_front();
                }
            }
        }

        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    
    let n = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap();
    let nums: Vec<i32> = tokens.take(n).collect();
    
    println!("{}", count_non_decreasing_subarrays(&nums, k));
}