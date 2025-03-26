use std::io::{self, BufRead, Write};

const fn father_node(i: usize) -> isize {
    if i == 0 { -1 } else { ((i - 1) >> 1) as isize }
}
const fn left_node(i: usize) -> usize {
    (i << 1) + 1
}
const fn right_node(i: usize) -> usize {
    (i << 1) + 2
}
const fn higher_int(i: i64) -> i32 {
    (i >> 32) as i32
}
const fn lower_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}
const fn merge_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64)
}

struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

impl Default for DpNode {
    fn default() -> Self {
        Self {
            max: [0; 4],
            idx: [0; 4],
        }
    }
}

impl Clone for DpNode {
    fn clone(&self) -> Self {
        Self {
            max: self.max,
            idx: self.idx,
        }
    }
}

struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new(capacity: usize) -> Self {
        Self {
            arr: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        let mut father = father_node(son);
        self.arr.push(0); // Placeholder
        
        while father != -1 && value < self.arr[father as usize] {
            self.arr[son] = self.arr[father as usize];
            son = father as usize;
            father = father_node(son);
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        let last_idx = self.arr.len() - 1;
        let last_val = self.arr[last_idx];
        self.arr.pop();
        
        if self.arr.is_empty() {
            return;
        }
        
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        
        while (left < self.arr.len() && last_val > self.arr[left]) ||
              (right < self.arr.len() && last_val > self.arr[right]) {
            
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
        
        self.arr[father] = last_val;
    }
    
    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
    
    fn size(&self) -> usize {
        self.arr.len()
    }
    
    fn peek(&self) -> Option<i64> {
        if self.arr.is_empty() {
            None
        } else {
            Some(self.arr[0])
        }
    }
}

fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut value = [0i32; 4];
    
    value[0] = (idx >> 48) as i32;
    value[1] = ((idx >> 32) & 0xFFFF) as i32;
    value[2] = ((idx >> 16) & 0xFFFF) as i32;
    value[count] = i;
    
    for x in (0..count).rev() {
        if value[x] > i {
            value[x + 1] = value[x];
            value[x] = i;
        }
    }
    
    ((value[0] as u64) << 48) | 
    ((value[1] as u64) << 32) | 
    ((value[2] as u64) << 16) | 
    (value[3] as u64)
}

fn maximum_weight(intervals: &[Vec<i32>]) -> Vec<i32> {
    let intervals_size = intervals.len();
    let mut return_size = 0;
    
    let mut last = DpNode::default();
    let mut dp = vec![DpNode::default(); intervals_size];
    
    let mut left_queue = PriorityQueue::new(intervals_size);
    let mut right_queue = PriorityQueue::new(intervals_size);
    
    // Initialize queues
    for (i, interval) in intervals.iter().enumerate() {
        left_queue.push(merge_long(interval[0], i as i32));
        right_queue.push(merge_long(interval[1], i as i32));
    }
    
    let mut max = 0i64;
    let mut sel = u64::MAX;
    
    while !left_queue.is_empty() {
        let top_val = left_queue.peek().unwrap();
        let i = lower_int(top_val) as usize;
        let edge = higher_int(top_val);
        
        while !right_queue.is_empty() && edge > higher_int(right_queue.peek().unwrap()) {
            let right_top = right_queue.peek().unwrap();
            let j = lower_int(right_top) as usize;
            
            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || 
                   (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
            
            right_queue.pop();
        }
        
        dp[i] = last.clone();
        
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i as i32, k)
                };
                
                let new_val = intervals[i][2] as i64 + prev;
                
                if dp[i].max[k] < new_val || 
                   (dp[i].max[k] == new_val && dp[i].idx[k] > idx) {
                    dp[i].max[k] = new_val;
                    dp[i].idx[k] = idx;
                }
                
                if max < dp[i].max[k] || 
                   (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    return_size = k + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
        
        left_queue.pop();
    }
    
    let mut result = Vec::with_capacity(return_size);
    for i in 0..return_size {
        let shift = (3 - i) << 4;
        result.push(((sel >> shift) & 0xFFFF) as i32);
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let stdout = io::stdout();
    let mut handle_out = stdout.lock();
    
    let mut input = String::new();
    handle.read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Failed to parse number of intervals");
    
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        handle.read_line(&mut input)?;
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse interval value"))
            .collect();
        
        intervals.push(values);
    }
    
    let result = maximum_weight(&intervals);
    
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            write!(handle_out, " ")?;
        }
        write!(handle_out, "{}", val)?;
    }
    writeln!(handle_out)?;
    
    Ok(())
}