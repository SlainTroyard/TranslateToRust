use std::io::{self, Read, Write};
use std::cmp::max;

// Constants for tree operations
const FATHER_NODE: fn(usize) -> isize = |i| if i == 0 { -1 } else { (i as isize - 1) / 2 };
const LEFT_NODE: fn(usize) -> usize = |i| (i << 1) + 1;
const RIGHT_NODE: fn(usize) -> usize = |i| (i << 1) + 2;
const HIGH_INT: fn(i64) -> i32 = |i| (i >> 32) as i32;
const LOW_INT: fn(i64) -> i32 = |i| (i & 0xFFFFFFFF) as i32;
const MER_LONG: fn(i32, i32) -> i64 = |i, j| ((i as i64) << 32) | (j as i64);

// Priority queue structure
struct PriorityQueue {
    arr: Vec<i64>,
    arr_size: usize,
}

// Binary tree structure
struct BinaryTree {
    arr: Vec<i32>,
    highest_bit: i32,
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
        let mut father = FATHER_NODE(son) as usize;
        self.arr_size += 1;
        while father != usize::MAX && value < self.arr[father] {
            self.arr[son] = self.arr[father];
            son = father;
            father = FATHER_NODE(son) as usize;
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        let mut father = 0;
        let mut left = LEFT_NODE(father);
        let mut right = RIGHT_NODE(father);
        let mut son = 0;
        self.arr_size -= 1;
        while (self.arr_size > left && self.arr[self.arr_size] > self.arr[left])
            || (self.arr_size > right && self.arr[self.arr_size] > self.arr[right])
        {
            son = if self.arr_size > right && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            self.arr[father] = self.arr[son];
            father = son;
            left = LEFT_NODE(father);
            right = RIGHT_NODE(father);
        }
        self.arr[father] = self.arr[self.arr_size];
    }
}

impl BinaryTree {
    fn new(capacity: usize) -> Self {
        BinaryTree {
            arr: vec![0; capacity],
            highest_bit: 0,
        }
    }

    fn highest_bit(&mut self, max: i32) {
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

    fn push_count(&mut self, value: i32) -> i32 {
        let mut i = 0;
        let mut bit = self.highest_bit;
        let mut result = 0;
        while bit != 0 {
            if bit & value != 0 {
                result += self.arr[LEFT_NODE(i)];
                i = RIGHT_NODE(i);
            } else {
                i = LEFT_NODE(i);
            }
            self.arr[i] += 1;
            bit >>= 1;
        }
        result += self.arr[i];
        result
    }
}

fn binary_search(map: &[i32], map_size: usize, value: i32) -> i32 {
    let mut mid = -1;
    let mut left = 0;
    let mut right = map_size as i32 - 1;
    if value < map[left as usize] {
        return mid;
    }
    while left < right {
        mid = (left + right + 1) / 2;
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
    let mut arr2 = vec![0; n];
    let mut lists = vec![Vec::new(); n];
    let mut prefix = vec![Vec::new(); n];
    let mut tree = BinaryTree::new(tree_size);
    let mut queue = PriorityQueue::new(n);
    let mut result = -1;
    let mut k = i32::MIN;
    let mut y_map_size = 0;
    let mut buff_size = 0;

    // Initialization
    tree.highest_bit((n - 1) as i32);

    // Discretize all vertical coordinates
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

    // Enqueue all coordinate points
    for j in 0..n {
        let y = binary_search(&y_map[..y_map_size], y_map_size, y_coord[j]);
        queue.push(MER_LONG(x_coord[j], y));
    }

    let mut i = 0;
    while queue.arr_size > 0 {
        let mut j = 0;
        lists[i] = Vec::new();
        prefix[i] = Vec::new();
        x_map[i] = HIGH_INT(queue.arr[0]);
        while queue.arr_size > 0 && x_map[i] == HIGH_INT(queue.arr[0]) {
            lists[i].push(LOW_INT(queue.arr[0]));
            prefix[i].push(tree.push_count(lists[i][j]));
            if j > 0 && x_last[lists[i][j] as usize] != -1 && x_last[lists[i][j] as usize] == k as usize {
                let x = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j]);
                let y = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j - 1]);
                let number = prefix[i][j] - prefix[i][j - 1] - prefix[k as usize][x as usize] + prefix[k as usize][y as usize];
                if x - 1 == y && number == 1 {
                    let t = (x_map[i] - x_map[k as usize]) as i64 * (y_map[lists[i][j] as usize] - y_map[lists[i][j - 1] as usize]) as i64;
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    // Input the number of points
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    // Input the coordinates of the points
    for _ in 0..n {
        let mut coords = lines.next().unwrap().split_whitespace();
        x_coord.push(coords.next().unwrap().parse().unwrap());
        y_coord.push(coords.next().unwrap().parse().unwrap());
    }

    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, &y_coord);

    // Output the result
    writeln!(io::stdout(), "{}", max_area)?;

    Ok(())
}