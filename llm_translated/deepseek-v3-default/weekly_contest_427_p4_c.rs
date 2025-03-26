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
        let mut father = father_node(son);
        self.arr_size += 1;
        self.arr.push(value);
        while father != usize::MAX && value < self.arr[father] {
            self.arr[son] = self.arr[father];
            son = father;
            father = father_node(son);
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        self.arr_size -= 1;
        while (self.arr_size > left && self.arr[self.arr_size] > self.arr[left])
            || (self.arr_size > right && self.arr[self.arr_size] > self.arr[right])
        {
            let son = if self.arr_size > right && self.arr[left] > self.arr[right] {
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

impl BinaryTree {
    fn new(size: usize) -> Self {
        BinaryTree {
            arr: vec![0; size],
            highest_bit: 0,
        }
    }

    fn highest_bit(&mut self, max_val: i32) {
        let mut i = 1;
        if max_val != 0 {
            let mut max = max_val;
            while max != 0 {
                i += 1;
                max >>= 1;
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

fn father_node(i: usize) -> usize {
    if i == 0 {
        usize::MAX
    } else {
        (i - 1) >> 1
    }
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

fn mer_long(high: i32, low: i32) -> i64 {
    ((high as i64) << 32) | (low as i64)
}

fn binary_search(map: &[i32], map_size: usize, value: i32) -> i32 {
    let mut left = 0;
    let mut right = map_size as i32 - 1;
    if value < map[left as usize] {
        return -1;
    }
    while left < right {
        let mid = left + (right - left + 1) / 2;
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
    let mut lists = vec![Vec::new(); n];
    let mut prefix = vec![Vec::new(); n];
    let mut arr2 = Vec::with_capacity(n);
    let mut queue = PriorityQueue {
        arr: arr2,
        arr_size: 0,
    };
    let mut tree = BinaryTree {
        arr: arr1,
        highest_bit: 0,
    };
    let mut k = i32::MIN;
    let mut y_map_size = 0;
    let mut buff_size = 0;
    let mut result = -1;

    tree.highest_bit((n - 1) as i32);

    for &y in y_coord {
        queue.push(y as i64);
    }

    while queue.arr_size > 0 {
        if k < queue.arr[0] as i32 {
            k = queue.arr[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue.pop();
    }

    for j in 0..n {
        let y = binary_search(&y_map, y_map_size, y_coord[j]);
        queue.push(mer_long(x_coord[j], y));
    }

    let mut i = 0;
    while queue.arr_size > 0 {
        let mut j = 0;
        lists[i] = Vec::new();
        prefix[i] = Vec::new();
        x_map[i] = high_int(queue.arr[0]);
        while queue.arr_size > 0 && x_map[i] == high_int(queue.arr[0]) {
            lists[i].push(low_int(queue.arr[0]));
            prefix[i].push(tree.push_count(lists[i][j]));
            if j > 0 && x_last[lists[i][j] as usize] != -1 && x_last[lists[i][j] as usize] == k {
                let x = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j]);
                let y = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j - 1]);
                let number = prefix[i][j] - prefix[i][j - 1] - prefix[k as usize][x as usize]
                    + prefix[k as usize][y as usize];
                if x - 1 == y && number == 1 {
                    let t = (x_map[i] - x_map[k as usize]) as i64
                        * (y_map[lists[i][j] as usize] - y_map[lists[i][j - 1] as usize]) as i64;
                    result = max(result, t);
                }
            }
            k = x_last[lists[i][j] as usize];
            x_last[lists[i][j] as usize] = i as i32;
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