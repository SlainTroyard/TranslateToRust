use std::io::{self, Read};
use std::process;

struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<&i32> {
        self.data.last()
    }
}

fn count(m: i32, k: i32) -> i64 {
    let m = m as i64;
    let k = k as i64;
    if m > k {
        (m * 2 - k + 1) * k / 2
    } else {
        (m + 1) * m / 2
    }
}

fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0i64;
    let mut stack = Stack::new();
    stack.push(-1); // Sentinel element

    for r in 0..nums.len() {
        // Process elements in stack while current element is smaller than top
        while stack.size() > 1 && nums[*stack.top().unwrap() as usize] >= nums[r] {
            let i = stack.pop().unwrap();
            let l = *stack.top().unwrap();
            let m = (r as i32) - l - 1;
            let cnt = count(m, k) - count(i - l - 1, k) - count((r as i32) - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Add sentinel

    let ans = sum_subarray_mins(&temp, k);

    // Negate elements and adjust sentinel
    for num in temp.iter_mut() {
        *num = -*num;
    }
    let last_idx = temp.len() - 1;
    temp[last_idx] = -temp[last_idx]; // Restore sentinel

    let ans_neg = sum_subarray_mins(&temp, k);

    ans - ans_neg
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap_or_else(|_| {
        eprintln!("Error reading input");
        process::exit(1);
    });

    let mut tokens = input.split_whitespace();
    
    // Parse n and k
    let n = match tokens.next() {
        Some(s) => s.parse::<usize>().unwrap_or_else(|_| {
            eprintln!("Error parsing n");
            process::exit(1);
        }),
        None => {
            eprintln!("Error: missing n");
            process::exit(1);
        }
    };

    let k = match tokens.next() {
        Some(s) => s.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error parsing k");
            process::exit(1);
        }),
        None => {
            eprintln!("Error: missing k");
            process::exit(1);
        }
    };

    // Parse nums array
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| {
            s.parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Error parsing number");
                process::exit(1);
            })
        })
        .collect();

    if nums.len() != n {
        eprintln!("Error: expected {} numbers, got {}", n, nums.len());
        process::exit(1);
    }

    let result = min_max_subarray_sum(&nums, k);
    println!("{}", result);
}