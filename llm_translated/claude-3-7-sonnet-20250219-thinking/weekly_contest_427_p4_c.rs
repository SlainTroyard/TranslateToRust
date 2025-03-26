use std::io::{self, BufRead};
use std::cmp::max;

// Helper macros equivalent to C macros
#[inline]
fn father_node(i: usize) -> isize {
    if i == 0 { -1 } else { ((i - 1) >> 1) as isize }
}

#[inline]
fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

#[inline]
fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

#[inline]
fn high_int(i: i64) -> i32 {
    (i >> 32) as i32
}

#[inline]
fn low_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}

#[inline]
fn mer_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64 & 0xFFFFFFFF)
}

// Priority queue implementation
struct PriorityQueue {
    arr: Vec<i64>,
    arr_size: usize,
}

impl PriorityQueue {
    fn new(capacity: usize) -> Self {
        PriorityQueue {
            arr: vec![0; capacity],
            arr_size: 0,
        }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr_size;
        self.arr_size += 1;
        
        let mut father = father_node(son);
        while father != -1 && value < self.arr[father as usize] {
            self.arr[son] = self.arr[father as usize];
            son = father as usize;
            father = father_node(son);
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        if self.arr_size == 0 {
            return;
        }
        
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        
        self.arr_size -= 1;
        
        while (left < self.arr_size && self.arr[self.arr_size] > self.arr[left]) ||
              (right < self.arr_size && self.arr[self.arr_size] > self.arr[right]) {
            let son = if right < self.arr_size && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            
            self.arr[father] = self.arr[son];
            father = son;
            left = left_node(father);
            right = right_node(father);
        }
        
        self.arr[father] = self.arr[self.arr_size];
    }
}

// Binary tree implementation
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

impl BinaryTree {
    fn new(arr: Vec<i32>) -> Self {
        BinaryTree {
            arr,
            highest_bit: 0,
        }
    }

    fn calculate_highest_bit(&mut self, max: i32) {
        let mut i = 1;
        if max != 0 {
            let mut tmp_max = max;
            while tmp_max != 0 {
                i += 1;
                tmp_max >>= 1;
            }
            i = 1 << (i - 2);
        }
        self.highest_bit = i;
    }

    fn push_count(&mut self, value: i32) -> i32 {
        let mut i = 0;
        let mut bit = self.highest_bit;
        let mut result = 0;
        
        while bit != 0 {
            if (bit & value) != 0 {
                result += self.arr[left_node(i)];
                i = right_node(i);
            } else {
                i = left_node(i);
            }
            self.arr[i] += 1;
            bit >>= 1;
        }
        
        result += self.arr[i];
        result
    }
}

// Binary search implementation
fn binary_search(map: &[i32], map_size: usize, value: i32) -> i32 {
    if map_size == 0 || value < map[0] {
        return -1;
    }
    
    let mut left = 0;
    let mut right = map_size - 1;
    
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

// Main algorithm function
fn max_rectangle_area(x_coord: &[i32], x_coord_size: usize, y_coord: &[i32], y_coord_size: usize) -> i64 {
    let n = x_coord_size;
    let tree_size = n * 3;
    
    let mut i = 0;
    let mut j;
    let mut k = i32::MIN;
    let mut x;
    let mut y;
    let mut number;
    let mut y_map_size = 0;
    let mut buff_size = 0;
    
    // Allocate arrays similar to the C version
    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists_buff = vec![0; n];
    let mut prefix_buff = vec![0; n];
    let mut x_last = vec![-1; n];
    let mut arr1 = vec![0; tree_size];
    let mut arr2 = vec![0i64; n];
    
    // Lists and prefix are references into the buffers
    let mut lists = vec![0; n];
    let mut prefix = vec![0; n];
    
    let mut t;
    let mut result = -1;
    
    // Initialize queue and tree
    let mut queue = PriorityQueue {
        arr: arr2,
        arr_size: 0,
    };
    
    let mut tree = BinaryTree {
        arr: arr1,
        highest_bit: 0,
    };
    
    tree.calculate_highest_bit((n - 1) as i32);
    
    // Discretize y coordinates
    for j in 0..n {
        queue.push(y_coord[j] as i64);
    }
    
    while queue.arr_size > 0 {
        if k < queue.arr[0] as i32 {
            k = queue.arr[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue.pop();
    }
    
    // Enqueue all points with their mapped coordinates
    for j in 0..n {
        y = binary_search(&y_map, y_map_size, y_coord[j]);
        queue.push(mer_long(x_coord[j], y));
    }
    
    // Process points column by column
    while queue.arr_size > 0 {
        j = 0;
        lists[i] = buff_size;
        prefix[i] = buff_size;
        x_map[i] = high_int(queue.arr[0]);
        
        // Process all points in the current column
        while queue.arr_size > 0 && x_map[i] == high_int(queue.arr[0]) {
            lists_buff[lists[i] + j] = low_int(queue.arr[0]);
            prefix_buff[prefix[i] + j] = tree.push_count(lists_buff[lists[i] + j]);
            
            // Check if it can be the top-right corner of a rectangle
            if j > 0 && x_last[lists_buff[lists[i] + j] as usize] != -1 && 
               x_last[lists_buff[lists[i] + j] as usize] == k {
                let k_idx = x_last[lists_buff[lists[i] + j] as usize] as usize;
                x = binary_search(&lists_buff[lists[k_idx]..], lists_size[k_idx], lists_buff[lists[i] + j]);
                y = binary_search(&lists_buff[lists[k_idx]..], lists_size[k_idx], lists_buff[lists[i] + j - 1]);
                
                number = prefix_buff[prefix[i] + j] - prefix_buff[prefix[i] + j - 1] - 
                        prefix_buff[prefix[k_idx] + x as usize] + prefix_buff[prefix[k_idx] + y as usize];
                
                if x - 1 == y && number == 1 {
                    t = (x_map[i] - x_map[k_idx]) as i64 * 
                        (y_map[lists_buff[lists[i] + j] as usize] - y_map[lists_buff[lists[i] + j - 1] as usize]) as i64;
                    result = max(result, t);
                }
            }
            
            // Update x_last
            k = x_last[lists_buff[lists[i] + j] as usize];
            x_last[lists_buff[lists[i] + j] as usize] = i as i32;
            
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
    
    // Read number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut x_coord = vec![0; n];
    let mut y_coord = vec![0; n];
    
    // Read coordinates
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        x_coord[i] = coords[0];
        y_coord[i] = coords[1];
    }
    
    // Calculate maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, n, &y_coord, n);
    
    // Output the result
    println!("{}", max_area);
}