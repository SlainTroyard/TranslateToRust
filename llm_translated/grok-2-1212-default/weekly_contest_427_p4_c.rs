use std::io::{self, BufRead};
use std::cmp;

// Constants for binary tree and priority queue operations
const FATHER_NODE: fn(usize) -> isize = |i| if i == 0 { -1 } else { (i - 1) as isize / 2 };
const LEFT_NODE: fn(usize) -> usize = |i| i * 2 + 1;
const RIGHT_NODE: fn(usize) -> usize = |i| i * 2 + 2;
const HIGH_INT: fn(i64) -> i32 = |i| (i >> 32) as i32;
const LOW_INT: fn(i64) -> i32 = |i| (i & 0xFFFFFFFF) as i32;
const MER_LONG: fn(i32, i32) -> i64 = |i, j| ((i as i64) << 32) | (j as i64);
const MAX_VAL: fn(i64, i64) -> i64 = |i, j| cmp::max(i, j);

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

// Function to push a value into the priority queue
fn queue_push(queue: &mut PriorityQueue, value: i64) {
    let mut son = queue.arr_size;
    let mut father = FATHER_NODE(son);
    queue.arr_size += 1;
    queue.arr.push(value);

    while father != -1 && value < queue.arr[father as usize] {
        queue.arr[son] = queue.arr[father as usize];
        son = father as usize;
        father = FATHER_NODE(son);
    }
    queue.arr[son as usize] = value;
}

// Function to pop a value from the priority queue
fn queue_pop(queue: &mut PriorityQueue) {
    let mut father = 0;
    let mut left = LEFT_NODE(father);
    let mut right = RIGHT_NODE(father);
    let mut son = 0;

    queue.arr_size -= 1;
    while (queue.arr_size > left && queue.arr[queue.arr_size] > queue.arr[left])
        || (queue.arr_size > right && queue.arr[queue.arr_size] > queue.arr[right])
    {
        son = if queue.arr_size > right && queue.arr[left] > queue.arr[right] {
            right
        } else {
            left
        };
        queue.arr[father] = queue.arr[son];
        father = son;
        left = LEFT_NODE(father);
        right = RIGHT_NODE(father);
    }
    queue.arr[father] = queue.arr[queue.arr_size];
}

// Function to perform binary search
fn binary_search(map: &[i32], map_size: usize, value: i32) -> isize {
    let mut mid = -1;
    let mut left = 0;
    let mut right = map_size as isize - 1;

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

// Function to calculate the highest bit needed for the binary tree
fn tree_highest_bit(tree: &mut BinaryTree, max: i32) {
    let mut i = 1;
    if max != 0 {
        let mut max_copy = max;
        while max_copy != 0 {
            i += 1;
            max_copy >>= 1;
        }
        i = 1 << (i - 2);
    }
    tree.highest_bit = i;
}

// Function to add a value to the binary tree and return the count of values less than or equal to it
fn tree_push_count(tree: &mut BinaryTree, value: i32) -> i32 {
    let mut i = 0;
    let mut bit = tree.highest_bit;
    let mut result = 0;

    while bit != 0 {
        if bit & value != 0 {
            result += tree.arr[LEFT_NODE(i)];
            i = RIGHT_NODE(i);
        } else {
            i = LEFT_NODE(i);
        }
        tree.arr[i] += 1;
        bit >>= 1;
    }
    result += tree.arr[i];
    result
}

// Main function to calculate the maximum rectangle area
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

    let mut lists = vec![vec![]; n];
    let mut prefix = vec![vec![]; n];
    let mut arr2 = vec![0; n];
    let mut tree = BinaryTree {
        arr: arr1,
        highest_bit: 0,
    };
    let mut queue = PriorityQueue {
        arr: arr2,
        arr_size: 0,
    };

    let mut t = 0;
    let mut result = -1;

    tree_highest_bit(&mut tree, (n - 1) as i32);

    // Discretize all vertical coordinates
    for &y in y_coord {
        queue_push(&mut queue, y as i64);
    }

    let mut k = i32::MIN;
    let mut y_map_size = 0;
    while queue.arr_size > 0 {
        if k < queue.arr[0] as i32 {
            k = queue.arr[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue_pop(&mut queue);
    }

    // Enqueue all coordinate points
    for j in 0..n {
        let y = binary_search(&y_map[..y_map_size], y_map_size, y_coord[j]);
        queue_push(&mut queue, MER_LONG(x_coord[j], y as i32));
    }

    let mut i = 0;
    let mut buff_size = 0;

    // Dequeue column by column
    while queue.arr_size > 0 {
        let mut j = 0;
        lists[i] = Vec::new();
        prefix[i] = Vec::new();
        x_map[i] = HIGH_INT(queue.arr[0]);

        while queue.arr_size > 0 && x_map[i] == HIGH_INT(queue.arr[0]) {
            lists[i].push(LOW_INT(queue.arr[0]));
            prefix[i].push(tree_push_count(&mut tree, LOW_INT(queue.arr[0])));

            if j > 0 && x_last[LOW_INT(queue.arr[0]) as usize] != -1 && x_last[LOW_INT(queue.arr[0]) as usize] == k as isize {
                let x = binary_search(&lists[k as usize], lists_size[k as usize], LOW_INT(queue.arr[0])) as usize;
                let y = binary_search(&lists[k as usize], lists_size[k as usize], lists[i][j - 1]) as usize;
                let number = prefix[i][j] - prefix[i][j - 1] - prefix[k as usize][x] + prefix[k as usize][y];

                if x as isize - 1 == y as isize && number == 1 {
                    t = (x_map[i] - x_map[k as usize]) as i64 * (y_map[LOW_INT(queue.arr[0]) as usize] - y_map[lists[i][j - 1] as usize]) as i64;
                    result = MAX_VAL(result, t);
                }
            }

            k = x_last[LOW_INT(queue.arr[0]) as usize] as i32;
            x_last[LOW_INT(queue.arr[0]) as usize] = i as isize;
            queue_pop(&mut queue);
            j += 1;
        }

        lists_size[i] = j;
        buff_size += j;
        i += 1;
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the number of points
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    let mut x_coord = Vec::with_capacity(n);
    let mut y_coord = Vec::with_capacity(n);

    // Input the coordinates of the points
    for _ in 0..n {
        let coords: Vec<i32> = lines.next().unwrap()?.trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x_coord.push(coords[0]);
        y_coord.push(coords[1]);
    }

    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, &y_coord);

    // Output the result
    println!("{}", max_area);

    Ok(())
}