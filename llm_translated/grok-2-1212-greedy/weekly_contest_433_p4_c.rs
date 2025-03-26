use std::io::{self, BufRead};

// Stack implementation
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    // Create a new stack
    fn new(capacity: usize) -> Stack {
        Stack { data: Vec::with_capacity(capacity) }
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the size of the stack
    fn size(&self) -> usize {
        self.data.len()
    }

    // Push a value onto the stack
    fn push(&mut self, value: i32) {
        if self.data.len() < self.data.capacity() {
            self.data.push(value);
        }
    }

    // Pop a value from the stack
    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    // Get the top element of the stack
    fn top(&self) -> Option<i32> {
        self.data.last().cloned()
    }
}

// Helper function to calculate the number of combinations
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
    } else {
        (m as i64 + 1) * m as i64 / 2
    }
}

// Calculate the sum of minimums of all subarrays
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new(nums.len() + 1);
    
    stack.push(-1); // Add sentinel element
    
    for r in 0..nums.len() {
        while stack.size() > 1 && nums[*stack.top().unwrap() as usize] >= nums[r] {
            let i = stack.pop().unwrap();
            let l = stack.top().unwrap();
            let cnt = count((r as i32 - l - 1) as i32, k) - count((i - l - 1) as i32, k) - count((r as i32 - i - 1) as i32, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }
    
    res
}

// Main function to calculate the min-max subarray sum
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // Create a temporary vector with the original array and an extra element
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Add a very small value as a sentinel
    
    // Calculate the sum of minimums of subarrays
    let mut ans = sum_subarray_mins(&temp, k);
    
    // Negate all elements and calculate again
    for i in 0..=nums.len() {
        temp[i] = -temp[i];
    }
    temp[nums.len()] = -temp[nums.len()]; // Restore the sign of the sentinel element
    
    // Subtract the sum of minimums of negated subarrays
    ans -= sum_subarray_mins(&temp, k);
    
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate result
    let result = min_max_subarray_sum(&nums, k);

    // Output result
    println!("{}", result);

    Ok(())
}