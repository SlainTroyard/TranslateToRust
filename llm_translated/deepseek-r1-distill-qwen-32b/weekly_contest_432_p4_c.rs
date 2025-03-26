use std::io;
use std::vec::Vec;

struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: i32,
}

impl Stack {
    fn new(capacity: i32) -> Option<Stack> {
        Some(Stack {
            data: Vec::with_capacity(capacity as usize),
            top: -1,
            capacity,
        })
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn push(&mut self, item: i32) -> bool {
        if self.top == self.capacity - 1 {
            return false;
        }
        self.top += 1;
        self.data.push(item);
        true
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let val = self.data[self.top as usize];
        self.top -= 1;
        val
    }

    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.top as usize]
    }
}

struct Deque {
    data: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
}

impl Deque {
    fn new(capacity: usize) -> Option<Deque> {
        Some(Deque {
            data: Vec::with_capacity(capacity),
            front: 0,
            rear: 0,
            size: 0,
            capacity,
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push_back(&mut self, item: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.data.push(item);
        self.size += 1;
        true
    }

    fn pop_back(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let val = self.data[self.rear];
        self.rear = (self.rear - 1 + self.capacity) % self.capacity;
        self.size -= 1;
        val
    }

    fn pop_front(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let val = self.data[self.front];
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        val
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front]
    }

    fn back(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.rear]
    }
}

struct Vector {
    data: Vec<i32>,
    size: usize,
    capacity: usize,
}

impl Vector {
    fn new(capacity: usize) -> Option<Vector> {
        Some(Vector {
            data: Vec::with_capacity(capacity),
            size: 0,
            capacity,
        })
    }

    fn push_back(&mut self, item: i32) {
        if self.size == self.capacity {
            let new_capacity = self.capacity * 2;
            self.data.resize(new_capacity, 0);
            self.capacity = new_capacity;
        }
        self.data.push(item);
        self.size += 1;
    }
}

fn count_non_decreasing_subarrays(nums: &Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut pos_r = vec![n as i32; n];

    let mut st = Stack::new(n as i32).unwrap();
    for i in 0..n {
        let x = nums[i];
        while !st.is_empty() && x >= nums[st.top() as usize] {
            let idx = st.pop() as usize;
            pos_r[idx] = i as i32;
        }
        if !st.is_empty() {
            let top = st.top() as usize;
            g[top].push(i as i32);
        }
        st.push(i as i32);
    }

    let mut ans = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = Deque::new(n).unwrap();

    for r in 0..n {
        let x = nums[r];
        while !q.is_empty() && nums[q.back() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += nums[q.front() as usize] - x;

        while cnt > k {
            let out = nums[l];
            for j in 0..g[l].len() {
                let i = g[l][j];
                if i > r as i32 {
                    break;
                }
                let min_pos = if pos_r[i as usize] < (r + 1) as i32 {
                    pos_r[i as usize]
                } else {
                    (r + 1) as i32
                };
                cnt -= (out - nums[i as usize]) * (min_pos - i);
            }
            l += 1;
            if !q.is_empty() && q.front() < l as i32 {
                q.pop_front();
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let nums_size = iter.next().unwrap().parse::<i32>().unwrap();
    let k = iter.next().unwrap().parse::<i32>().unwrap();
    
    let mut nums = Vec::new();
    for _ in 0..nums_size {
        let num = iter.next().unwrap().parse::<i32>().unwrap();
        nums.push(num);
    }
    
    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);
}