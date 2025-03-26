use std::collections::VecDeque;
use std::io;
use std::cmp::{max, min};

const INF: i32 = i32::MAX;
const N: usize = 50;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Solution {
    pos: Vec<Position>,
    dist: Vec<Vec<i32>>,
    n: usize,
}

impl Solution {
    fn new(num_positions: usize) -> Solution {
        Solution {
            pos: vec![Position { x: 0, y: 0 }; num_positions + 1],
            dist: vec![vec![0; num_positions]; num_positions + 1],
            n: num_positions,
        }
    }
}

fn is_valid(x: isize, y: isize) -> bool {
    x >= 0 && x < N as isize && y >= 0 && y < N as isize
}

fn calculate_distances(sol: &mut Solution) {
    let directions = [
        (-2, -1), (-2, 1), (-1, -2), (-1, 2),
        (1, -2), (1, 2), (2, -1), (2, 1),
    ];

    for i in 0..=sol.n {
        for j in 0..sol.n {
            if i == j {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(sol.pos[i]);
            let mut seen = vec![vec![false; N]; N];
            seen[sol.pos[i].x][sol.pos[i].y] = true;
            let mut steps = 0;

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let current = queue.pop_front().unwrap();
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
                        break;
                    }

                    for &(dx, dy) in &directions {
                        let nx = current.x as isize + dx;
                        let ny = current.y as isize + dy;
                        if is_valid(nx, ny) {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if !seen[nx][ny] {
                                queue.push_back(Position { x: nx, y: ny });
                                seen[nx][ny] = true;
                            }
                        }
                    }
                }
                steps += 1;
            }
        }
    }
}

fn dfs(
    sol: &Solution,
    memo: &mut Vec<Vec<Vec<Option<i32>>>>,
    i: usize,
    m: usize,
    turn: usize,
) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if let Some(value) = memo[i][m][turn] {
        return value;
    }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if m & (1 << k) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, memo, k, next_m, 1 - turn);
            if turn == 0 {
                ans = max(ans, result);
            } else {
                ans = min(ans, result);
            }
        }
    }

    memo[i][m][turn] = Some(ans);
    ans
}

fn max_moves(
    kx: usize,
    ky: usize,
    positions: Vec<Position>,
) -> i32 {
    let positions_size = positions.len();
    let mut sol = Solution::new(positions_size);
    sol.n = positions_size;

    for (i, pos) in positions.iter().enumerate() {
        sol.pos[i] = *pos;
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };

    calculate_distances(&mut sol);

    let mut memo = vec![vec![vec![None; 2]; 1 << positions_size]; positions_size + 1];
    dfs(&sol, &mut memo, sol.n, 0, 0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut values = input.trim().split_whitespace();

    let kx: usize = values.next().unwrap().parse().unwrap();
    let ky: usize = values.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let positions_size: usize = input.trim().parse().unwrap();

    let mut positions = vec![];
    for _ in 0..positions_size {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        positions.push(Position { x: coords[0], y: coords[1] });
    }

    let result = max_moves(kx, ky, positions);
    println!("{}", result);
}