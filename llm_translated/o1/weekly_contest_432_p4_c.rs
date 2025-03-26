// Translation of the C solution for LeetCode Weekly Contest 432 Problem 4 into Rust.
// This Rust program preserves the EXACT same algorithm and I/O format as the original C code.
// It uses idiomatic Rust with basic error handling where appropriate.
//
// HOW TO BUILD/RUN:
// 1. Save this file as main.rs (or any .rs file).
// 2. Run: "rustc main.rs"
// 3. Execute: "./main"
// 4. Provide the input via stdin as in the original C program.
//
// The input format is exactly the same as the C program:
// First line contains two integers: numsSize k
// Then numsSize integers follow (white-space separated, can be on the same line or multiple lines).
// The output is a single integer (the count), printed to stdout.

use std::io::{self, BufRead};
use std::process;

// A simple integer stack with a fixed capacity.
struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: usize,
}

impl Stack {
    fn create_stack(capacity: usize) -> Self {
        // Pre-allocate the vector to the given capacity, fill with 0's for index-safety.
        Stack {
            data: vec![0; capacity],
            top: -1,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn push(&mut self, item: i32) {
        if (self.top as usize) < self.capacity.wrapping_sub(1) {
            self.top += 1;
            self.data[self.top as usize] = item;
        }
        // If the stack is full, we do nothing (same as the C code).
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            // Return -1 if empty (same behavior as the C code).
            return -1;
        }
        let item = self.data[self.top as usize];
        self.top -= 1;
        item
    }

    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.top as usize]
    }
}

// A simple integer deque with a ring buffer.
struct Deque {
    data: Vec<i32>,
    front: i32,
    rear: i32,
    size: i32,
    capacity: i32,
}

impl Deque {
    fn create_deque(capacity: i32) -> Self {
        // Pre-allocate the vector, fill with 0's for index-safety.
        Deque {
            data: vec![0; capacity as usize],
            front: 0,
            rear: -1,
            size: 0,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push_back(&mut self, item: i32) {
        if self.size == self.capacity {
            // Deque is full, do nothing (same as the C code).
            return;
        }
        self.rear = (self.rear + 1).rem_euclid(self.capacity);
        self.data[self.rear as usize] = item;
        self.size += 1;
    }

    fn pop_back(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // same as C code for empty
        }
        let item = self.data[self.rear as usize];
        self.rear = (self.rear - 1 + self.capacity).rem_euclid(self.capacity);
        self.size -= 1;
        item
    }

    fn pop_front(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // same as C code for empty
        }
        let item = self.data[self.front as usize];
        self.front = (self.front + 1).rem_euclid(self.capacity);
        self.size -= 1;
        item
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1; // same as C code for empty
        }
        self.data[self.front as usize]
    }

    fn back(&self) -> i32 {
        if self.is_empty() {
            return -1; // same as C code for empty
        }
        self.data[self.rear as usize]
    }
}

// A simple dynamic array for i32 with manual capacity management.
struct Vector {
    data: Vec<i32>,
    size: usize,
    capacity: usize,
}

impl Vector {
    fn create_vector(initial_capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(initial_capacity),
            size: 0,
            capacity: initial_capacity,
        }
    }

    fn push_back(&mut self, item: i32) {
        // If size reaches capacity, we double the capacity to mimic the C code's realloc usage.
        if self.size == self.capacity {
            let new_capacity = self.capacity * 2;
            self.data.reserve(new_capacity - self.capacity);
            self.capacity = new_capacity;
        }
        self.data.push(item);
        self.size += 1;
    }
}

// This function replicates the logic from the C code's countNonDecreasingSubarrays:
// It calculates the number of subarrays of a non-decreasing sequence whose sum of (max-min)
// is at least k (based on the logic in the code).
// The original comment says: "计算非递减子数组中和至少为k的子数组数量" (the code's logic handles that).
fn count_non_decreasing_subarrays(nums: &[i32], nums_size: usize, k: i64) -> i64 {
    // Create a Vec<Vector> to mimic Vector** g in C.
    let mut g: Vec<Vector> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        g.push(Vector::create_vector(10));
    }

    // pos_r array as in the C code
    let mut pos_r = vec![nums_size; nums_size];

    // Using a stack to find the next position of a larger-or-equal element
    let mut st = Stack::create_stack(nums_size);
    for i in 0..nums_size {
        let x = nums[i];
        while !st.is_empty() && x >= nums[st.top() as usize] {
            let top_index = st.top() as usize;
            pos_r[top_index] = i;
            st.pop();
        }
        if !st.is_empty() {
            let top_index = st.top() as usize;
            g[top_index].push_back(i as i32);
        }
        st.push(i as i32);
    }

    let mut ans: i64 = 0;
    let mut cnt: i64 = 0; // track the partial sum logic
    let mut l: i32 = 0;   // left pointer

    // Create a Deque for the max elements (like in the C code).
    let mut q = Deque::create_deque(nums_size as i32);

    // Expand right pointer
    for r in 0..nums_size {
        let x = nums[r];
        // Maintain the deque in descending order of nums[...] values
        while !q.is_empty() && nums[q.back() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);

        // Increase cnt by difference between deque front's value and x
        let front_val = nums[q.front() as usize];
        cnt += (front_val - x) as i64;

        // Shrink from the left while cnt > k
        while cnt > k {
            let out = nums[l as usize];
            // For each "next greater or equal" link from g[l]
            for &i_val in &g[l as usize].data {
                let i_i32 = i_val;
                if i_i32 as usize >= nums_size {
                    break;
                }
                if i_i32 > r as i32 {
                    break;
                }
                let min_pos = if (pos_r[i_i32 as usize] as i32) < (r as i32 + 1) {
                    pos_r[i_i32 as usize] as i32
                } else {
                    r as i32 + 1
                };
                // Decrease cnt by (out - nums[i]) for the segment from i..min_pos-1
                cnt -= (out as i64 - nums[i_i32 as usize] as i64)
                    * ((min_pos - i_i32) as i64);
            }
            l += 1;
            if !q.is_empty() && q.front() < l {
                q.pop_front();
            }
        }
        // Count valid subarrays ending at r
        ans += (r as i64 - l as i64 + 1);
    }

    ans
}

// Helper function to read all tokens from stdin into a vec of strings.
// This allows us to handle the same "scanf" style input from multiple lines.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error reading a line from stdin.");
                process::exit(1);
            }
        };
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }
    tokens
}

fn main() {
    // Read all tokens from stdin
    let tokens = read_tokens();
    let mut idx = 0;

    // We need at least 2 tokens (numsSize, k)
    if tokens.len() < 2 {
        eprintln!("Error reading input for numsSize and k");
        process::exit(1);
    }

    // Parse numsSize
    let nums_size: usize = match tokens[idx].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error parsing numsSize");
            process::exit(1);
        }
    };
    idx += 1;

    // Parse k
    let k: i64 = match tokens[idx].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error parsing k");
            process::exit(1);
        }
    };
    idx += 1;

    // Now read numsSize integers into nums
    if tokens.len() < idx + nums_size {
        eprintln!("Not enough integers provided for nums array");
        process::exit(1);
    }

    let mut nums = Vec::with_capacity(nums_size);
    for i in 0..nums_size {
        let val: i32 = match tokens[idx + i].parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error parsing nums[{}]", i);
                process::exit(1);
            }
        };
        nums.push(val);
    }

    // Compute the result
    let result = count_non_decreasing_subarrays(&nums, nums_size, k);

    // Print the result to stdout
    println!("{}", result);
}