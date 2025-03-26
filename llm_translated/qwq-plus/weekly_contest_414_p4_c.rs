use std::collections::VecDeque;
use std::io;

const N: usize = 50;
const DIRECTIONS: [[i32; 2]; 8] = [
    [-2, -1], [-2, 1], [-1, -2], [-1, 2],
    [1, -2], [1, 2], [2, -1], [2, 1],
];

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Solution {
    pos: [Position; N + 1],
    dist: [[i32; N]; N + 1],
    n: usize,
}

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < 50 && y < 50
}

fn calculate_distances(sol: &mut Solution) {
    for i in 0..=sol.n {
        for j in 0..sol.n {
            if i == j {
                continue;
            }
            let target = sol.pos[j];
            let mut queue = VecDeque::new();
            let mut seen = [[false; 50]; 50];
            let start = sol.pos[i];
            queue.push_back(start);
            seen[start.x as usize][start.y as usize] = true;
            let mut steps = 0;
            let mut found = false;
            loop {
                let level_size = queue.len();
                if level_size == 0 {
                    break;
                }
                for _ in 0..level_size {
                    let current = queue.pop_front().unwrap();
                    if current.x == target.x && current.y == target.y {
                        sol.dist[i][j] = steps as i32;
                        found = true;
                        break;
                    }
                    for dir in &DIRECTIONS {
                        let nx = current.x + dir[0];
                        let ny = current.y + dir[1];
                        if is_valid(nx, ny) {
                            let nx_usize = nx as usize;
                            let ny_usize = ny as usize;
                            if !seen[nx_usize][ny_usize] {
                                seen[nx_usize][ny_usize] = true;
                                queue.push_back(Position { x: nx, y: ny });
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

fn dfs(
    sol: &Solution,
    i: usize,
    m: usize,
    turn: usize,
    memo: &mut [Vec<[i32; 2]>],
) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m][turn] != -1 {
        return memo[i][m][turn];
    }

    let mut ans = if turn == 0 { std::i32::MIN } else { std::i32::MAX };
    for k in 0..sol.n {
        if (m & (1 << k)) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] as i32 + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }

    memo[i][m][turn] = ans;
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let kx: i32 = parts.next().unwrap().parse().unwrap();
    let ky: i32 = parts.next().unwrap().parse().unwrap();

    let positions_size: usize = lines.next().unwrap().parse().unwrap();

    let mut positions = Vec::with_capacity(positions_size);
    for line in lines {
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        positions.push(Position { x, y });
    }

    let mut sol = Solution {
        n: positions_size,
        pos: [Position { x: 0, y: 0 }; N + 1],
        dist: [[0; N]; N + 1],
    };

    for i in 0..positions_size {
        sol.pos[i] = positions[i];
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };

    calculate_distances(&mut sol);

    let mut memo = vec![
        vec![[std::i32::MIN; 2]; (1 << positions_size) as usize];
        N + 1
    ];

    for i in 0..=N {
        for m in 0..(1 << positions_size) as usize {
            memo[i][m][0] = -1;
            memo[i][m][1] = -1;
        }
    }

    let result = dfs(&sol, positions_size, 0, 0, &mut memo);

    println!("{}", result);
}