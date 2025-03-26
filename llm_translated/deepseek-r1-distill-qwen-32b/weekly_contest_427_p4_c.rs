use std::io;
use std::cmp;

#[derive(Debug)]
struct PriorityQueue {
    arr: Vec<i64>,
}

#[derive(Debug)]
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

static mut X_LAST: Vec<i32> = Vec::new();
static mut LISTS_SIZE: Vec<i32> = Vec::new();
static mut BUFF_SIZE: i32 = 0;
static mut LISTS_BUFF: Vec<i32> = Vec::new();
static mut PREFIX_BUFF: Vec<i32> = Vec::new();

impl PriorityQueue {
    fn new() -> Self {
        Self {
            arr: Vec::new(),
        }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        let mut father = if son == 0 { -1 } else { (son - 1) >> 1 };
        self.arr.push(value);
        while father != -1 && value < self.arr[father as usize] {
            self.arr.swap(son, father as usize);
            son = father as usize;
            father = if son == 0 { -1 } else { (son - 1) >> 1 };
        }
    }

    fn pop(&mut self) {
        if self.arr.is_empty() {
            return;
        }
        self.arr.swap(0, self.arr.len() - 1);
        self.arr.pop();
        let mut father = 0;
        while father < self.arr.len() {
            let left = 2 * father + 1;
            let right = 2 * father + 2;
            let mut son = father;
            if left < self.arr.len() && self.arr[left] < self.arr[son] {
                son = left;
            }
            if right < self.arr.len() && self.arr[right] < self.arr[son] {
                son = right;
            }
            if son == father {
                break;
            }
            self.arr.swap(father, son);
            father = son;
        }
    }
}

impl BinaryTree {
    fn new(size: usize) -> Self {
        Self {
            arr: vec![0; size],
            highest_bit: 0,
        }
    }

    fn highest_bit(&mut self, max: i32) {
        let mut i = 1;
        if max != 0 {
            let mut m = max;
            while m != 0 {
                i += 1;
                m >>= 1;
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
                result += self.arr[2 * i + 1];
                i = 2 * i + 2;
            } else {
                i = 2 * i + 1;
            }
            self.arr[i] += 1;
            bit >>= 1;
        }
        result += self.arr[i];
        result
    }
}

fn binary_search(map: &[i32], value: i32) -> i32 {
    if map.is_empty() || value < map[0] {
        return -1;
    }
    let mut left = 0;
    let mut right = map.len() - 1;
    while left < right {
        let mid = left + right + 1 >> 1;
        if value < map[mid] {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left as i32
}

fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists = vec![vec![0; n]; n];
    let mut prefix = vec![vec![0; n]; n];
    let mut x_last = vec![-1; n];
    let mut arr1 = vec![0; n * 3];
    let mut arr2 = vec![0; n];
    let mut tree = BinaryTree::new(n * 3);
    let mut queue = PriorityQueue::new();

    unsafe {
        X_LAST = x_last.clone();
        LISTS_SIZE = lists_size.clone();
        BUFF_SIZE = 0;
        LISTS_BUFF = vec![0; n];
        PREFIX_BUFF = vec![0; n];
    }

    tree.highest_bit(n as i32 - 1);

    for j in 0..n {
        queue.push(y_coord[j] as i64);
    }

    let mut y_map_size = 0;
    let mut k = i32::MIN;
    while !queue.arr.is_empty() {
        let current = queue.arr[0];
        if k < current {
            k = current;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue.pop();
    }

    for j in 0..n {
        let y = binary_search(&y_map[..y_map_size], y_coord[j]);
        queue.push(((x_coord[j] as i64) << 32) | (y as i64));
    }

    let mut i = 0;
    while !queue.arr.is_empty() {
        let mut j = 0;
        let x = queue.arr[0];
        let x_val = (x >> 32) as i32;
        x_map[i] = x_val;

        while !queue.arr.is_empty() && x_map[i] == (queue.arr[0] >> 32) as i32 {
            let y = queue.arr[0] as i32;
            lists[i][j] = y;
            prefix[i][j] = tree.push_count(y);
            if j > 0 && unsafe { X_LAST[y as usize] } != -1 {
                let k = unsafe { X_LAST[y as usize] };
                if k >= 0 && k < n as i32 {
                    let x_list = &lists[k as usize];
                    let x_size = unsafe { LISTS_SIZE[k as usize] };
                    let x_idx = binary_search(&x_list[..x_size as usize], y);
                    let y_idx = binary_search(&x_list[..x_size as usize], lists[i][j - 1]);
                    if x_idx != -1 && y_idx != -1 {
                        let number = prefix[i][j] - prefix[i][j - 1] - prefix[k as usize][x_idx as usize] + prefix[k as usize][y_idx as usize];
                        if x_idx - y_idx == 1 && number == 1 {
                            let width = x_map[i] - x_map[k as usize];
                            let height = y_map[y as usize] - y_map[lists[i][j - 1] as usize];
                            let area = (width as i64) * (height as i64);
                            if area > 0 {
                                return cmp::max(area, 0);
                            }
                        }
                    }
                }
            }
            unsafe {
                let prev = X_LAST[y as usize];
                X_LAST[y as usize] = i as i32;
            }
            queue.pop();
            j += 1;
        }
        lists_size[i] = j;
        unsafe {
            BUFF_SIZE += j as i32;
        }
        i += 1;
    }

    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();

    let mut x_coord = vec![0; n as usize];
    let mut y_coord = vec![0; n as usize];

    for i in 0..n as usize {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        x_coord[i] = parts.next().unwrap().parse::<i32>().unwrap();
        y_coord[i] = parts.next().unwrap().parse::<i32>().unwrap();
    }

    let max_area = max_rectangle_area(&x_coord, &y_coord);
    println!("{}", max_area);
}