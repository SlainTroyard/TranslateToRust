use std::cmp::{max, Ordering};
use std::io::{self, BufRead};

#[derive(Debug)]
struct PriorityQueue {
    arr: Vec<i64>,
    arr_size: usize,
}

#[derive(Debug)]
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            arr: Vec::new(),
            arr_size: 0,
        }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr_size;
        self.arr_size += 1;
        self.arr.push(value);
        while son > 0 {
            let father = (son - 1) >> 1;
            if self.arr[son] < self.arr[father] {
                self.arr.swap(son, father);
                son = father;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) {
        self.arr_size -= 1;
        self.arr.swap(0, self.arr_size);
        let mut father = 0;
        loop {
            let left = (father << 1) + 1;
            let right = (father << 1) + 2;
            let mut son = father;
            if left < self.arr_size && self.arr[left] < self.arr[son] {
                son = left;
            }
            if right < self.arr_size && self.arr[right] < self.arr[son] {
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
        BinaryTree {
            arr: vec![0; size],
            highest_bit: 0,
        }
    }

    fn tree_highest_bit(&mut self, max: i32) {
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
            if bit & value != 0 {
                result += self.arr[((i << 1) + 1) as usize];
                i = (i << 1) + 2;
            } else {
                i = (i << 1) + 1;
            }
            self.arr[i as usize] += 1;
            bit >>= 1;
        }
        result += self.arr[i as usize];
        result
    }
}

fn binary_search(map: &[i32], map_size: usize, value: i32) -> i32 {
    let mut left = 0;
    let mut right = map_size as i32 - 1;
    if value < map[left as usize] {
        return -1;
    }
    while left < right {
        let mid = (left + right + 1) >> 1;
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
    let mut lists_buff = vec![0; n];
    let mut prefix_buff = vec![0; n];
    let mut x_last = vec![-1; n];
    let mut arr1 = vec![0; tree_size];
    let mut arr2 = Vec::with_capacity(n);
    let mut lists = vec![Vec::new(); n];
    let mut prefix = vec![Vec::new(); n];
    let mut queue = PriorityQueue::new();
    let mut tree = BinaryTree::new(tree_size);
    let mut result = -1;

    for &y in y_coord {
        queue.push(y as i64);
    }
    let mut y_map_size = 0;
    let mut k = i32::MIN;
    while queue.arr_size > 0 {
        if k < queue.arr[0] as i32 {
            k = queue.arr[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue.pop();
    }

    for (i, &x) in x_coord.iter().enumerate() {
        let y = binary_search(&y_map, y_map_size, y_coord[i]);
        queue.push(((x as i64) << 32) | (y as i64));
    }

    let mut buff_size = 0;
    let mut i = 0;
    while queue.arr_size > 0 {
        let mut j = 0;
        lists[i] = Vec::new();
        prefix[i] = Vec::new();
        x_map[i] = (queue.arr[0] >> 32) as i32;
        while queue.arr_size > 0 && x_map[i] == (queue.arr[0] >> 32) as i32 {
            let y = (queue.arr[0] & 0xFFFFFFFF) as i32;
            lists[i].push(y);
            prefix[i].push(tree.push_count(y));
            if j > 0 && x_last[y as usize] != -1 && x_last[y as usize] == k {
                let x = binary_search(&lists[k as usize], lists_size[k as usize], y);
                let y_prev = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j - 1]);
                let number = prefix[i][j] - prefix[i][j - 1] - prefix[k as usize][x as usize] + prefix[k as usize][y_prev as usize];
                if x - 1 == y_prev && number == 1 {
                    let t = ((x_map[i] - x_map[k as usize]) as i64) * ((y_map[y as usize] - y_map[lists[i][j - 1] as usize]) as i64);
                    result = max(result, t);
                }
            }
            k = x_last[y as usize];
            x_last[y as usize] = i as i32;
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
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        x_coord.push(iter.next().unwrap().parse().unwrap());
        y_coord.push(iter.next().unwrap().parse().unwrap());
    }
    let max_area = max_rectangle_area(&x_coord, &y_coord);
    println!("{}", max_area);
}