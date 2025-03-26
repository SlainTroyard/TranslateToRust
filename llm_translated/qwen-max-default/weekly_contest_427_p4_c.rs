use std::io::{self, BufRead, Write};

// Define macros for convenience
macro_rules! father_node {
    ($i:expr) => {
        if $i == 0 { -1 } else { ($i - 1) >> 1 }
    };
}

macro_rules! left_node {
    ($i:expr) => {
        ($i << 1) + 1
    };
}

macro_rules! right_node {
    ($i:expr) => {
        ($i << 1) + 2
    };
}

macro_rules! high_int {
    ($i:expr) => {
        $i >> 32
    };
}

macro_rules! low_int {
    ($i:expr) => {
        $i & 0xFFFFFFFF
    };
}

macro_rules! mer_long {
    ($i:expr, $j:expr) => {
        ((($i as u64) << 32) | ($j as u64)) as i64
    };
}

macro_rules! max_val {
    ($i:expr, $j:expr) => {
        if $i > $j { $i } else { $j }
    };
}

// Priority queue. Used for discretization and listing coordinate points from left to right and bottom to top.
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
        let mut father = father_node!(son);
        while father != -1 && value < self.arr[father] {
            self.arr[son] = self.arr[father];
            son = father;
            father = father_node!(son);
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        self.arr.swap(0, self.arr.len() - 1);
        self.arr.pop();
        let mut father = 0;
        let mut left = left_node!(father);
        let mut right = right_node!(father);
        while (left < self.arr.len() && self.arr[left] < self.arr[father])
            || (right < self.arr.len() && self.arr[right] < self.arr[father])
        {
            let son = if right < self.arr.len() && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            self.arr.swap(father, son);
            father = son;
            left = left_node!(father);
            right = right_node!(father);
        }
    }
}

// Binary tree. Used to calculate the number of values in a given interval and to add values into it.
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

impl BinaryTree {
    fn new(max: i32) -> Self {
        let mut tree = BinaryTree { arr: vec![0; (max * 3) as usize], highest_bit: 0 };
        tree.highest_bit(max);
        tree
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
                result += self.arr[left_node!(i)];
                i = right_node!(i);
            } else {
                i = left_node!(i);
            }
            self.arr[i] += 1;
            bit >>= 1;
        }
        result + self.arr[i]
    }
}

fn binary_search(map: &[i32], map_size: usize, value: i32) -> i32 {
    let mut mid = -1;
    let mut left = 0;
    let mut right = map_size - 1;
    if value < map[left] {
        return mid;
    }
    while left < right {
        mid = (left + right + 1) >> 1;
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
    let tree_size = n * 3;
    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists_buff = vec![0; n];
    let mut prefix_buff = vec![0; n];
    let mut x_last = vec![-1; n];
    let mut lists = vec![&lists_buff[0]; n];
    let mut prefix = vec![&prefix_buff[0]; n];
    let mut arr1 = vec![0; tree_size];
    let mut arr2 = vec![0; n];
    let mut tree = BinaryTree::new(n as i32 - 1);
    let mut queue = PriorityQueue::new();

    let mut t = 0;
    let mut result = -1;
    let mut k = i32::MIN;
    let mut y_map_size = 0;
    let mut buff_size = 0;

    // Discretize all vertical coordinates.
    for &y in y_coord.iter() {
        queue.push(y as i64);
    }
    while !queue.arr.is_empty() {
        if k < queue.arr[0] {
            k = queue.arr[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue.pop();
    }

    // Enqueue all coordinate points, recording the mapped discretized vertical coordinates.
    for j in 0..n {
        let y = binary_search(&y_map, y_map_size, y_coord[j]);
        queue.push(mer_long!(x_coord[j] as i64, y as i64));
    }

    // Dequeue column by column.
    while !queue.arr.is_empty() {
        let mut j = 0;
        lists[buff_size] = &lists_buff[buff_size];
        prefix[buff_size] = &prefix_buff[buff_size];
        x_map[buff_size] = high_int!(queue.arr[0]);
        while !queue.arr.is_empty() && x_map[buff_size] == high_int!(queue.arr[0]) {
            lists[buff_size][j] = low_int!(queue.arr[0]) as i32;
            prefix[buff_size][j] = tree.push_count(lists[buff_size][j]);

            if j > 0 && x_last[lists[buff_size][j] as usize] != -1 && x_last[lists[buff_size][j] as usize] == k {
                let x = binary_search(&lists[k as usize], lists_size[k as usize], lists[buff_size][j]);
                let y = binary_search(&lists[k as usize], lists_size[k as usize], lists[buff_size][j - 1]);
                let number = prefix[buff_size][j] - prefix[buff_size][j - 1] - prefix[k as usize][x as usize] + prefix[k as usize][y as usize];

                if x - 1 == y && number == 1 {
                    t = (x_map[buff_size] - x_map[k as usize]) as i64 * (y_map[lists[buff_size][j] as usize] - y_map[lists[buff_size][j - 1] as usize]) as i64;
                    result = max_val!(result, t);
                }
            }

            let prev_k = x_last[lists[buff_size][j] as usize];
            x_last[lists[buff_size][j] as usize] = buff_size as i32;
            k = prev_k;
            queue.pop();
            j += 1;
        }
        lists_size[buff_size] = j;
        buff_size += j;
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read the number of points
    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut x_coord = vec![0; n];
    let mut y_coord = vec![0; n];

    // Read the coordinates of the points
    for i in 0..n {
        stdin.read_line(&mut input).unwrap();
        let coords: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        x_coord[i] = coords[0];
        y_coord[i] = coords[1];
        input.clear();
    }

    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, &y_coord);

    // Output the result
    writeln!(stdout, "{}", max_area).unwrap();
}