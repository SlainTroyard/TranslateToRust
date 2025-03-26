use std::collections::VecDeque;
use std::io::{self, Read};

fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g = vec![Vec::new(); n];
    let mut pos_r = vec![n; n];
    let mut stack = Vec::new();

    // Build graph (g) and pos_r array using monotonic stack
    for i in 0..n {
        let x = nums[i];
        // Pop elements from stack while current element is >= stack top
        while let Some(&top) = stack.last() {
            if x >= nums[top] {
                pos_r[top] = i;
                stack.pop();
            } else {
                break;
            }
        }
        // Add current index to the previous top's list if stack is not empty
        if let Some(&top) = stack.last() {
            g[top].push(i);
        }
        stack.push(i);
    }

    let mut ans = 0i64;
    let mut cnt = 0i64;
    let mut l = 0;
    let mut deque = VecDeque::new();

    // Process each right boundary of the window
    for r in 0..n {
        let x = nums[r];
        // Maintain deque in decreasing order
        while let Some(&back) = deque.back() {
            if nums[back] <= x {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(r);
        cnt += (nums[*deque.front().unwrap()] - x) as i64;

        // Adjust window left boundary if sum exceeds k
        while cnt > k as i64 {
            let out = nums[l];
            // Subtract contributions from elements in g[l]
            for &i in &g[l] {
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < r + 1 { pos_r[i] } else { r + 1 };
                let contribution = (out - nums[i]) as i64 * (min_pos - i) as i64;
                cnt -= contribution;
            }
            l += 1;
            // Remove elements from deque front that are out of window
            if let Some(&front) = deque.front() {
                if front < l {
                    deque.pop_front();
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
    let mut tokens = input.split_whitespace();

    // Parse n and k from input
    let n = tokens
        .next()
        .expect("Missing n")
        .parse()
        .expect("Invalid n");
    let k = tokens
        .next()
        .expect("Missing k")
        .parse()
        .expect("Invalid k");

    // Parse nums array
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Ensure correct number of elements
    if nums.len() != n {
        panic!("Invalid input: incorrect number of elements");
    }

    // Compute and print result
    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);
}