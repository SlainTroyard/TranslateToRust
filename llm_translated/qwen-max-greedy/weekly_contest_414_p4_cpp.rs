use std::io::{self, BufRead, Write};

const DIRS: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];

fn max_moves(kx: i32, ky: i32, positions: &mut Vec<(i32, i32)>) -> i32 {
    let n = positions.len();
    let mut dis = vec![vec![vec![-1; 50]; 50]; n];

    for i in 0..n {
        let (px, py) = positions[i];
        dis[i][px as usize][py as usize] = 0;
        let mut q = vec![(px, py)];

        for step in 1.. {
            let mut tmp = Vec::new();
            for (qx, qy) in q.iter() {
                for (dx, dy) in DIRS.iter() {
                    let x = qx + dx;
                    let y = qy + dy;
                    if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] == -1 {
                        dis[i][x as usize][y as usize] = step;
                        tmp.push((x, y));
                    }
                }
            }
            if tmp.is_empty() {
                break;
            }
            q = tmp;
        }
    }

    positions.push((kx, ky));
    let u = (1 << n) - 1;
    let mut memo = vec![vec![-1; 1 << n]; n + 1];

    fn dfs(i: usize, mask: i32, n: usize, u: i32, dis: &Vec<Vec<Vec<i32>>>, positions: &Vec<(i32, i32)>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if mask == 0 {
            return 0;
        }
        if memo[i][mask as usize] != -1 {
            return memo[i][mask as usize];
        }
        let (x, y) = positions[i];
        let mut res = if (u ^ mask).count_ones() % 2 == 0 {
            // Alice's move
            (0..n).filter(|&j| mask & (1 << j) != 0)
                  .map(|j| dfs(j, mask ^ (1 << j), n, u, dis, positions, memo) + dis[j][x as usize][y as usize])
                  .max().unwrap_or(0)
        } else {
            // Bob's move
            (0..n).filter(|&j| mask & (1 << j) != 0)
                  .map(|j| dfs(j, mask ^ (1 << j), n, u, dis, positions, memo) + dis[j][x as usize][y as usize])
                  .min().unwrap_or(i32::MAX)
        };
        memo[i][mask as usize] = res;
        res
    }

    dfs(n, u, n, u, &dis, &positions, &mut memo)
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let kx: i32 = iter.next().unwrap().parse().unwrap();
    let ky: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let px: i32 = iter.next().unwrap().parse().unwrap();
        let py: i32 = iter.next().unwrap().parse().unwrap();
        positions.push((px, py));
    }

    let result = max_moves(kx, ky, &mut positions);
    writeln!(stdout, "{}", result).unwrap();
}