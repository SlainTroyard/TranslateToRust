```rust
use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Priority Queue implementation
#[derive(Eq)]
struct PriorityQueueItem {
    value: i64,
}

impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value) // Min-heap behavior
    }
}

impl PartialOrd for PriorityQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PriorityQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

struct BinaryTree {
    arr: Vec<usize>,
    highest_bit: usize,
}

impl BinaryTree {
    fn new(size: usize) -> Self {
        let highest_bit = if size == 0 {
            1
        } else {
            let mut bit = 1;
            while size >> bit != 0 {
                bit += 1;
            }
            1 << (bit - 1)
        };
        Self {
            arr: vec![0; 2 * size],
            highest_bit,
        }
    }

    fn push_count(&mut self, value: usize) -> usize {
        let mut result = 0;
        let mut bit = self.highest_bit;
        let mut index = 0;

        while bit > 0 {
            if (value & bit) != 0 {
                result += self.arr[Self::left_node(index)];
                index = Self::right_node(index);
            } else {
                index = Self::left_node(index);
            }
            self.arr[index] += 1;
            bit >>= 1;
        }

        result + self.arr[index]
    }

    fn left_node(index: usize) -> usize {
        (index << 1) + 1
    }

    fn right_node(index: usize) -> usize {
        (index << 1) + 2
    }
}

fn binary_search(map: &[usize], value: usize) -> Option<usize> {
    let mut left = 0;
    let mut right = map.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if map[mid] <= value {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left > 0 {
        Some(left - 1)
    } else {
        None
    }
}

fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    let tree_size = 3 * n;
    let mut queue = BinaryHeap::new();
    let mut tree = BinaryTree::new(tree_size);

    let mut y_map = vec![];
    let mut x_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists_buff: Vec<usize> = vec![0; n];
    let mut prefix_buff: Vec<usize> = vec![0; n];
    let mut x_last = vec![-1; n];

    // 1. Discretize all y-coordinates
    for &y in y_coord {
        queue.push(PriorityQueueItem {
            value: y as i64,
        });
    }

    let mut prev_y = i32::MIN;
    while let Some(item) = queue.pop() {
        if prev_y != item.value as i32 {
            y_map.push(item.value as usize);
            prev_y = item.value as i32;
        }
    }

    // 2. Convert coordinates
    for (&x, &y) in x_coord.iter().zip(y_coord.iter()) {
        let y_index = binary_search(&y_map, y as usize).expect("Y not found");
        queue.push(PriorityQueueItem {
            value: ((x as i64) << 32) | (y_index as i64),
        });
    }

    let mut result = -1;
    let mut i = 0;
    while let Some(item) = queue.pop() {
        let cur_x = (item.value >> 32) as usize;
        let cur_y = (item.value & 0xFFFFFFFF) as usize;

        // Process column
        if i == 0 || x_map[i - 1] != cur_x {
            x_map[i] = cur_x;
            lists_buff[i] = cur_y; // Start new column
        }
    }

    result धार 

{}
 greg