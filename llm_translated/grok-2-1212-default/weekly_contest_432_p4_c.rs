use std::io::{self, BufRead, Write};
use std::collections::VecDeque;

// Stack implementation
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack { data: Vec::with_capacity(capacity) }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, item: i32) {
        if self.data.len() < self.data.capacity() {
            self.data.push(item);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<i32> {
        self.data.last().cloned()
    }
}

// Deque implementation
struct Deque {
    data: VecDeque<i32>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
}

impl Deque {
    fn new(capacity: usize) -> Self {
        Deque {
            data: VecDeque::with_capacity(capacity),
            front: 0,
            rear: capacity - 1,
            size: 0,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push_back(&mut self, item: i32) {
        if self.size < self.capacity {
            self.rear = (self.rear + 1) % self.capacity;
            if self.size == self.data.len() {
                self.data.push_back(item);
            } else {
                self.data[self.rear] = item;
            }
            self.size += 1;
        }
    }

    fn pop_back(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let item = self.data[self.rear];
            self.rear = (self.rear + self.capacity - 1) % self.capacity;
            self.size -= 1;
            Some(item)
        }
    }

    fn pop_front(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let item = self.data[self.front];
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Some(item)
        }
    }

    fn front(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[self.front])
        }
    }

    fn back(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[self.rear])
        }
    }
}

// Vector implementation
struct Vector {
    data: Vec<i32>,
}

impl Vector {
    fn new(capacity: usize) -> Self {
        Vector { data: Vec::with_capacity(capacity) }
    }

    fn push_back(&mut self, item: i32) {
        if self.data.len() == self.data.capacity() {
            self.data.reserve(self.data.capacity() * 2);
        }
        self.data.push(item);
    }
}

// Function to count non-decreasing subarrays
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vector> = vec![Vector::new(10); n];
    let mut pos_r: Vec<usize> = vec![n; n];

    // Find next greater or equal element for each position
    let mut st = Stack::new(n);
    for (i, &x) in nums.iter().enumerate() {
        while !st.is_empty() && x >= nums[*st.top().as_ref().unwrap() as usize] {
            let top = st.pop().unwrap() as usize;
            pos_r[top] = i;
        }
        if let Some(top) = st.top() {
            g[top as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    // Calculate result
    let mut ans: i64 = 0;
    let mut cnt: i64 = 0;
    let mut l: usize = 0;
    let mut q = Deque::new(n);

    for r in 0..n {
        let x = nums[r];
        while !q.is_empty() && nums[*q.back().as_ref().unwrap() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += nums[*q.front().as_ref().unwrap() as usize] as i64 - x as i64;

        while cnt > k as i64 {
            let out = nums[l];
            for &i in &g[l].data {
                let i = i as usize;
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < r + 1 { pos_r[i] } else { r + 1 };
                cnt -= (out - nums[i]) as i64 * (min_pos - i) as i64;
            }
            l += 1;
            if !q.is_empty() && *q.front().as_ref().unwrap() as usize < l {
                q.pop_front();
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read numsSize and k
    stdin.lock().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let nums_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read nums array
    input.clear();
    stdin.lock().read_line(&mut input)?;
    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate result
    let result = count_non_decreasing_subarrays(&nums, k);

    // Output result
    writeln!(stdout, "{}", result)?;

    Ok(())
}