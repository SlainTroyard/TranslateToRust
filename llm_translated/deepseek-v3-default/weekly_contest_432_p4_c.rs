use std::collections::VecDeque;
use std::io::{self, BufRead};

// Simple stack implementation
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, item: i32) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<&i32> {
        self.data.last()
    }
}

// Simple deque implementation
struct Deque {
    data: VecDeque<i32>,
}

impl Deque {
    fn new(capacity: usize) -> Self {
        Deque {
            data: VecDeque::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push_back(&mut self, item: i32) {
        self.data.push_back(item);
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.data.pop_front()
    }

    fn front(&self) -> Option<&i32> {
        self.data.front()
    }

    fn back(&self) -> Option<&i32> {
        self.data.back()
    }
}

// Dynamic vector implementation
struct Vector {
    data: Vec<i32>,
}

impl Vector {
    fn new(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push_back(&mut self, item: i32) {
        self.data.push(item);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }
}

// Count non-decreasing subarrays with sum at least k
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vector> = (0..n).map(|_| Vector::new(10)).collect();
    let mut pos_r = vec![n; n];

    // Use stack to find the next greater or equal element for each position
    let mut st = Stack::new(n);
    for i in 0..n {
        let x = nums[i];
        while !st.is_empty() && x >= nums[*st.top().unwrap() as usize] {
            pos_r[st.pop().unwrap() as usize] = i;
        }
        if !st.is_empty() {
            g[st.top().unwrap() as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    // Calculate the result
    let mut ans: i64 = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = Deque::new(n);

    for r in 0..n {
        let x = nums[r];
        while !q.is_empty() && nums[*q.back().unwrap() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += nums[*q.front().unwrap() as usize] - x;

        while cnt > k {
            let out = nums[l];
            for j in 0..g[l].size() {
                let i = *g[l].get(j).unwrap() as usize;
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < r + 1 { pos_r[i] } else { r + 1 };
                cnt -= (out - nums[i]) * (min_pos - i) as i32;
            }
            l += 1;
            if !q.is_empty() && *q.front().unwrap() as usize < l {
                q.pop_front();
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numsSize and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let nums_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Read nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);
}