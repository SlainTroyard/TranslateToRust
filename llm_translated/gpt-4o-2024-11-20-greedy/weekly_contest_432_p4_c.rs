use std::collections::VecDeque;
use std::io::{self, BufRead};

// Simple integer stack implementation
struct Stack {
    data: Vec<i32>,
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

    fn push(&mut self, item: i32) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<i32> {
        self.data.last().copied()
    }
}

// Simple integer deque implementation
struct Deque {
    data: VecDeque<i32>,
}

impl Deque {
    fn new(capacity: usize) -> Self {
        Self {
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

    fn front(&self) -> Option<i32> {
        self.data.front().copied()
    }

    fn back(&self) -> Option<i32> {
        self.data.back().copied()
    }
}

// Simple dynamic array implementation
struct Vector {
    data: Vec<i32>,
}

impl Vector {
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push_back(&mut self, item: i32) {
        self.data.push(item);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }
}

// Function to calculate the number of non-decreasing subarrays with sum at least k
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let nums_size = nums.len();

    // Create graph `g` and `pos_r` array
    let mut g: Vec<Vector> = (0..nums_size).map(|_| Vector::new(10)).collect();
    let mut pos_r = vec![nums_size; nums_size];

    // Use stack to find the next greater or equal element for each position
    let mut st = Stack::new(nums_size);
    for (i, &x) in nums.iter().enumerate() {
        while !st.is_empty() && x >= nums[st.top().unwrap() as usize] {
            pos_r[st.top().unwrap() as usize] = i;
            st.pop();
        }
        if let Some(top) = st.top() {
            g[top as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    // Calculate the result
    let mut ans = 0i64;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = Deque::new(nums_size);

    for (r, &x) in nums.iter().enumerate() {
        while !q.is_empty() && nums[q.back().unwrap() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += nums[q.front().unwrap() as usize] - x;

        while cnt > k {
            let out = nums[l];
            for j in 0..g[l].size() {
                let i = g[l].get(j).unwrap() as usize;
                if i > r {
                    break;
                }
                let min_pos = pos_r[i].min(r + 1);
                cnt -= (out - nums[i]) * (min_pos - i) as i32;
            }
            l += 1;
            if !q.is_empty() && q.front().unwrap() < l as i32 {
                q.pop_front();
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let nums_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_line_iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Call the function and print the result
    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);
}