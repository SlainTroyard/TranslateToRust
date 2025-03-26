use std::io::{self, Read};

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

fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let n = positions.len();
    let mut dis = vec![[[-1i32; 50]; 50]; n];

    // Precompute BFS distances for each soldier's position
    for i in 0..n {
        let px = positions[i][0] as usize;
        let py = positions[i][1] as usize;
        dis[i][px][py] = 0;
        let mut queue = vec![(px, py)];
        let mut step = 1;
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            for (qx, qy) in queue {
                for (dx, dy) in DIRS.iter() {
                    let x = (qx as i32) + dx;
                    let y = (qy as i32) + dy;
                    if x >= 0 && x < 50 && y >= 0 && y < 50 {
                        let x = x as usize;
                        let y = y as usize;
                        if dis[i][x][y] == -1 {
                            dis[i][x][y] = step;
                            next_queue.push((x, y));
                        }
                    }
                }
            }
            queue = next_queue;
            step += 1;
        }
    }

    let mut positions = positions;
    positions.push(vec![kx, ky]); // Add knight's position as the last element
    let mask_size = 1 << n;
    let mut memo = vec![vec![-1i32; mask_size]; positions.len()];
    let u = (1 << n) - 1;

    // Helper function to perform DFS with memoization
    fn dfs(
        i: usize,
        mask: u32,
        positions: &[Vec<i32>],
        dis: &[[[i32; 50]; 50]],
        u: u32,
        memo: &mut [Vec<i32>],
        n: usize,
    ) -> i32 {
        if mask == 0 {
            return 0;
        }
        let memo_val = memo[i][mask as usize];
        if memo_val != -1 {
            return memo_val;
        }

        let x = positions[i][0] as usize;
        let y = positions[i][1] as usize;
        let alice_turn = (u ^ mask).count_ones() % 2 == 0;

        let mut result = if alice_turn {
            i32::MIN
        } else {
            i32::MAX
        };

        for j in 0..n {
            if (mask & (1 << j)) != 0 {
                let next_mask = mask ^ (1 << j);
                let steps = dis[j][x][y];
                let val = dfs(j, next_mask, positions, dis, u, memo, n) + steps;
                if alice_turn {
                    if val > result {
                        result = val;
                    }
                } else {
                    if val < result {
                        result = val;
                    }
                }
            }
        }

        memo[i][mask as usize] = result;
        result
    }

    dfs(positions.len() - 1, u, &positions, &dis, u, &mut memo, n)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    
    let kx = tokens.next().unwrap();
    let ky = tokens.next().unwrap();
    let n = tokens.next().unwrap() as usize;
    
    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        let x = tokens.next().unwrap();
        let y = tokens.next().unwrap();
        positions.push(vec![x, y]);
    }
    
    println!("{}", max_moves(kx, ky, positions));
}