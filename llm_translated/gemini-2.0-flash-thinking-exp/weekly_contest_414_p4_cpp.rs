use std::io;
use std::cmp::{max, min};

const DIRS: &[[i32; 2]] = &[[2, 1], [1, 2], [-1, 2], [-2, 1], [-2, -1], [-1, -2], [1, -2], [2, -1]];

fn max_moves(kx: i32, ky: i32, positions: &mut Vec<Vec<i32>>) -> i32 {
    let n = positions.len();
    let mut dis = vec![vec![vec![-1; 50]; 50]; n];

    // Calculate the number of moves required for the knight to reach each position
    for i in 0..n {
        let px = positions[i][0];
        let py = positions[i][1];
        dis[i][px as usize][py as usize] = 0;
        let mut q = vec![(px, py)];
        let mut step = 1;
        while !q.is_empty() {
            let mut tmp = Vec::new();
            for &(qx, qy) in q.iter() {
                for &[dx, dy] in DIRS.iter() {
                    let x = qx + dx;
                    let y = qy + dy;
                    // Ensure the new position is within bounds and not yet visited
                    if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] < 0 {
                        dis[i][x as usize][y as usize] = step;
                        tmp.push((x, y));
                    }
                }
            }
            q = tmp;
            step += 1;
        }
    }

    // Add the knight's position to the list of positions
    positions.push(vec![kx, ky]);
    let mut memo = vec![vec![-1; 1 << n]; n + 1]; // -1 indicates that the result is not yet computed
    let u = (1 << n) - 1;

    // Use closure for recursion
    let mut dfs = |i_usize: usize, mask: i32, memo: &mut Vec<Vec<i32>>| -> i32 {
        if mask == 0 {
            return 0; // No more soldiers to move
        }
        let i = i_usize as usize;
        if memo[i][mask as usize] != -1 { // If the result has been calculated before, return it
            return memo[i][mask as usize];
        }
        let x = positions[i][0];
        let y = positions[i][1];
        let mut res = -1;
        if (u ^ mask).count_ones() % 2 == 0 { // Alice's move
            for j in 0..n {
                if (mask >> j) & 1 != 0 {
                    let val = dfs(j, mask ^ (1 << j), memo);
                    if val != -1 { // Avoid adding -1 if dfs returns uncomputed value
                        res = max(res, val + dis[j][x as usize][y as usize]);
                    }
                }
            }
        } else { // Bob's move
            res = i32::MAX;
            for j in 0..n {
                if (mask >> j) & 1 != 0 {
                    let val = dfs(j, mask ^ (1 << j), memo);
                     if val != -1 { // Avoid min with -1 if dfs returns uncomputed value
                        res = min(res, val + dis[j][x as usize][y as usize]);
                    }
                }
            }
        }
        memo[i][mask as usize] = res;
        res
    };

    dfs(n, u, &mut memo) // Start the DFS from the knight's position
}

fn main() {
    let mut kx = 0;
    let mut ky = 0;
    let mut n = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    kx = parts[0];
    ky = parts[1];
    n = parts[2];

    let mut positions: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let pos: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        positions.push(pos);
    }

    let result = max_moves(kx, ky, &mut positions);
    println!("{}", result);
}