use std::io::{self, BufRead, Write};

// Simple integer stack implementation
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
        Vector { data: Vec::with_capacity(capacity) }
    }

    fn push_back(&mut self, item: i32) {
        self.data.push(item);
    }

    fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

fn count_non_decreasing_subarrays(nums: &Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vector> = (0..n).map(|_| Vector::new(10)).collect();
    let mut pos_r: Vec<usize> = vec![n; n];

    // Use stack to find the next greater or equal element for each position
    let mut st = Stack::new(n);
    for (i, &x) in nums.iter().enumerate() {
        while !st.is_empty() && x >= nums[st.top().unwrap() as usize] {
            let j = st.pop().unwrap() as usize;
            pos_r[j] = i;
        }
        if !st.is_empty() {
            g[st.top().unwrap() as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    // Calculate the result
    let mut ans: i64 = 0;
    let mut cnt: i32 = 0;
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
            for j in 0..g[l].len() {
                let i = g[l].get(j).unwrap() as usize;
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < r + 1 { pos_r[i] } else { r + 1 };
                cnt -= (out - nums[i]) * (min_pos - i) as i32;
            }
            l += 1;
            if q.front().unwrap() as usize < l {
                q.pop_front();
            }
        }
        ans += r as i64 - l as i64 + 1;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read input
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<&str> = buffer.trim().split_whitespace().collect();
    let nums_size: usize = input[0].parse().expect("Invalid numsSize");
    let k: i32 = input[1].parse().expect("Invalid k");

    buffer.clear();
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        let num: i32 = buffer.trim().parse().expect("Invalid number");
        nums.push(num);
    }

    // Call function to calculate the result
    let result = count_non_decreasing_subarrays(&nums, k);

    // Output the result
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}