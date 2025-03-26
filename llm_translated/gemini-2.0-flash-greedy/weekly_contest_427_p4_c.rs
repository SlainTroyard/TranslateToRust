use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::io;
use std::io::Read;

// Macros for easier bit manipulation
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

macro_rules! high_int {
    ($i:expr) => {
        (($i) >> 32) as i32
    };
}

macro_rules! low_int {
    ($i:expr) => {
        (($i) & 0xFFFFFFFF) as i32
    };
}

macro_rules! mer_long {
    ($i:expr, $j:expr) => {
        (($i as i64) << 32) | ($j as i64)
    };
}

macro_rules! max_val {
    ($i:expr, $j:expr) => {
        if $i > $j {
            $i
        } else {
            $j
        }
    };
}

// Priority queue implementation (min-heap)
#[derive(Debug)]
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len() as i32;
        self.arr.push(value);
        while father_node!(son) != -1 && value < self.arr[father_node!(son) as usize] {
            let father = father_node!(son) as usize;
            self.arr.swap(son as usize, father);
            son = father as i32;
        }
    }

    fn pop(&mut self) -> Option<i64> {
        if self.arr.is_empty() {
            return None;
        }

        if self.arr.len() == 1 {
            return self.arr.pop();
        }

        let last = self.arr.pop().unwrap();
        let root = self.arr[0];
        self.arr[0] = last;

        let mut father = 0;
        let mut left = left_node!(father);
        let mut right = right_node!(father);
        let mut son;

        while (left < self.arr.len() && self.arr[self.arr.len() - 1] > self.arr[left])
            || (right < self.arr.len() && self.arr[self.arr.len() - 1] > self.arr[right])
        {
            if right < self.arr.len() && self.arr[left] > self.arr[right] {
                son = right;
            } else {
                son = left;
            }

            self.arr.swap(father, son);
            father = son;
            left = left_node!(father);
            right = right_node!(father);

            if left >= self.arr.len() {
                break;
            }
        }
        self.arr[father] = self.arr[self.arr.len()];

        Some(root)
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

// Binary tree for range queries (prefix sum)
#[derive(Debug)]
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

impl BinaryTree {
    fn new(size: usize) -> Self {
        BinaryTree {
            arr: vec![0; size],
            highest_bit: 0,
        }
    }

    fn tree_highest_bit(&mut self, max_val: i32) {
        let mut i = 1;
        let mut max_copy = max_val;
        if max_copy != 0 {
            while max_copy != 0 {
                i += 1;
                max_copy >>= 1;
            }
            i = 1 << (i - 2);
        }
        self.highest_bit = i;
    }

    fn tree_push_count(&mut self, value: i32) -> i32 {
        let mut i = 0;
        let mut bit = self.highest_bit;
        let mut result = 0;

        while bit != 0 {
            if bit & value != 0 {
                result += self.arr[left_node!(i) as usize];
                i = right_node!(i);
            } else {
                i = left_node!(i);
            }
            self.arr[i as usize] += 1;
            bit >>= 1;
        }

        result += self.arr[i as usize];
        result
    }
}

// Binary search to find the largest index <= value
fn binary_search(map: &[i32], value: i32) -> i32 {
    let mut left = 0;
    let mut right = (map.len() as i32) - 1;
    let mut mid = -1;

    if value < map[left as usize] {
        return mid;
    }

    while left < right {
        mid = left + right + 1 >> 1;
        if value < map[mid as usize] {
            right = mid - 1;
        } else {
            left = mid;
        }
    }

    left
}

fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    let tree_size = n * 3;
    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists_buff = vec![0; n * n]; // Increased buffer size
    let mut prefix_buff = vec![0; n * n]; // Increased buffer size
    let mut x_last = vec![-1; n];

    let mut lists = vec![Vec::<i32>::new(); n];
    let mut prefix = vec![Vec::<i32>::new(); n];

    let mut tree = BinaryTree::new(tree_size);
    let mut queue = PriorityQueue::new();

    tree.tree_highest_bit((n - 1) as i32);

    // Discretize y coordinates
    for &y in y_coord {
        queue.push(y as i64);
    }

    let mut y_map_size = 0;
    let mut k = i32::MIN;
    while !queue.is_empty() {
        if let Some(val) = queue.pop() {
            if k < val as i32 {
                k = val as i32;
                y_map[y_map_size] = k;
                y_map_size += 1;
            }
        }
    }

    // Enqueue coordinate pairs (x, discretized y)
    for i in 0..n {
        let y = binary_search(&y_map[..y_map_size], y_coord[i]);
        queue.push(mer_long!(x_coord[i], y) as i64);
    }

    let mut i = 0;
    let mut buff_size = 0;
    let mut result = -1;
    while !queue.is_empty() {
        let mut j = 0;
        lists[i] = lists_buff[buff_size..].to_vec();
        prefix[i] = prefix_buff[buff_size..].to_vec();
        x_map[i] = high_int!(queue.arr[0]);

        let k_val = x_map[i];

        while !queue.is_empty() && k_val == high_int!(queue.arr[0]) {
            let val = queue.pop().unwrap();
            lists[i].push(low_int!(val));
            prefix[i].push(tree.tree_push_count(low_int!(val)));

            if j > 0 && x_last[lists[i][j as usize] as usize] != -1 && x_last[lists[i][j as usize] as usize] == i as i32 - 1 {

                let k = x_last[lists[i][j as usize] as usize] as usize;

                let x = binary_search(&lists[k], lists[i][j as usize]);
                let y = binary_search(&lists[k], lists[i][j as usize -1]);

                let number = prefix[i][j as usize] - prefix[i][j as usize -1] - prefix[k][x as usize] + prefix[k][y as usize];

                if x - 1 == y && 1 == number {
                    let t = (x_map[i] as i64 - x_map[k] as i64) * (y_map[lists[i][j as usize] as usize] as i64 - y_map[lists[i][j as usize -1] as usize] as i64);
                    result = max_val!(result, t);
                }
            }

            let k = x_last[lists[i][j as usize] as usize];
            x_last[lists[i][j as usize] as usize] = i as i32;
            j += 1;
        }
        lists_size[i] = j as usize;
        buff_size += j as usize;
        i += 1;
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Input the number of points
    let n: usize = lines.next().unwrap().trim().parse().unwrap();

    let mut x_coord = vec![0; n];
    let mut y_coord = vec![0; n];

    // Input the coordinates of the points
    for i in 0..n {
        let line = lines.next().unwrap();
        let mut coords = line.split_whitespace();

        x_coord[i] = coords.next().unwrap().trim().parse().unwrap();
        y_coord[i] = coords.next().unwrap().trim().parse().unwrap();
    }

    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, &y_coord);

    // Output the result
    println!("{}", max_area);

    Ok(())
}