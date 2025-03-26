use std::io;
use std::cmp::max;

// Stack implementation
#[derive(Debug)]
struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: usize,
}

// Create a stack
impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
            top: -1,
            capacity,
        }
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.top == -1
    }

    // Get the size of the stack
    fn size(&self) -> i32 {
        self.top + 1
    }

    // Push an element onto the stack
    fn push(&mut self, value: i32) {
        if (self.top as usize) == self.capacity - 1 {
            return; // Stack is full
        }
        self.top += 1;
        if (self.top as usize) >= self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.top as usize] = value;
        }
    }

    // Pop an element from the stack
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        let value = self.data[self.top as usize];
        self.top -= 1;
        value
    }

    // Get the top element of the stack
    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1; // Stack is empty
        }
        self.data[self.top as usize]
    }
}

// Helper function to calculate combinations
fn count(m: i64, k: i64) -> i64 {
    if m > k {
        (m * 2 - k + 1) * k / 2
    } else {
        (m + 1) * m / 2
    }
}

// Calculate the sum of subarray minimums
fn sum_subarray_mins(nums: &Vec<i32>, k: i64) -> i64 {
    let mut res: i64 = 0;
    let mut stack = Stack::new(nums.len() + 1);

    stack.push(-1); // Add a sentinel element

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top();
            let cnt: i64 = count((r as i64) - (l as i64) - 1, k) - count((i as i64) - (l as i64) - 1, k) - count((r as i64) - (i as i64) - 1, k);
            res += (nums[i as usize] as i64) * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// Calculate the min-max subarray sum
fn min_max_subarray_sum(nums: &Vec<i32>, k: i64) -> i64 {
    // Create a temporary array containing the original array and an extra element
    let mut temp = nums.clone();
    temp.push(i32::MIN / 2); // Add a very small value as a sentinel

    // Calculate the sum of subarray minimums
    let mut ans = sum_subarray_mins(&temp, k);

    // Invert all elements and calculate again
    let mut temp_neg = temp.clone();
    for i in 0..temp_neg.len() {
        temp_neg[i] = -temp_neg[i];
    }
    temp_neg[temp_neg.len() - 1] = -temp_neg[temp_neg.len() - 1];

    // Subtract the inverted result from the total
    ans -= sum_subarray_mins(&temp_neg, k);

    ans
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    for s in input.trim().split_whitespace() {
        nums.push(s.parse::<i32>().unwrap());
    }

    // Calculate the result
    let result = min_max_subarray_sum(&nums, k);

    // Print the result
    println!("{}", result);

    Ok(())
}