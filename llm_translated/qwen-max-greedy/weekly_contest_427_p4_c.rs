use std::io::{self, BufRead, Write};

// Define macros for node calculations
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

// Priority queue structure
struct PriorityQueue {
    arr: Vec<i64>,
    arr_size: usize,
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
        let mut father = father_node!(son);
        self.arr_size += 1;
        while father != -1 && value < self.arr[father] {
            self.arr[son] = self.arr[father];
            son = father;
            father = father_node!(son);
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        self.arr_size -= 1;
        let mut father = 0;
        let mut left = left_node!(father);
        let mut right = right_node!(father);
        while (left < self.arr_size && self.arr[self.arr_size] > self.arr[left])
            || (right < self.arr_size && self.arr[self.arr_size] > self.arr[right])
        {
            let son = if right < self.arr_size && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            self.arr[father] = self.arr[son];
            father = son;
            left = left_node!(father);
            right = right_node!(father);
        }
        self.arr[father] = self.arr[self.arr_size];
    }
}

// Binary tree structure
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
}

impl BinaryTree {
    fn new(max: i32) -> Self {
        let mut tree = BinaryTree {
            arr: vec![0; (max * 3) as usize],
            highest_bit: 0,
        };
        tree.tree_highest_bit(max);
        tree
    }

    fn tree_highest_bit(&mut self, max: i32) {
        let mut i = 1;
        if max != 0 {
            let mut max = max;
            while max != 0 {
                i += 1;
                max >>= 1;
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

fn max_rectangle_area(x_coord: &[i32], x_coord_size: usize, y_coord: &[i32], y_coord_size: usize) -> i64 {
    let n = x_coord_size;
    let tree_size = n * 3;
    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists_buff = vec![0; n];
    let mut prefix_buff = vec![0; n];
    let mut x_last = vec![-1; n];
    let mut lists = vec![&lists_buff[..]; n];
    let mut prefix = vec![&prefix_buff[..]; n];
    let mut arr1 = vec![0; tree_size];
    let mut arr2 = vec![0; n];
    let mut tree = BinaryTree::new(n as i32 - 1);
    let mut queue = PriorityQueue::new();
    let mut t = 0;
    let mut result = -1;

    // Discretize all vertical coordinates
    for j in 0..n {
        queue.push(y_coord[j] as i64);
    }
    let mut k = i32::MIN;
    while queue.arr_size > 0 {
        if k < queue.arr[0] as i32 {
            k = queue.arr[0] as i32;
            y_map[y_map.len()] = k;
        }
        queue.pop();
    }

    // Enqueue all coordinate points
    for j in 0..n {
        let y = binary_search(&y_map, y_map.len(), y_coord[j]);
        queue.push(mer_long!(x_coord[j] as i64, y as i64));
    }

    let mut buff_size = 0;
    let mut i = 0;
    while queue.arr_size > 0 {
        let mut j = 0;
        lists[i] = &lists_buff[buff_size..];
        prefix[i] = &prefix_buff[buff_size..];
        x_map[i] = high_int!(queue.arr[0]);
        while queue.arr_size > 0 && x_map[i] == high_int!(queue.arr[0]) {
            lists[i][j] = low_int!(queue.arr[0]) as i32;
            prefix[i][j] = tree.tree_push_count(lists[i][j]);
            if j > 0 && x_last[lists[i][j] as usize] != -1 && x_last[lists[i][j] as usize] == k as i32 {
                let x = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j]);
                let y = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j - 1]);
                let number = prefix[i][j] - prefix[i][j - 1] - prefix[k as usize][x as usize] + prefix[k as usize][y as usize];
                if x - 1 == y && number == 1 {
                    t = (x_map[i] - x_map[k as usize]) as i64 * (y_map[lists[i][j] as usize] - y_map[lists[i][j - 1] as usize]) as i64;
                    result = max_val!(result, t);
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
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of points
    stdin.lock().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the coordinates of the points
    let mut x_coord = vec![0; n];
    let mut y_coord = vec![0; n];
    for i in 0..n {
        stdin.lock().read_line(&mut buffer).unwrap();
        let coords: Vec<i32> = buffer.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        x_coord[i] = coords[0];
        y_coord[i] = coords[1];
        buffer.clear();
    }

    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, n, &y_coord, n);

    // Output the result
    writeln!(stdout, "{}", max_area).unwrap();
}