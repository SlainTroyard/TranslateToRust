use std::io;
use std::process::exit;

// A simple stack structure to mimic the original C stack behavior
// -- storing i32 values; internally uses a Vec<i32> for data
// -- has a capacity limit, just like the C code
// -- returns -1 on pop or top if empty
struct Stack {
    data: Vec<i32>,
    capacity: usize,
}

impl Stack {
    // Create a new stack with the given capacity
    fn with_capacity(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Return the current size of the stack
    fn size(&self) -> usize {
        self.data.len()
    }

    // Push a value onto the stack (do nothing if at capacity limit)
    fn push(&mut self, value: i32) {
        if self.data.len() < self.capacity {
            self.data.push(value);
        }
        // If the stack is "full" like in C, we do nothing
    }

    // Pop the top value from the stack, returning -1 if empty
    fn pop(&mut self) -> i32 {
        match self.data.pop() {
            Some(val) => val,
            None => -1,
        }
    }

    // Return the top value without removing it, returning -1 if empty
    fn top(&self) -> i32 {
        match self.data.last() {
            Some(&val) => val,
            None => -1,
        }
    }
}

// Compute the helper "count" function as in the C code
// If m > k, compute (m*2 - k + 1)*k/2, else ((m + 1)*m)/2
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ( (m * 2 - k + 1) as i64 * k as i64 ) / 2
    } else {
        ( (m + 1) as i64 * m as i64 ) / 2
    }
}

// Calculate the sum of subarray minimums with the logic given in C code
fn sum_subarray_mins(nums: &mut [i32], k: i32) -> i64 {
    let mut res: i64 = 0;
    let n = nums.len();
    let mut stack = Stack::with_capacity(n + 1);

    // Sentinel push
    stack.push(-1);

    for r in 0..n {
        // While we can pop and the current value is less or equal
        // to handle "nums[top(stack)] >= nums[r]"
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top();
            // Convert to i32 and do arithmetic carefully
            let total = count((r as i32) - l - 1, k)
                      - count(i - l - 1, k)
                      - count((r as i32) - i - 1, k);
            res += nums[i as usize] as i64 * total;
        }
        // Push the current index
        stack.push(r as i32);
    }

    res
}

// Calculate the final value, combining min and max subarray sums
// by using sum_subarray_mins on the original array and its negation
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();

    // Create a temporary array of size n+1, and copy over nums
    let mut temp = vec![0i32; n + 1];
    temp[..n].copy_from_slice(nums);

    // Sentinel: INT_MIN/2
    // In Rust, i32::MIN / 2 = -1073741824
    // which matches the typical INT_MIN/2 in C in a 32-bit environment.
    temp[n] = i32::MIN / 2;

    // Sum of subarray minimums
    let mut ans = sum_subarray_mins(&mut temp, k);

    // Negate all elements
    for i in 0..(n + 1) {
        temp[i] = -temp[i];
    }
    // Restore the sentinel element sign to match the original approach
    temp[n] = -temp[n];

    // Subtract the sum of subarray minimums of the negated array
    ans -= sum_subarray_mins(&mut temp, k);

    ans
}

fn main() {
    // Read n and k
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        eprintln!("Error reading input for n and k");
        exit(1);
    }
    let mut parts = line.split_whitespace();
    let n: usize = match parts.next().and_then(|p| p.parse::<usize>().ok()) {
        Some(val) => val,
        None => {
            eprintln!("Error reading input for n and k");
            exit(1);
        }
    };
    let k: i32 = match parts.next().and_then(|p| p.parse::<i32>().ok()) {
        Some(val) => val,
        None => {
            eprintln!("Error reading input for n and k");
            exit(1);
        }
    };

    // "Allocate" the array (vector in Rust)
    // In C, there's a check for memory allocation failure, but Rust's Vec should succeed
    // unless OOM, in which case it will panic. Mimic the behavior:
    let mut nums = Vec::with_capacity(n);
    if nums.capacity() < n {
        eprintln!("Memory allocation failed for nums array");
        exit(1);
    }

    // Read n integers for nums
    let mut read_count = 0;
    let mut input_buffer = String::new();
    while read_count < n {
        // If we finish read_line with no new data, break
        input_buffer.clear();
        if io::stdin().read_line(&mut input_buffer).unwrap_or(0) == 0 {
            // If we can't read enough integers, mimic the original error
            eprintln!("Error reading input for nums[{}]", read_count);
            exit(1);
        }
        let mut split_iter = input_buffer.split_whitespace();
        while let Some(token) = split_iter.next() {
            let val = match token.parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Error reading input for nums[{}]", read_count);
                    exit(1);
                }
            };
            nums.push(val);
            read_count += 1;
            if read_count == n {
                break;
            }
        }
    }

    // Call the function to compute the result
    let result = min_max_subarray_sum(&nums, k);

    // Print the result
    println!("{}", result);
}