use std::io::{self, BufRead};

// Utility functions for binary heap and bit operations
fn father_node(i: usize) -> isize {
    if i == 0 { -1 } else { ((i - 1) >> 1) as isize }
}

fn left_node(i: usize) -> usize {
    ((i << 1) + 1)
}

fn right_node(i: usize) -> usize {
    ((i << 1) + 2)
}

fn higher_int(i: i64) -> i32 {
    (i >> 32) as i32
}

fn lower_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}

fn merge_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64 & 0xFFFFFFFF)
}

// DpNode struct equivalent to C's DpNode
#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

// Priority Queue implementation
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(0); // Placeholder to increase size
        
        let mut father = father_node(son);
        while father != -1 && value < self.arr[father as usize] {
            self.arr[son] = self.arr[father as usize];
            son = father as usize;
            father = father_node(son);
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        if self.arr.is_empty() {
            return;
        }
        
        let last = self.arr.len() - 1;
        let last_value = self.arr[last];
        self.arr.pop();
        
        if self.arr.is_empty() {
            return;
        }
        
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        
        while (left < self.arr.len() && last_value > self.arr[left]) ||
              (right < self.arr.len() && last_value > self.arr[right]) {
            let son = if right < self.arr.len() && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            
            self.arr[father] = self.arr[son];
            father = son;
            left = left_node(father);
            right = right_node(father);
        }
        
        self.arr[father] = last_value;
    }
}

// Helper function to insert an index into the packed representation
fn insert_index(idx: u64, i: i32, count: i32) -> u64 {
    let mut value = [0i32; 4];
    value[0] = (idx >> 48) as i32;
    value[1] = ((idx >> 32) & 0xFFFF) as i32;
    value[2] = ((idx >> 16) & 0xFFFF) as i32;
    value[count as usize] = i;
    
    let mut x = count - 1;
    while x >= 0 && value[x as usize] > i {
        value[(x + 1) as usize] = value[x as usize];
        value[x as usize] = i;
        x -= 1;
    }
    
    ((value[0] as u64) << 48) | 
    ((value[1] as u64) << 32) | 
    ((value[2] as u64) << 16) | 
    (value[3] as u64)
}

// Main algorithm function
fn maximum_weight(intervals: &[Vec<i32>], return_size: &mut i32) -> Vec<i32> {
    let intervals_size = intervals.len();
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };
    
    let mut dp = vec![DpNode {
        max: [0; 4],
        idx: [0; 4],
    }; intervals_size];
    
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();
    
    for i in 0..intervals_size {
        left_queue.push(merge_long(intervals[i][0], i as i32));
        right_queue.push(merge_long(intervals[i][1], i as i32));
    }
    
    let mut max = 0;
    let mut sel = 0xFFFFFFFFFFFFFFFF_u64;
    
    while !left_queue.arr.is_empty() {
        let i = lower_int(left_queue.arr[0]) as usize;
        let edge = higher_int(left_queue.arr[0]);
        
        while !right_queue.arr.is_empty() && edge > higher_int(right_queue.arr[0]) {
            let j = lower_int(right_queue.arr[0]) as usize;
            
            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
            
            right_queue.pop();
        }
        
        dp[i] = last;
        
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    ((i as u64) << 48)
                } else {
                    insert_index(last.idx[k - 1], i as i32, k as i32)
                };
                
                if dp[i].max[k] < intervals[i][2] as i64 + prev || 
                   (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx) {
                    dp[i].max[k] = intervals[i][2] as i64 + prev;
                    dp[i].idx[k] = idx;
                }
                
                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    *return_size = (k + 1) as i32;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
        
        left_queue.pop();
    }
    
    let mut result = Vec::with_capacity(*return_size as usize);
    for i in 0..*return_size {
        let shift = ((3 - i) << 4) as u64;
        result.push(((sel >> shift) & 0xFFFF) as i32);
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read number of intervals
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Expected a number");
    
    // Read intervals
    let mut intervals: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Expected a number"))
            .collect();
        intervals.push(values);
    }
    
    // Process and output result
    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);
    
    for i in 0..return_size as usize {
        print!("{} ", result[i]);
    }
    println!();
    
    Ok(())
}