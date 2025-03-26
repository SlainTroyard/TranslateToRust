use std::collections::BinaryHeap;
use std::cmp::{PartialOrd, Ord, Ordering};

#[derive(Debug)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use negative time to simulate min-heap behavior
        other.time.cmp(&self.time)
    }
}

impl Eq for HeapNode {}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.x == other.x && self.y == other.y
    }
}

fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    if rows == 0 {
        return 0;
    }
    let cols = move_time[0].len();
    if cols == 0 {
        return 0;
    }

    let mut time = vec![vec![i32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Push the starting point with time 0
    heap.push(HeapNode { time: 0, x: 0, y: 0 });
    time[0][0] = 0;

    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];

    while let Some(current_node) = heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x;
        let y = current_node.y;

        // Check if we've reached the destination
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Explore all four directions
        for i in 0..4 {
            let new_x = x as i32 + dx[i];
            let new_y = y as i32 + dy[i];

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // Calculate wait time if needed
                let wait_time = if move_time[new_x][new_y] > current_time {
                    move_time[new_x][new_y] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    heap.push(HeapNode { time: new_time, x: new_x, y: new_y });
                }
            }
        }
    }

    // If destination is unreachable (though problem assumes it's reachable)
    -1
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();
    let rows: usize = tokens.next().unwrap().parse().unwrap();
    let cols: usize = tokens.next().unwrap().parse().unwrap();

    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            let val: i32 = tokens.next().unwrap().parse().unwrap();
            row.push(val);
        }
        move_time.push(row);
    }

    let result = min_time_to_reach(move_time);
    println!("{}", result);
}