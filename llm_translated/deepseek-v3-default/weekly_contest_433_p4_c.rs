use std::io::{self, BufRead};
use std::cmp;

struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: i32,
}

impl Stack {
    fn new(capacity: i32) -> Self {
        Stack {
            data: vec![0; capacity as usize],
            top: -1,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn size(&self) -> i32 {
        self.top + 1
    }

    fn push(&mut self, value: i32) {
        if self.top == self.capacity - 1 {
            return;
        }
        self.top += 1;
        self.data[self.top as usize] = value;
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let value = self.data[self.top as usize];
        self.top -= 1;
        value
    }

    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.top as usize]
    }
}

fn count(m: i32, k: i32) -> i64 {
    if m > k {
        (m * 2 - k + 1) as i64 * k as i64 / 2
    } else {
        (m + 1) as i64 * m as i64 / 2
    }
}

fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new(nums.len() as i32 + 1);

    stack.push(-1); // Sentinel element

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top();
            let cnt = count((r as i32) - l - 1, k) - count(i - l - 1, k) - count((r as i32) - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // Sentinel element

    let mut ans = sum_subarray_mins(&temp, k);

    for num in temp.iter_mut() {
        *num = -*num;
    }
    temp[temp.len() - 1] = -temp[temp.len() - 1]; // Restore sentinel element

    ans -= sum_subarray_mins(&temp, k);

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let num: i32 = line.trim().parse().unwrap();
        nums.push(num);
    }

    let result = min_max_subarray_sum(&nums, k);
    println!("{}", result);

    Ok(())
}