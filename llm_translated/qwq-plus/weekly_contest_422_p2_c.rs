use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

impl HeapNode {
    fn new(time: i32, x: usize, y: usize) -> Self {
        HeapNode { time, x, y }
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut time = vec![vec![i32::MAX; cols]; rows];
    time[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(HeapNode::new(0, 0, 0)));

    let directions = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];

    while let Some(Reverse(node)) = heap.pop() {
        let current_time = node.time;
        let x = node.x;
        let y = node.y;

        // Check if destination reached
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Skip outdated entries
        if current_time > time[x][y] {
            continue;
        }

        // Explore all directions
        for &(dx, dy) in directions.iter() {
            let new_x = (x as i32) + dx;
            let new_y = (y as i32) + dy;

            // Check boundaries
            if new_x < 0 || new_x >= rows as i32 || new_y < 0 || new_y >= cols as i32 {
                continue;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            // Calculate wait time and new_time
            let move_time_val = move_time[new_x][new_y];
            let wait_time = if move_time_val > current_time {
                move_time_val - current_time
            } else {
                0
            };
            let new_time = current_time + 1 + wait_time;

            // Update time and push to heap if better path found
            if new_time < time[new_x][new_y] {
                time[new_x][new_y] = new_time;
                heap.push(Reverse(HeapNode::new(new_time, new_x, new_y)));
            }
        }
    }

    // Unreachable if problem constraints are met
    -1
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    // Read rows and columns
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let rows: usize = parts.next().unwrap().parse().unwrap();
    let cols: usize = parts.next().unwrap().parse().unwrap();

    // Initialize move_time matrix
    let mut move_time = vec![vec![0; cols]; rows];

    // Read each row's data
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        move_time[i] = nums;
    }

    // Compute and print result
    let result = min_time_to_reach(&move_time, rows, cols);
    println!("{}", result);
}