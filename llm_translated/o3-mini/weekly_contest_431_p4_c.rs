use std::io::{self, BufRead, Write};

// Helper macros converted to functions:
//
// FATHER_NODE(i) -> if i == 0 { None } else { Some((i - 1) >> 1) }
fn father_node(i: usize) -> Option<usize> {
    if i == 0 {
        None
    } else {
        Some((i - 1) >> 1)
    }
}

// LEFT_NODE(i) = (i << 1) + 1
#[inline]
fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

// RIGHT_NODE(i) = (i << 1) + 2
#[inline]
fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

// HIGHER_INT(i): for a 64-bit integer, shift right 32 bits and return lower 32 bits as i32
#[inline]
fn higher_int(x: i64) -> i32 {
    (x >> 32) as i32
}

// LOWER_INT(i): bitwise AND with 0xFFFFFFFF to get lower 32 bits as i32
#[inline]
fn lower_int(x: i64) -> i32 {
    (x & 0xFFFFFFFF) as i32
}

// MERGE_LONG(i, j): Combine two 32-bit integers into a single 64-bit integer
#[inline]
fn merge_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | ((j as u32) as i64)
}

/// DpNode struct holds two arrays: max holds scores; idx holds the chosen indices encoded as a u64.
/// Using Default to initialize arrays to zero.
#[derive(Clone, Copy, Debug)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

impl Default for DpNode {
    fn default() -> Self {
        DpNode {
            max: [0; 4],
            idx: [0; 4],
        }
    }
}

/// PriorityQueue struct wraps a Vec<i64> and maintains a min-heap property.
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }
    
    // Push a new value into the priority queue
    fn push(&mut self, value: i64) {
        // Insert at the end and bubble up the new value
        let mut son = self.arr.len();
        self.arr.push(value); // temporarily push; will fix position below
        while let Some(father) = father_node(son) {
            // If new value is smaller than parent's value, swap upward.
            if value < self.arr[father] {
                self.arr[son] = self.arr[father];
                son = father;
            } else {
                break;
            }
        }
        self.arr[son] = value;
    }
    
    // Pop the smallest value (root of the heap)
    fn pop(&mut self) {
        if self.arr.is_empty() {
            return;
        }
        let last_index = self.arr.len() - 1;
        // Remove one element from size; we'll reposition the last element.
        let temp = self.arr[last_index];
        self.arr.pop();
        if self.arr.is_empty() {
            return;
        }
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        // Bubble down the last element
        while (left < self.arr.len() && temp > self.arr[left])
            || (right < self.arr.len() && temp > self.arr[right])
        {
            // choose the smaller child
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
        self.arr[father] = temp;
    }
    
    // Peek at the smallest element in the queue
    fn peek(&self) -> Option<i64> {
        self.arr.first().copied()
    }
    
    // Check if the queue is empty
    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

/// insert_index: Inserts a new index into the existing u64 encoded indices.
/// The u64 is interpreted as four 16-bit values:
///   value[0] = bits 48..63, value[1] = bits 32..47, value[2] = bits 16..31, value[3] = bits 0..15.
/// The new index i is to be inserted at position count, and then bubble it up if needed.
fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut value = [0i32; 4];
    // Extract the first three 16-bit segments.
    value[0] = (idx >> 48) as i32;
    value[1] = ((idx >> 32) & 0xFFFF) as i32;
    value[2] = ((idx >> 16) & 0xFFFF) as i32;
    // Set the new index at position 'count'.
    value[count] = i;
    // Bubble the new index up if necessary.
    let mut x = count as i32 - 1;
    while x >= 0 && value[x as usize] > i {
        value[(x + 1) as usize] = value[x as usize];
        value[x as usize] = i;
        x -= 1;
    }
    ((value[0] as u64) << 48)
        | ((value[1] as u64) << 32)
        | ((value[2] as u64) << 16)
        | (value[3] as u64)
}

/// maximum_weight: The main dynamic programming function that implements the algorithm.
/// It takes the intervals (each interval is an array of 3 integers) and returns a vector of indices.
/// The return size is also updated via a mutable reference.
fn maximum_weight(
    intervals: &Vec<[i32; 3]>,
    return_size: &mut i32,
) -> Vec<i32> {
    let n = intervals.len();
    // Initialize leftQueue and rightQueue with capacity n.
    let mut left_queue = PriorityQueue {
        arr: Vec::with_capacity(n),
    };
    let mut right_queue = PriorityQueue {
        arr: Vec::with_capacity(n),
    };
    
    // dp[i] will store the best state ending at interval i.
    let mut dp = vec![DpNode::default(); n];
    // last will hold the best state among finished intervals.
    let mut last = DpNode::default();
    
    // Variables to record overall maximum weight and selection code.
    let mut max_weight: i64 = 0;
    let mut sel: u64 = 0xFFFFFFFFFFFFFFFF; // initialize to max possible u64
    
    // Push each interval into both queues.
    for i in 0..n {
        // The left queue is keyed by the start time and index.
        left_queue.push(merge_long(intervals[i][0], i as i32));
        // The right queue is keyed by the end time and index.
        right_queue.push(merge_long(intervals[i][1], i as i32));
    }
    
    // Process intervals in order of increasing left endpoint.
    while !left_queue.is_empty() {
        // Get the smallest element from left_queue
        let cur_val = left_queue.peek().unwrap();
        let i = lower_int(cur_val) as usize;
        let edge = higher_int(cur_val);
        
        // While there are intervals in right_queue with end time less than current left edge
        while !right_queue.is_empty() && edge > higher_int(right_queue.peek().unwrap()) {
            let j_val = right_queue.peek().unwrap();
            let j = lower_int(j_val) as usize;
            // Update the 'last' DP state from dp[j]
            for k in 0..3 {
                if last.max[k] < dp[j].max[k]
                    || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k])
                {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
            right_queue.pop();
        }
        
        // Start new DP state for interval i based on 'last'
        dp[i] = last;
        for k in 0..4 {
            // For k == 0, previous weight is 0, otherwise previous state is last.max[k - 1]
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                // Calculate new selection index
                let idx_val: u64 = if k == 0 {
                    // For the first selection, just encode i in the highest 16 bits.
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i as i32, k)
                };
                let new_val = intervals[i][2] as i64 + prev;
                if dp[i].max[k] < new_val
                    || (dp[i].max[k] == new_val && dp[i].idx[k] > idx_val)
                {
                    dp[i].max[k] = new_val;
                    dp[i].idx[k] = idx_val;
                }
                if max_weight < dp[i].max[k]
                    || (max_weight == dp[i].max[k] && sel > dp[i].idx[k])
                {
                    *return_size = (k + 1) as i32;
                    max_weight = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
        left_queue.pop();
    }
    
    // Build the result vector by extracting each 16-bit field from the selection code.
    let rs = *return_size as usize;
    let mut result = Vec::with_capacity(rs);
    // For each interval index, extract bits from highest bits to lower.
    for i in 0..rs {
        // (3-i) << 4 is equivalent to (3-i)*16.
        let shift = (3 - i) * 16;
        result.push(((sel >> shift) & 0xFFFF) as i32);
    }
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    // Read the whole input as string.
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();
    
    // Read number of intervals (n)
    let n: usize = match tokens.next() {
        Some(token) => token.parse().expect("Failed to parse n"),
        None => return Ok(()),
    };
    
    // Read intervals: each interval has 3 integers.
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        // Read 3 integers per interval.
        let a: i32 = tokens
            .next()
            .expect("Missing interval value")
            .parse()
            .expect("Failed to parse value");
        let b: i32 = tokens
            .next()
            .expect("Missing interval value")
            .parse()
            .expect("Failed to parse value");
        let c: i32 = tokens
            .next()
            .expect("Missing interval value")
            .parse()
            .expect("Failed to parse value");
        intervals.push([a, b, c]);
    }
    
    let mut return_size: i32 = 0;
    let result = maximum_weight(&intervals, &mut return_size);
    
    // Write the result to stdout, space-separated on one line ending with a newline.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 0..(return_size as usize) {
        write!(handle, "{} ", result[i])?;
    }
    writeln!(handle)?;
    Ok(())
}