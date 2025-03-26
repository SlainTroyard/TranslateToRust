use std::cmp::{max, min};
use std::io;
use std::io::Read;

// Macros (converted to inline functions where appropriate)

#[inline]
fn father_node(i: usize) -> Option<usize> {
    if i == 0 {
        None
    } else {
        Some((i - 1) >> 1)
    }
}

#[inline]
fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

#[inline]
fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

#[inline]
fn high_int(i: i64) -> i32 {
    (i >> 32) as i32
}

#[inline]
fn low_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}

#[inline]
fn mer_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64 & 0xFFFFFFFF)
}

#[inline]
fn max_val(i: i64, j: i64) -> i64 {
    max(i, j)
}

// Priority queue.
#[derive(Debug)]
struct PriorityQueue {
    arr: Vec<i64>,
    arr_size: usize,
}

impl PriorityQueue {
    fn new(capacity: usize) -> Self {
        PriorityQueue {
            arr: Vec::with_capacity(capacity),
            arr_size: 0,
        }
    }
}

// Binary tree.
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
}

// Push into the priority queue.
fn queue_push(queue: &mut PriorityQueue, value: i64) {
    let mut son = queue.arr_size;
    let mut father = father_node(son);
    queue.arr_size += 1;

    if queue.arr.len() < queue.arr_size {
        queue.arr.push(value);
    } else {
        queue.arr[queue.arr_size - 1] = value;
    }

    while let Some(f) = father {
        if value < queue.arr[f] {
            queue.arr.swap(son, f);
            son = f;
            father = father_node(son);
        } else {
            break;
        }
    }
}

// Pop from the priority queue.
fn queue_pop(queue: &mut PriorityQueue) {
    queue.arr_size -= 1;
    let last_value = queue.arr[queue.arr_size];
    let mut father = 0;
    let mut left = left_node(father);
    let mut right = right_node(father);
    let mut son;

    while (queue.arr_size > left && last_value > queue.arr[left])
        || (queue.arr_size > right && last_value > queue.arr[right])
    {
        if queue.arr_size > right && queue.arr[left] > queue.arr[right] {
            son = right;
        } else {
            son = left;
        }

        queue.arr[father] = queue.arr[son];
        father = son;
        left = left_node(father);
        right = right_node(father);

        if left >= queue.arr_size {
            break;
        }
    }

    queue.arr[father] = last_value;
}

// In a sorted array (without duplicates), find the largest index less than or equal to value.
// Return -1 if it doesn't exist.
fn binary_search(map: &[i32], value: i32) -> Option<usize> {
    if map.is_empty() || value < map[0] {
        return None;
    }

    let mut left = 0;
    let mut right = map.len() - 1;
    let mut mid;

    while left < right {
        mid = (left + right + 1) >> 1;
        if value < map[mid] {
            right = mid - 1;
        } else {
            left = mid;
        }
    }

    Some(left)
}

// Calculate the highest bit needed for storing values in the binary tree.
// If the input is 0, record as the first bit from the right.
fn tree_highest_bit(tree: &mut BinaryTree, max: i32) {
    let mut i = 1;
    let mut max_val = max;
    if 0 != max_val {
        while 0 != max_val {
            i += 1;
            max_val = max_val >> 1;
        }
        i = 1 << (i - 2);
    }
    tree.highest_bit = i;
}

// Add a value to the binary tree and return the count of values less than or equal to it.
fn tree_push_count(tree: &mut BinaryTree, value: i32) -> i32 {
    let mut i = 0;
    let mut bit = tree.highest_bit;
    let mut result = 0;

    while 0 != bit {
        if (bit & value) != 0 {
            result += tree.arr[left_node(i)];
            i = right_node(i);
        } else {
            i = left_node(i);
        }
        tree.arr[i] += 1;
        bit = bit >> 1;
    }
    result += tree.arr[i];
    result
}

// Main function.
fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    let tree_size = n * 3;

    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];

    //Preallocate these as Vecs, and then slice them later
    let mut lists_buff = vec![0; n * n]; //Sufficient size
    let mut prefix_buff = vec![0; n * n]; //Sufficient size

    let mut x_last = vec![-1; n]; // Initialize xLast to an invalid value -1

    //Create vectors for lists and prefix
    let mut lists: Vec<&mut [i32]> = Vec::with_capacity(n);
    let mut prefix: Vec<&mut [i32]> = Vec::with_capacity(n);

    let mut tree = BinaryTree::new(tree_size);
    let mut queue = PriorityQueue::new(n);

    let mut y_map_size = 0;
    let mut buff_size = 0;
    let mut k = i32::MIN;
    let mut result: i64 = -1;

    tree_highest_bit(&mut tree, (n - 1) as i32);

    // Discretize all vertical coordinates.
    for &y in y_coord {
        queue_push(&mut queue, y as i64);
    }

    while queue.arr_size > 0 {
        // Ensure no duplicate values in yMap by comparing with the previous value.
        if k < queue.arr[0] as i32 {
            k = queue.arr[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue_pop(&mut queue);
    }

    // Enqueue all coordinate points, recording the mapped discretized vertical coordinates.
    for j in 0..n {
        let y = binary_search(&y_map[..y_map_size], y_coord[j]).unwrap() as i32;
        queue_push(&mut queue, mer_long(x_coord[j], y));
    }

    // Dequeue column by column.
    let mut i = 0;
    while queue.arr_size > 0 {
        let mut j = 0;
        lists.push(&mut lists_buff[buff_size..]);
        prefix.push(&mut prefix_buff[buff_size..]);
        x_map[i] = high_int(queue.arr[0]);

        // Points with the same horizontal coordinate are treated as column xMap[i].
        while queue.arr_size > 0 && x_map[i] == high_int(queue.arr[0]) {
            // Record the mapped values (ascending order) of vertical coordinates
            // for this column and the interval prefix sum.
            lists[i][j] = low_int(queue.arr[0]);
            prefix[i][j] = tree_push_count(&mut tree, lists[i][j]);

            // If it can serve as the top-right corner of a rectangle.
            if j > 0 && x_last[lists[i][j] as usize] != -1 && x_last[lists[i][j] as usize] as i32 == k {
                // x and y represent the array indices of the two vertices on the left
                // in column xMap[k].
                let k_usize = k as usize;
                let x = binary_search(&lists[k_usize][..lists_size[k_usize]], lists[i][j])
                    .unwrap();
                let y = binary_search(&lists[k_usize][..lists_size[k_usize]], lists[i][j - 1])
                    .unwrap();

                let number = prefix[i][j] - prefix[i][j - 1]
                    - prefix[k_usize][x]
                    + prefix[k_usize][y];

                // If x and y are adjacent, and the interval only contains one vertex
                // (i.e., the top-right corner), the condition is satisfied.
                if x.wrapping_sub(1) == y && 1 == number {
                    let t = (x_map[i] as i64 - x_map[k_usize] as i64)
                        * (y_map[lists[i][j] as usize] as i64 - y_map[lists[i][j - 1] as usize] as i64);
                    result = max_val(result, t);
                }
            }

            // Update xLast of the current point's vertical coordinate to the current column index i.
            // Here, k records the xLast value of the point below it.
            let list_val = lists[i][j] as usize;
            k = x_last[list_val];
            x_last[list_val] = i as i32;

            // Dequeue and increment counter.
            queue_pop(&mut queue);
            j += 1;
        }

        // Update buffer space and index.
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

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut x_coord: Vec<i32> = Vec::with_capacity(n);
    let mut y_coord: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut coords = line.split_whitespace();
        x_coord.push(coords.next().unwrap().parse().unwrap());
        y_coord.push(coords.next().unwrap().parse().unwrap());
    }

    let max_area = max_rectangle_area(&x_coord, &y_coord);

    println!("{}", max_area);

    Ok(())
}