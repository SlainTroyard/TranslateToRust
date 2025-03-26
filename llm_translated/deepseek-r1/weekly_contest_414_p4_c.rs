use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Solution {
    positions: Vec<Position>,
    n: usize,
    dist: Vec<Vec<i32>>,
}

impl Solution {
    fn new(knight_pos: Position, target_positions: Vec<Position>) -> Self {
        let n = target_positions.len();
        let mut positions = target_positions;
        positions.push(knight_pos);
        let dist = vec![vec![0; n]; n + 1];
        Self { positions, n, dist }
    }

    fn calculate_distances(&mut self) {
        const DIRECTIONS: [(i32, i32); 8] = [
            (-2, -1), (-2, 1),
            (-1, -2), (-1, 2),
            (1, -2), (1, 2),
            (2, -1), (2, 1),
        ];

        for i in 0..=self.n {
            for j in 0..self.n {
                if i == j {
                    continue;
                }

                let start = self.positions[i];
                let target = self.positions[j];
                let mut queue = VecDeque::new();
                let mut visited = [[false; 50]; 50];
                queue.push_back((start.x, start.y));
                visited[start.x as usize][start.y as usize] = true;
                let mut steps = 0;
                let mut found = false;

                while !queue.is_empty() {
                    let size = queue.len();
                    for _ in 0..size {
                        let (x, y) = queue.pop_front().unwrap();
                        if x == target.x && y == target.y {
                            self.dist[i][j] = steps;
                            found = true;
                            break;
                        }
                        for &(dx, dy) in &DIRECTIONS {
                            let nx = x + dx;
                            let ny = y + dy;
                            if nx >= 0 && nx < 50 && ny >= 0 && ny < 50 {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                if !visited[nx][ny] {
                                    visited[nx][ny] = true;
                                    queue.push_back((nx as i32, ny as i32));
                                }
                            }
                        }
                    }
                    if found {
                        break;
                    }
                    steps += 1;
                }
            }
        }
    }
}

fn dfs(
    sol: &Solution,
    i: usize,
    m: u64,
    turn: usize,
    memo: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m as usize][turn] != -1 {
        return memo[i][m as usize][turn];
    }

    let mut ans = if turn == 0 { 0 } else { i32::MAX };

    for k in 0..sol.n {
        if (m & (1 << k)) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }

    memo[i][m as usize][turn] = ans;
    ans
}

fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let n = positions.len();
    let target_positions: Vec<Position> = positions
        .iter()
        .map(|p| Position { x: p[0], y: p[1] })
        .collect();
    let knight_pos = Position { x: kx, y: ky };
    let mut sol = Solution::new(knight_pos, target_positions);
    sol.calculate_distances();

    let mut memo = vec![vec![vec![-1; 2]; 1 << n]; n + 1];

    dfs(&sol, n, 0, 0, &mut memo)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let (kx, ky) = (parts[0], parts[1]);

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let positions_size: usize = input.trim().parse().expect("Invalid input");

    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let parts: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        positions.push(parts);
    }

    let result = max_moves(kx, ky, positions);
    println!("{}", result);
}