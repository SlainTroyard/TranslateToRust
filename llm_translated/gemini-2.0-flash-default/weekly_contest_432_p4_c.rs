use std::collections::VecDeque;
use std::io;

// 简单的整数栈实现
struct Stack {
    data: Vec<i32>,
    top: i32,
    capacity: usize,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
            top: -1,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn push(&mut self, item: i32) {
        if (self.top as usize) == self.capacity - 1 {
            return; // 栈满
        }
        self.top += 1;
        if (self.top as usize) < self.data.len() {
            self.data[self.top as usize] = item;
        } else {
            self.data.push(item);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // 栈空
        }
        let item = self.data[self.top as usize];
        self.top -= 1;
        item
    }

    fn top(&self) -> i32 {
        if self.is_empty() {
            return -1; // 栈空
        }
        self.data[self.top as usize]
    }
}

// 简单的整数双端队列实现
struct Deque {
    data: Vec<i32>,
    front: usize,
    rear: i32,
    size: usize,
    capacity: usize,
}

impl Deque {
    fn new(capacity: usize) -> Self {
        Deque {
            data: Vec::with_capacity(capacity),
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
            return; // 队列满
        }
        self.rear = (self.rear + 1) % (self.capacity as i32);
        if (self.rear as usize) < self.data.len() {
            self.data[self.rear as usize] = item;
        } else {
            self.data.push(item);
        }
        self.size += 1;
    }

    fn pop_back(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // 队列空
        }
        let item = self.data[self.rear as usize];
        self.rear = (self.rear - 1 + (self.capacity as i32)) % (self.capacity as i32);
        self.size -= 1;
        item
    }

    fn pop_front(&mut self) -> i32 {
        if self.is_empty() {
            return -1; // 队列空
        }
        let item = self.data[self.front];
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        item
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1; // 队列空
        }
        self.data[self.front]
    }

    fn back(&self) -> i32 {
        if self.is_empty() {
            return -1; // 队列空
        }
        self.data[self.rear as usize]
    }
}

// 动态数组实现
struct Vector {
    data: Vec<i32>,
    size: usize,
    capacity: usize,
}

impl Vector {
    fn new(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
            size: 0,
            capacity,
        }
    }

    fn push_back(&mut self, item: i32) {
        if self.size == self.capacity {
            // 扩容
            let new_capacity = self.capacity * 2;
            self.data.reserve(new_capacity - self.capacity);
            self.capacity = new_capacity;
        }
        if self.size < self.data.len() {
            self.data[self.size] = item;
        } else {
            self.data.push(item);
        }
        self.size += 1;
    }
}

// 计算非递减子数组中和至少为k的子数组数量
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let nums_size = nums.len();

    // 创建图g和pos_r数组
    let mut g: Vec<Vector> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        g.push(Vector::new(10)); // 初始容量为10
    }

    let mut pos_r: Vec<i32> = vec![nums_size as i32; nums_size];

    // 使用栈找到每个位置的下一个更大或相等的元素
    let mut st = Stack::new(nums_size);
    for i in 0..nums_size {
        let x = nums[i];
        while !st.is_empty() && x >= nums[st.top() as usize] {
            pos_r[st.top() as usize] = i as i32;
            st.pop();
        }
        if !st.is_empty() {
            g[st.top() as usize].push_back(i as i32);
        }
        st.push(i as i32);
    }

    // 计算结果
    let mut ans: i64 = 0;
    let mut cnt: i32 = 0;
    let mut l: usize = 0;
    let mut q = Deque::new(nums_size);

    for r in 0..nums_size {
        let x = nums[r];
        while !q.is_empty() && nums[q.back() as usize] <= x {
            q.pop_back();
        }
        q.push_back(r as i32);
        cnt += nums[q.front()] - x;

        while cnt > k {
            let out = nums[l];
            for j in 0..g[l].size {
                let i = g[l].data[j] as usize;
                if i > r {
                    break;
                }
                let min_pos = if pos_r[i] < (r + 1) as i32 {
                    pos_r[i] as usize
                } else {
                    r + 1
                };
                cnt -= (out - nums[i]) * ((min_pos - i) as i32);
            }
            l += 1;
            if !q.is_empty() && q.front() < l {
                q.pop_front();
            }
        }
        ans += (r - l + 1) as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();

    let nums_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(nums_size);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    for num_str in input.trim().split_whitespace() {
        nums.push(num_str.parse().unwrap());
    }

    let result = count_non_decreasing_subarrays(&nums, k);

    println!("{}", result);

    Ok(())
}