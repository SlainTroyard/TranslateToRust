use std::io;

pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> i32 {
        self.data.pop().expect("Stack underflow")
    }

    pub fn top(&self) -> i32 {
        *self.data.last().expect("Stack is empty when accessing top")
    }
}

fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m * 2 - k + 1) as i64) * k as i64 / 2
    } else {
        ((m + 1) as i64) * m as i64 / 2
    }
}

fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0i64;
    let mut stack = Stack::new();
    stack.push(-1); // sentinel

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top() as usize] >= nums[r] {
            let i = stack.pop();
            let l = stack.top() as usize;
            let m1 = r as i32 - l as i32 - 1;
            let m2 = i as i32 - l as i32 - 1;
            let m3 = r as i32 - i as i32 - 1;
            let cnt = count(m1, k) - count(m2, k) - count(m3, k);
            res += (nums[i as usize] as i64) * cnt;
        }
        stack.push(r as i32);
    }
    res
}

fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2); // sentinel

    let ans = sum_subarray_mins(&temp, k);

    // Flip elements except the sentinel's last adjustment
    for num in temp.iter_mut() {
        *num = -*num;
    }
    // revert the sentinel
    temp[n] = -temp[n]; // since the sentinel is at index n

    let ans2 = sum_subarray_mins(&temp, k);

    ans - ans2
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>());

    let n = match iter.next() {
        Some(Ok(n)) => n,
        _ => {
            eprintln!("Error reading input for n and k");
            return;
        }
    };

    let k = match iter.next() {
        Some(Ok(k)) => k,
        _ => {
            eprintln!("Error reading input for n and k");
            return;
        }
    };

    let nums: Vec<i32> = iter
        .by_ref()
        .take(n as usize)
        .map(|res| res.unwrap_or_else(|_| {
            eprintln!("Error reading input for nums");
            std::process::exit(1);
        }))
        .collect();

    if nums.len() != n as usize {
        eprintln!("Not enough elements for the array");
        return;
    }

    let result = min_max_subarray_sum(&nums, k);
    println!("{}", result);
}