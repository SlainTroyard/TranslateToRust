use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// A node in our priority queue.
#[derive(Eq, PartialEq)]
struct Node {
    time: i32,
    row: usize,
    col: usize,
}

// Implement ordering so that BinaryHeap becomes a min-heap by time.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Invert the comparison to turn the max-heap into a min-heap.
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// This function mirrors the logic of the original C function minTimeToReach.
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    // If no rows or no columns, return 0 immediately
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create visited array initialized to false
    let mut vis = vec![vec![false; cols]; rows];

    // Priority queue for (time, row, col)
    let mut min_heap = BinaryHeap::new();

    // Insert the starting point with time = 0
    min_heap.push(Node { time: 0, row: 0, col: 0 });
    vis[0][0] = true;

    // Direction arrays
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut final_time = 0;

    // Extract minimum node until empty
    while let Some(current_node) = min_heap.pop() {
        let time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;
        final_time = time;

        // If destination (bottom-right) is reached, return
        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        // Check all four directions
        for i in 0..4 {
            let nr = r as isize + drow[i];
            let nc = c as isize + dcol[i];

            if nr >= 0
                && nr < rows as isize
                && nc >= 0
                && nc < cols as isize
            {
                let nr_u = nr as usize;
                let nc_u = nc as usize;

                if !vis[nr_u][nc_u] {
                    // nextCost = 2 - ((nr + nc) % 2)
                    let next_cost = 2 - ((nr_u + nc_u) % 2) as i32;

                    let new_time = if move_time[nr_u][nc_u] >= time {
                        move_time[nr_u][nc_u] + next_cost
                    } else {
                        time + next_cost
                    };

                    // Insert into the priority queue and mark visited
                    min_heap.push(Node {
                        time: new_time,
                        row: nr_u,
                        col: nc_u,
                    });
                    vis[nr_u][nc_u] = true;
                }
            }
        }
    }

    // If somehow we exit the loop without reaching bottom-right, return the last known time
    final_time
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing rows and cols
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let rows: usize = parts[0].parse().unwrap();
    let cols: usize = parts[1].parse().unwrap();

    // Allocate a 2D vector for moveTime
    let mut move_time = vec![vec![0; cols]; rows];

    // Read the grid values. The original C code uses scanf("%d") in a nested loop,
    // which does not care about line boundaries. Here we read row-by-row.
    let mut input_nums = Vec::new();
    for line in lines {
        if let Ok(line_str) = line {
            // Collect all numbers in this line
            for num_str in line_str.split_whitespace() {
                input_nums.push(num_str.parse::<i32>().unwrap());
            }
            // If we've read all needed numbers, we can stop collecting
            if input_nums.len() >= rows * cols {
                break;
            }
        }
    }

    // Fill move_time with the parsed numbers
    let mut index = 0;
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = input_nums[index];
            index += 1;
        }
    }

    // Calculate the result
    let result = min_time_to_reach(&move_time);

    // Print the result to stdout
    println!("{}", result);
}