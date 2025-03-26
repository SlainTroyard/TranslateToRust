use std::cmp;

struct PriorityQueue {
    arr: Vec<i64>,
    size: usize,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            arr: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, value: i64) {
        self.arr.push(value);
        self.size += 1;
        let mut son = self.size - 1;
        while son > 0 {
            let father = (son - 1) / 2;
            if self.arr[son] < self.arr[father] {
                self.arr.swap(son, father);
                son = father;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i64> {
        if self.size == 0 {
            return None;
        }
        let result = self.arr[0];
        self.arr[0] = self.arr[self.size - 1];
        self.size -= 1;
        let mut father = 0;
        loop {
            let left = 2 * father + 1;
            let right = 2 * father + 2;
            let mut son = father;
            if left < self.size && self.arr[left] < self.arr[son] {
                son = left;
            }
            if right < self.size && self.arr[right] < self.arr[son] {
                son = right;
            }
            if son != father {
                self.arr.swap(father, son);
                father = son;
            } else {
                break;
            }
        }
        Some(result)
    }
}

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

    fn set_highest_bit(&mut self, max: i32) {
        if max == 0 {
            self.highest_bit = 1;
        } else {
            let mut i = 1;
            let mut m = max;
            while m != 0 {
                i += 1;
                m >>= 1;
            }
            self.highest_bit = 1 << (i - 2);
        }
    }

    fn push_count(&mut self, value: i32) -> i32 {
        let mut i = 0;
        let mut bit = self.highest_bit;
        let mut result = 0;
        while bit != 0 {
            if (value & bit) != 0 {
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

fn left_node(i: usize) -> usize {
    i * 2 + 1
}

fn right_node(i: usize) -> usize {
    i * 2 + 2
}

fn binary_search(map: &[i32], value: i32) -> i32 {
    if map.is_empty() || value < map[0] {
        return -1;
    }
    let mut left = 0;
    let mut right = map.len() - 1;
    while left < right {
        let mid = (left + right + 1) / 2;
        if map[mid] > value {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left as i32
}

fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    if n == 0 {
        return 0;
    }

    let mut y_queue = PriorityQueue::new();
    for &y in y_coord {
        y_queue.push(y as i64);
    }

    let mut y_map = Vec::new();
    let mut k = i64::MIN;
    while let Some(val) = y_queue.pop() {
        if val > k {
            k = val;
            y_map.push(k as i32);
        }
    }

    let mut merged_queue = PriorityQueue::new();
    for i in 0..n {
        let y = binary_search(&y_map, y_coord[i]);
        if y == -1 {
            continue;
        }
        let merged = (x_coord[i] as i64) << 32 | (y as i64);
        merged_queue.push(merged);
    }

    let mut x_map = Vec::new();
    let mut lists = Vec::new();
    let mut prefix = Vec::new();
    let mut x_last = vec![-1; y_map.len()];
    let tree_size = n * 3;
    let mut tree = BinaryTree::new(tree_size);
    tree.set_highest_bit((n - 1) as i32);

    let mut result = -1;

    while let Some(merged) = merged_queue.pop() {
        let x = (merged >> 32) as i32;
        let y_idx = (merged & 0xFFFFFFFF) as i32;

        if x_map.is_empty() || x != *x_map.last().unwrap() {
            x_map.push(x);
            lists.push(Vec::new());
            prefix.push(Vec::new());
        }

        let list = lists.last_mut().unwrap();
        let p = prefix.last_mut().unwrap();

        let count = tree.push_count(y_idx);
        p.push(count);
        list.push(y_idx);

        let j = list.len() - 1;
        if j > 0 {
            let prev_y = list[j - 1];
            let k_val = x_last[y_idx as usize];
            if k_val != -1 && k_val == x_last[prev_y as usize] {
                let k_list = &lists[k_val as usize];
                let x_idx = binary_search(k_list, y_idx);
                let y_idx_prev = binary_search(k_list, prev_y);
                if x_idx != -1 && y_idx_prev != -1 && x_idx == y_idx_prev + 1 {
                    let p_k = &prefix[k_val as usize];
                    let num = p[j] - p[j - 1] - p_k[x_idx as usize] + p_k[y_idx_prev as usize];
                    if num == 1 {
                        let x_diff = (x - x_map[k_val as usize]) as i64;
                        let y_diff = (y_map[y_idx as usize] - y_map[prev_y as usize]) as i64;
                        result = cmp::max(result, x_diff * y_diff);
                    }
                }
            }
        }

        x_last[y_idx as usize] = (x_map.len() - 1) as i32;
    }

    if result == -1 { 0 } else { result }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        x_coord.push(x);
        y_coord.push(y);
    }

    let max_area = max_rectangle_area(&x_coord, &y_coord);
    println!("{}", max_area);
}