use std::io::{self, BufRead, Write};

// Simple integer stack implementation
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
        if self.data.len() < self.data.capacity() {
            self.data.push(item);
        }
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
            rear: 0,
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

    fn pop_back(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let item = self.data[self.rear];
            self.rear = (self.rear + self.capacity - 1) % self.capacity;
            Some(item)
        }
    }

    fn pop_front(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let item = self.data[self.front];
            self.front = (self.front + 1) % self.capacity;
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
}

fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vector> = (0..n).map(|_| Vector::new(10)).collect();
    let mut pos_r = vec![n; n];

    let mut st = Stack::new(n);
    for (i, &x) in nums.iter().enumerate() {
        while !st.is_empty() && x >= nums[st.top().unwrap() as usize] {
            let j = st.pop().unwrap();
            pos_r[j as usize] = i;
        }
        if !st.is_empty() {
            g[st.top().unwrap() as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    let mut ans = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = Deque::new(n);

    for r in 0..n {
        let x = nums[r];
        while !q.is_empty() && nums[q.back().unwrap() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += nums[q.front().unwrap() as usize] - x;

        while cnt > k {
            let out = nums[l];
            for &i in &g[l] {
                if i > r as i32 {
                    break;
                }
                let min_pos = if pos_r[i as usize] < r + 1 { pos_r[i as usize] } else { r + 1 };
                cnt -= (out - nums[i as usize]) * (min_pos - i as usize);
            }
            l += 1;
            if !q.is_empty() && q.front().unwrap() as usize < l {
                q.pop_front();
            }
        }
        ans += r - l + 1;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();

    // Read the first line to get numsSize and k
    stdin_lock.read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let nums_size: usize = iter.next().and_then(|s| s.parse().ok()).expect("Invalid numsSize");
    let k: i32 = iter.next().and_then(|s| s.parse().ok()).expect("Invalid k");

    // Allocate memory and read the array
    let mut nums = vec![0; nums_size];
    for i in 0..nums_size {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read input");
        nums[i] = input.trim().parse().expect("Invalid nums value");
    }

    // Call the function to compute the result
    let result = count_non_decreasing_subarrays(&nums, k);

    // Output the result
    writeln!(stdout, "{}", result).expect("Failed to write output");
}