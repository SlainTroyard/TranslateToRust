use std::io::{self, BufRead};
use std::cmp::max;

// Binary tree. Used to calculate the number of values in a given interval
// and to add values into it.
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

// Priority queue. Used for discretization and listing coordinate points
// from left to right and bottom to top.
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    // Push into the priority queue
    fn push(&mut self, value: i64) {
        self.arr.push(value);
        let mut son = self.arr.len() - 1;
        
        while son > 0 {
            let father = (son - 1) >> 1;
            if self.arr[father] <= value {
                break;
            }
            self.arr[son] = self.arr[father];
            son = father;
        }
        
        self.arr[son] = value;
    }

    // Pop from the priority queue
    fn pop(&mut self) {
        if self.arr.is_empty() {
            return;
        }
        
        let last = self.arr.pop().unwrap();
        if self.arr.is_empty() {
            return;
        }
        
        let mut father = 0;
        loop {
            let left = (father << 1) + 1;
            let right = (father << 1) + 2;
            
            if (left < self.arr.len() && last > self.arr[left]) ||
               (right < self.arr.len() && last > self.arr[right]) {
                let son = if right < self.arr.len() && self.arr[left] > self.arr[right] {
                    right
                } else {
                    left
                };
                self.arr[father] = self.arr[son];
                father = son;
            } else {
                break;
            }
        }
        
        self.arr[father] = last;
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    fn peek(&self) -> Option<i64> {
        self.arr.first().copied()
    }

    fn size(&self) -> usize {
        self.arr.len()
    }
}

impl BinaryTree {
    fn new(max: i32) -> Self {
        let mut tree = BinaryTree {
            arr: vec![0; max as usize * 3],
            highest_bit: 0,
        };
        tree.set_highest_bit(max);
        tree
    }

    // Calculate the highest bit needed for storing values in the binary tree.
    // If the input is 0, record as the first bit from the right.
    fn set_highest_bit(&mut self, max: i32) {
        let mut i = 1;
        if max != 0 {
            let mut max_copy = max;
            while max_copy != 0 {
                i += 1;
                max_copy >>= 1;
            }
            i = 1 << (i - 2);
        }
        self.highest_bit = i;
    }

    // Add a value to the binary tree and return the count of values less than or equal to it.
    fn push_count(&mut self, value: i32) -> i32 {
        let mut i = 0;
        let mut bit = self.highest_bit;
        let mut result = 0;
        
        // Add the value to the tree while recording the count of values less than it.
        while bit != 0 {
            if (bit & value) != 0 {
                result += self.arr[((i << 1) + 1) as usize];
                i = (i << 1) + 2;
            } else {
                i = (i << 1) + 1;
            }
            self.arr[i as usize] += 1;
            bit >>= 1;
        }
        
        // Add the count of the value itself to get the total count of values less than or equal to it.
        result += self.arr[i as usize];
        result
    }
}

// Helper functions
fn high_int(i: i64) -> i32 {
    (i >> 32) as i32
}

fn low_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}

fn mer_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64)
}

// In a sorted array (without duplicates), find the largest index less than or equal to value.
// Return -1 if it doesn't exist.
fn binary_search(map: &[i32], value: i32) -> i32 {
    if map.is_empty() || value < map[0] {
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
    
    left as i32
}

// Main function
fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    let mut result: i64 = -1;
    
    // Initialize data structures
    let mut queue = PriorityQueue::new();
    let mut tree = BinaryTree::new(n as i32 - 1);
    
    let mut x_map = vec![0; n];
    let mut y_map = Vec::new();
    let mut lists_size = vec![0; n];
    let mut lists_buff = vec![0; n];
    let mut prefix_buff = vec![0; n];
    let mut x_last = vec![-1; n];
    
    let mut lists = vec![Vec::new(); n];
    let mut prefix = vec![Vec::new(); n];
    
    // Discretize all vertical coordinates
    for &y in y_coord {
        queue.push(y as i64);
    }
    
    let mut k = i32::MIN;
    while !queue.is_empty() {
        let val = queue.peek().unwrap() as i32;
        if k < val {
            k = val;
            y_map.push(k);
        }
        queue.pop();
    }
    
    // Enqueue all coordinate points, recording the mapped discretized vertical coordinates
    for j in 0..n {
        let y = binary_search(&y_map, y_coord[j]);
        queue.push(mer_long(x_coord[j], y));
    }
    
    // Dequeue column by column
    let mut i = 0;
    let mut buff_size = 0;
    
    while !queue.is_empty() {
        let mut j = 0;
        lists[i] = Vec::new();
        prefix[i] = Vec::new();
        x_map[i] = high_int(queue.peek().unwrap());
        
        // Points with the same horizontal coordinate are treated as column x_map[i]
        while !queue.is_empty() && x_map[i] == high_int(queue.peek().unwrap()) {
            let point_y = low_int(queue.peek().unwrap());
            lists[i].push(point_y);
            prefix[i].push(tree.push_count(point_y));
            
            // If it can serve as the top-right corner of a rectangle
            if j > 0 && x_last[point_y as usize] != -1 && x_last[point_y as usize] == k {
                let k_idx = x_last[point_y as usize] as usize;
                let x = binary_search(&lists[k_idx], point_y);
                let y = binary_search(&lists[k_idx], lists[i][j-1]);
                
                let number = prefix[i][j] - prefix[i][j-1] - prefix[k_idx][x as usize] + prefix[k_idx][y as usize];
                
                // If x and y are adjacent, and the interval only contains one vertex
                if x - 1 == y && number == 1 {
                    let t = (x_map[i] - x_map[k_idx]) as i64 * (y_map[point_y as usize] - y_map[lists[i][j-1] as usize]) as i64;
                    result = max(result, t);
                }
            }
            
            // Update x_last of the current point's vertical coordinate
            k = x_last[point_y as usize];
            x_last[point_y as usize] = i as i32;
            
            queue.pop();
            j += 1;
        }
        
        lists_size[i] = j;
        buff_size += j;
        i += 1;
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut x_coord = vec![0; n];
    let mut y_coord = vec![0; n];
    
    // Input the coordinates of the points
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        x_coord[i] = coords[0];
        y_coord[i] = coords[1];
    }
    
    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, &y_coord);
    
    // Output the result
    println!("{}", max_area);
}