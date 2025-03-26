```rust
use std::cmp::max;
use std::io::{self, BufRead};

/// Helper function to calculate the parent node index in a binary heap
fn father_node(i: usize) -> isize {
    if i == 0 {
        -1
    } else {
        ((i - 1) >> 1) as isize
    }
}

/// Helper function to calculate the left child node index in a binary heap
fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

/// Helper function to calculate the right child node index in a binary heap
fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

/// Helper function to extract the high 32 bits of a 64-bit number
fn high_int(i: i64) -> i32 {
    (i >> 32) as i32
}

/// Helper function to extract the low 32 bits of a 64-bit number
fn low_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}

/// Helper function to merge two 32-bit integers into a 64-bit integer
fn mer_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64)
}

/// PriorityQueue structure for managing coordinate points
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new(size: usize) -> Self {
        PriorityQueue {
            arr: Vec::with_capacity(size),
        }
    }

    /// Push a value into the priority queue
    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(value);
        while let Some(father) = father_node(son).try_into().ok().filter(|&f| value < self.arr[f]) {
            self.arr[son] = self.arr[father];
            son = father;
        }
        self.arr[son] = value;
    }

    /// Pop the smallest value from the priority queue
    fn pop(&mut self) -> Option<i64> {
        if self.arr.is_empty() {
            return None;
        }
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        let value = self.arr.swap_remove(0);
        if self.arr.is_empty() {
            return Some(value);
        }
        while (left < self.arr.len() && self.arr[self.arr.len() - 1] > self.arr[left])
            || (right < self.arr.len() && self.arr[self.arr.len() - 1] > self.arr[right])
        {
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
        self.arr[father] = self.arr[self.arr.len() - 1];
        Some(value)
    }

    fn peek(&self) -> Option<&i64> {
        self.arr.first()
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

/// BinaryTree structure for interval queries and updates
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: usize,
}

impl BinaryTree {
    fn new(size: usize) -> Self {
        BinaryTree {
            arr: vec![0; size],
            highest_bit: 0,
        }
    }

    fn set_highest_bit(&mut self, max: usize) {
        let mut highest_bit = 1;
        let mut max = max;
        while max != 0 {
            highest_bit <<= 1;
            max >>= 1;
        }
        self.highest_bit = highest_bit >> 1;
    }

    /// Update the tree and return the count of elements less than or equal to the given value
    fn push_count(&mut self, value: usize) -> i32 {
        let mut result = 0;
        let mut idx = 0;
        let mut bit = self.highest_bit;
        while bit != 0 {
            if (bit & value) != 0 {
                result += self.arr[left_node(idx)];
                idx = right_node(idx);
            } else {
                idx = left_node(idx);
            }
            self.arr[idx] += 1;
            bit >>= 1;
        }
        result + self.arr[idx]
    }
}

/// Binary search on a sorted array of integers to find the largest index less than or equal to `value`
fn binary_search(map: &[i32], value: i32) -> isize {
    if value < map[0] {
        return -1;
    }
    let mut left = 0;
    let mut right = map.len() - 1;
    while left < right {
        let mid = (left + right + 1) >> 1;
        if value < map[mid] {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left as isize
}

/// Main function to calculate the maximum rectangle area
fn max_rectangle_area(x_coords: &[i32], y_coords: &[i32]) -> i64 {
    let n = x_coords.len();
    let mut tree = BinaryTree::new(3 * n);
    tree.set_highest_bit(n);

    let mut queue = PriorityQueue::new(n);

    let mut y_map = Vec::with_capacity(n);
    for &y in y_coords {
        queue.push(y as i64);
    }
    while let Some(val) = queue.pop() {
        if y_map.last().map_or(true, |&last| last < val as i32) {
            y_map.push(val as i32);
        }
    }

modify(std...)