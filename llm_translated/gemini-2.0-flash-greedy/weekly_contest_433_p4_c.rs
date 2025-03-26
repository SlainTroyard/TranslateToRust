use std::cmp::min;
use std::io;
use std::io::Read;

// Stack implementation
struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: usize,
}

// Creating a stack
impl Stack {
    fn new(capacity: usize) -> Stack {
        Stack {
            data: Vec::with_capacity(capacity),
            top: -1,
            capacity,
        }
    }

    // Checking if the stack is empty
    fn is_empty(&self) -> bool {
        self.top == -1
    }

    // Checking the size of the stack
    fn size(&self) -> usize {
        (self.top + 1) as usize
    }

    // Push onto the stack
    fn push(&mut self, value: i32) {
        if (self.top as usize) == self.capacity - 1 {
            return; // Stack is full
        }
        self.top += 1;
        if self.data.len() <= self.top as usize {
            self.data.push(value);
        } else {
            self.data[self.top as usize] = value;
        }
    }

    // Pop from the stack
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        let value = self.data[self.top as usize];
        self.top -= 1;
        value
    }

    // View the top element of the stack
    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        self.data[self.top as usize]
    }
}

// Helper function to calculate combinations
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m as i64 * 2 - k as i64 + 1) as i64 * k as i64) / 2
    } else {
        ((m as i64 + 1) * m as i64) / 2
    }
}

// Function to calculate the sum of minimums of subarrays
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res: i64 = 0;
    let mut stack = Stack::new(nums.len() + 1);

    stack.push(-1); // Add a sentinel element

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top();
            let cnt = count(r as i32 - l - 1, k) - count(i - l - 1, k) - count(r as i32 - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Function to calculate the sum of min-max subarrays
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // Create a temporary array containing the original array and an extra element
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Add a very small value as a sentinel

    // Calculate the sum of minimums of subarrays
    let mut ans = sum_subarray_mins(&temp, k);

    // Invert all elements and calculate again
    for i in 0..temp.len() {
        temp[i] = -temp[i];
    }
    temp[temp.len() - 1] = -temp[temp.len() - 1]; // Restore the sign of the sentinel element

    // Subtract the inverted result from the total
    ans -= sum_subarray_mins(&temp, k);

    ans
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();

    let n: i32 = parts.next().unwrap().parse().expect("Invalid input for n");
    let k: i32 = parts.next().unwrap().parse().expect("Invalid input for k");

    let second_line = lines.next().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    // Call function to calculate result
    let result = min_max_subarray_sum(&nums, k);

    // Output result
    println!("{}", result);

    Ok(())
}