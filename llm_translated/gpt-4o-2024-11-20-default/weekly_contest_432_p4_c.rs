// Problem: Weekly Contest 432 Problem 4
use std::cmp::{max, min};
use std::collections::{VecDeque};
use std::io::{self, BufRead};

// Simple integer stack implementation
struct Stack {
    data: Vec<usize>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, item: usize) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<usize> {
        self.data.pop()
    }

    fn top(&self) -> Option<usize> {
        self.data.last().copied()
    }
}

// Simple integer deque implementation
struct Deque {
    data: VecDeque<usize>,
}

impl Deque {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push_back(&mut self, item: usize) {
        self.data.push_back(item);
    }

    fn pop_back(&mut self) -> Option<usize> {
        self.data.pop_back()
    }

    fn pop_front(&mut self) -> Option<usize> {
        self.data.pop_front()
    }

    fn front(&self) -> Option<usize> {
        self.data.front().copied()
    }

    fn back(&self) -> Option<usize> {
        self.data.back().copied()
    }
}

// Simple integer vector implementation
struct Vector {
    data: Vec<usize>,
}

impl Vector {
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push_back(&mut self, item: usize) {
        self.data.push(item);
    }

    fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.data.iter()
    }
}

// Computing the count of non-decreasing subarrays with sum at least k
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let nums_size = nums.len();
    let mut g: Vec<Vector> = (0..nums_size).map(|_| Vector::new(10)).collect();
    let mut pos_r = vec![nums_size; nums_size];

    let mut st = Stack::new(nums_size);
    for i in 0..nums_size {
        let x = nums[i];
        while let Some(&top) = st.top() {
            if x >= nums[top] {
                pos_r[top] = i;
                st.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = st.top() {
            g[top].push_back(i);
        }
        st.push(i);
    }

    let mut ans: i64 = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = Deque::new();

    for r in 0..nums_size {
        let x = nums[r];
        while let Some(&back) = q.back() {
            if nums[back] <= x {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(r);
        cnt += nums[q.front().unwrap()] - x;

        while cnt > k {
            let out = nums[l];
            for &i in g[l].iter() {
                if i > r {
                    break;
                }
                let min_pos = min(pos_r[i], r + 1);
                cnt -= (out - nums[i]) * (min_pos - i) as i32;
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

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();

    let first_line = lines_iter.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let nums_size: usize = first_line_split.next().unwrap().parse().unwrap();
    let k: i32 = first_line_split.next().unwrap().parse().unwrap();

    let nums_line = lines_iter.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), nums_size);

    // Computing the result
    let result = count_non_decreasing_subarrays(&nums, k);

    // Output the result
    println!("{}", result);
}