use std::cmp::Ordering;
use std::io;
use std::io::Read;

macro_rules! father_node {
    ($i:expr) => {
        if $i == 0 {
            -1
        } else {
            ($i - 1) >> 1
        }
    };
}

macro_rules! left_node {
    ($i:expr) => {
        (($i) << 1) + 1
    };
}

macro_rules! right_node {
    ($i:expr) => {
        (($i) << 1) + 2
    };
}

macro_rules! higher_int {
    ($i:expr) => {
        (($i) >> 32) as i32
    };
}

macro_rules! lower_int {
    ($i:expr) => {
        (($i) & 0xFFFFFFFF) as i32
    };
}

macro_rules! merge_long {
    ($i:expr, $j:expr) => {
        (($i as i64) << 32 | ($j as i64)) as i64
    };
}

#[derive(Copy, Clone, Debug)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

struct PriorityQueue {
    arr: Vec<i64>,
    arr_size: usize,
}

impl PriorityQueue {
    fn new(capacity: usize) -> Self {
        PriorityQueue {
            arr: Vec::with_capacity(capacity),
            arr_size: 0,
        }
    }

    fn queue_push(&mut self, value: i64) {
        let mut son = self.arr_size;
        let mut father = father_node!(son);
        self.arr_size += 1;
        self.arr.resize(self.arr_size, 0); // Ensure capacity

        while father != -1 && value < self.arr[father as usize] {
            self.arr[son] = self.arr[father as usize];
            son = father as usize;
            father = father_node!(son);
        }
        self.arr[son] = value;
    }

    fn queue_pop(&mut self) {
        self.arr_size -= 1;
        let father = 0;
        let mut left = left_node!(father);
        let mut right = right_node!(father);
        let mut son;

        while (self.arr_size > left && self.arr[self.arr_size] > self.arr[left])
            || (self.arr_size > right && self.arr[self.arr_size] > self.arr[right])
        {
            if self.arr_size > right && self.arr[left] > self.arr[right] {
                son = right;
            } else {
                son = left;
            }

            self.arr[father] = self.arr[son];
            let father_temp = son;
            left = left_node!(father_temp);
            right = right_node!(father_temp);
            father = father_temp;

        }
        if self.arr_size < self.arr.len(){
            self.arr[father] = self.arr[self.arr_size];
        }


    }
}

fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut value = [0u32; 4];
    value[0] = (idx >> 48) as u32;
    value[1] = ((idx >> 32) & 0xFFFF) as u32;
    value[2] = ((idx >> 16) & 0xFFFF) as u32;
    value[3] = (idx & 0xFFFF) as u32; // Corrected this line

    value[count] = i as u32;
    for x in (0..count).rev() {
        if value[x] > i as u32 {
            value.swap(x, x + 1);
        }
    }

    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | value[3] as u64
}

fn maximum_weight(intervals: &Vec<Vec<i32>>) -> Vec<i32> {
    let intervals_size = intervals.len();
    let mut dp = vec![DpNode {
        max: [0; 4],
        idx: [0; 4],
    }; intervals_size];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };

    let mut left_queue = PriorityQueue::new(intervals_size);
    let mut right_queue = PriorityQueue::new(intervals_size);

    for i in 0..intervals_size {
        left_queue.queue_push(merge_long!(intervals[i][0], i as i32));
        right_queue.queue_push(merge_long!(intervals[i][1], i as i32));
    }

    let mut max: i64 = 0;
    let mut sel: u64 = 0xFFFFFFFFFFFFFFFF;
    let mut return_size: usize = 0;

    while left_queue.arr_size > 0 {
        let i = lower_int!(left_queue.arr[0]);
        let edge = higher_int!(left_queue.arr[0]);

        while right_queue.arr_size > 0 && edge > higher_int!(right_queue.arr[0]) {
            let j = lower_int!(right_queue.arr[0]);
            for k in 0..3 {
                if last.max[k] < dp[j as usize].max[k]
                    || (last.max[k] == dp[j as usize].max[k] && last.idx[k] > dp[j as usize].idx[k])
                {
                    last.max[k] = dp[j as usize].max[k];
                    last.idx[k] = dp[j as usize].idx[k];
                }
            }
            right_queue.queue_pop();
        }

        dp[i as usize] = last;

        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };

            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i, k)
                };

                if dp[i as usize].max[k] < intervals[i as usize][2] as i64 + prev
                    || (dp[i as usize].max[k] == intervals[i as usize][2] as i64 + prev
                        && dp[i as usize].idx[k] > idx)
                {
                    dp[i as usize].max[k] = intervals[i as usize][2] as i64 + prev;
                    dp[i as usize].idx[k] = idx;
                }

                if max < dp[i as usize].max[k] || (max == dp[i as usize].max[k] && sel > dp[i as usize].idx[k]) {
                    return_size = k + 1;
                    max = dp[i as usize].max[k];
                    sel = dp[i as usize].idx[k];
                }
            }
        }
        left_queue.queue_pop();
    }

    let mut result = Vec::with_capacity(return_size);
    for i in 0..return_size {
        result.push(((sel >> (3 - i << 4)) & 0xFFFF) as i32);
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(values);
    }

    let result = maximum_weight(&vec);

    for &val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}