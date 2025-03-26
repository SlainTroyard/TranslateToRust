use std::io::{self, BufRead};

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

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data.pop().unwrap()
        }
    }

    fn top(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            *self.data.last().unwrap()
        }
    }
}

// Deque implementation
struct Deque {
    data: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
}

impl Deque {
    fn new(capacity: usize) -> Self {
        Deque {
            data: vec![0; capacity],
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
            self.data[self.rear] = item;
            self.size += 1;
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let item = self.data[self.rear];
            self.rear = (self.rear + self.capacity - 1) % self.capacity;
            self.size -= 1;
            item
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let item = self.data[self.front];
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            item
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front]
        }
    }

    fn back(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.rear]
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
        self.data.push(item);
    }
}

// Function to count non-decreasing subarrays with sum at least k
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g = vec![Vector::new(10); n];
    let mut pos_r = vec![n as i32; n];

    // Find the next greater or equal element for each position
    let mut st = Stack::new(n);
    for (i, &x) in nums.iter().enumerate() {
        while !st.is_empty() && x >= nums[st.top() as usize] {
            pos_r[st.top() as usize] = i as i32;
            st.pop();
        }
        if !st.is_empty() {
            g[st.top() as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    // Calculate the result
    let mut ans = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = Deque::new(n);

    for (r, &x) in nums.iter().enumerate() {
        while !q.is_empty() && nums[q.back() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += (nums[q.front() as usize] - x) as i64;

        while cnt > k as i64 {
            let out = nums[l];
            for &i in &g[l].data {
                if i as usize > r {
                    break;
                }
                let min_pos = if pos_r[i as usize] < (r + 1) as i32 {
                    pos_r[i as usize]
                } else {
                    (r + 1) as i32
                };
                cnt -= (out - nums[i as usize]) as i64 * (min_pos - i) as i64;
            }
            l += 1;
            if !q.is_empty() && q.front() < l as i32 {
                q.pop_front();
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let nums_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate result
    let result = count_non_decreasing_subarrays(&nums, k);

    // Output result
    println!("{}", result);

    Ok(())
}