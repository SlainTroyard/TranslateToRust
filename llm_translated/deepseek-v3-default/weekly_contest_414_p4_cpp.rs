use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io;

const DIRS: [(i32, i32); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let kx: i32 = iter.next().unwrap().parse().unwrap();
    let ky: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        positions.push((x, y));
    }

    let result = max_moves(kx, ky, &positions);
    println!("{}", result);
}

fn max_moves(kx: i32, ky: i32, positions: &[(i32, i32)]) -> i32 {
    let n = positions.len();
    let mut dis = vec![vec![vec![-1; 50]; 50]; n];

    // Calculate the number of moves required for the knight to reach each position
    for i in 0..n {
        let (px, py) = positions[i];
        dis[i][px as usize][py as usize] = 0;
        let mut q = VecDeque::new();
        q.push_back((px, py));

        // Perform a BFS to calculate the minimum steps from each soldier to all other positions
        let mut step = 1;
        while !q.is_empty() {
            let mut tmp = VecDeque::new();
            while let Some((qx, qy)) = q.pop_front() {
                for &(dx, dy) in &DIRS {
                    let x = qx + dx;
                    let y = qy + dy;
                    if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] == -1 {
                        dis[i][x as usize][y as usize] = step;
                        tmp.push_back((x, y));
                    }
                }
            }
            q = tmp;
            step += 1;
        }
    }

    // Add the knight's position to the list of positions
    let mut positions = positions.to_vec();
    positions.push((kx, ky));

    let mut memo = vec![vec![-1; 1 << n]; n + 1];
    let u = (1 << n) - 1;

    // Use a closure to allow recursion within the lambda
    let mut dfs = |i: usize, mask: usize| -> i32 {
        if mask == 0 {
            return 0; // No more soldiers to move
        }
        if memo[i][mask] != -1 {
            return memo[i][mask]; // If the result has been calculated before, return it
        }
        let (x, y) = positions[i];
        let mut res = if (u ^ mask).count_ones() % 2 == 0 {
            // Alice's move
            let mut max_val = 0;
            for j in 0..n {
                if (mask >> j) & 1 == 1 {
                    max_val = max(max_val, dfs(j, mask ^ (1 << j)) + dis[j][x as usize][y as usize]);
                }
            }
            max_val
        } else {
            // Bob's move
            let mut min_val = i32::MAX;
            for j in 0..n {
                if (mask >> j) & 1 == 1 {
                    min_val = min(min_val, dfs(j, mask ^ (1 << j)) + dis[j][x as usize][y as usize]);
                }
            }
            min_val
        };
        memo[i][mask] = res;
        res
    };

    dfs(n, u) // Start the DFS from the knight's position
}