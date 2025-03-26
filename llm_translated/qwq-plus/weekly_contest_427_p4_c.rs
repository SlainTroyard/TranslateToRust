use std::io;

// Define macros as functions
fn father_node(i: usize) -> isize {
    if i == 0 { -1 } else { (i - 1) as isize >> 1 }
}

fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

fn high_int(value: i64) -> i32 {
    (value >> 32) as i32
}

fn low_int(value: i64) -> i32 {
    (value & 0xFFFFFFFF) as i32
}

fn mer_long(x: i32, y: i32) -> i64 {
    ((x as i64) << 32) | (y as i64)
}

fn max_val(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// Binary search function
fn binary_search(map: &[i32], value: i32) -> i32 {
    let (mut left, mut right) = (0, map.len() as i32 - 1);
    if value < map[0] {
        return -1;
    }
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if map[mid as usize] > value {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left
}

// Binary Tree struct
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

impl BinaryTree {
    fn new(size: usize) -> Self {
        BinaryTree { arr: vec![0; size], highest_bit: 0 }
    }

    fn highest_bit(&mut self, max: i32) {
        let mut i = 1;
        if max != 0 {
            let mut temp = max;
            while temp != 0 {
                i += 1;
                temp >>= 1;
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

// Priority Queue struct
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(value);
        let mut father = father_node(son);
        while father != -1 && self.arr[son] < self.arr[father as usize] {
            self.arr.swap(son, father as usize);
            son = father as usize;
            father = father_node(son);
        }
    }

    fn pop(&mut self) -> Option<i64> {
        if self.arr.is_empty() {
            return None;
        }
        let last = self.arr.pop()?;
        if !self.arr.is_empty() {
            self.arr[0] = last;
            let mut father = 0;
            loop {
                let left = left_node(father);
                let right = right_node(father);
                let mut son = -1;
                if right < self.arr.len() {
                    if self.arr[right] < self.arr[left] {
                        son = right as isize;
                    } else {
                        son = left as isize;
                    }
                } else if left < self.arr.len() {
                    son = left as isize;
                } else {
                    break;
                }
                if self.arr[son as usize] < self.arr[father] {
                    self.arr.swap(father, son as usize);
                    father = son as usize;
                } else {
                    break;
                }
            }
        }
        Some(last)
    }
}

fn max_rectangle_area(x_coords: &[i32], y_coords: &[i32]) -> i64 {
    let n = x_coords.len();
    if n == 0 {
        return -1;
    }

    let tree_size = n * 3;
    let mut x_last = vec![-1; n];
    let mut tree = BinaryTree::new(tree_size);
    let mut queue = PriorityQueue::new();

    // Discretize y coordinates
    let mut y_values: Vec<i32> = y_coords.iter().cloned().collect();
    queue.arr.clear();
    for &y in &y_values {
        queue.push(y as i64);
    }
    let mut y_map = Vec::new();
    let mut prev = i32::MIN;
    while let Some(current) = queue.pop() {
        let y = current as i32;
        if y > prev {
            y_map.push(y);
            prev = y;
        }
    }

    // Enqueue all points as MER_LONG(x, y_discretized)
    queue.arr.clear();
    for i in 0..n {
        let y = y_coords[i];
        let y_discretized = binary_search(&y_map, y);
        let value = mer_long(x_coords[i], y_discretized);
        queue.push(value);
    }

    let mut x_map = Vec::new();
    let mut lists = Vec::new();
    let mut prefix = Vec::new();
    let mut result = -1i64;

    while !queue.arr.is_empty() {
        let current_x = high_int(queue.arr[0]);
        let mut current_column_ys = Vec::new();
        let mut current_prefix = Vec::new();
        while !queue.arr.is_empty() && high_int(queue.arr[0]) == current_x {
            let value = queue.pop().unwrap();
            let y = low_int(value) as i32;
            current_column_ys.push(y);
            let count = tree.push_count(y);
            current_prefix.push(count);
        }

        x_map.push(current_x);
        lists.push(current_column_ys.clone());
        prefix.push(current_prefix.clone());

        for j in 0..current_column_ys.len() {
            let current_y = current_column_ys[j];
            if j > 0 {
                let previous_y = current_column_ys[j - 1];
                let current_x_last = x_last[current_y as usize];
                let previous_x_last = x_last[previous_y as usize];

                if current_x_last != -1 && current_x_last == previous_x_last {
                    let previous_column = current_x_last as usize;
                    if previous_column >= lists.len() {
                        continue;
                    }
                    let lists_prev = &lists[previous_column];
                    let prefix_prev = &prefix[previous_column];

                    let x = binary_search(lists_prev, current_y);
                    let y_pos = binary_search(lists_prev, previous_y);
                    if x == -1 || y_pos == -1 {
                        continue;
                    }
                    let x_idx = x as usize;
                    let y_idx = y_pos as usize;

                    let num1 = current_prefix[j] - current_prefix[j - 1];
                    let num2 = prefix_prev[x_idx] - prefix_prev[y_idx];
                    let number = num1 - num2;

                    if x == y_pos + 1 && number == 1 {
                        let width = current_x - x_map[previous_column];
                        let height = y_map[current_y as usize] - y_map[previous_y as usize];
                        let area = (width as i64) * (height as i64);
                        if area > result {
                            result = area;
                        }
                    }
                }
            }

            let previous_x_last = x_last[current_y as usize];
            x_last[current_y as usize] = (lists.len() - 1) as i32;
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut x_coords = Vec::with_capacity(n);
    let mut y_coords = Vec::with_capacity(n);
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();
        x_coords.push(x);
        y_coords.push(y);
    }
    let result = max_rectangle_area(&x_coords, &y_coords);
    println!("{}", result);
}