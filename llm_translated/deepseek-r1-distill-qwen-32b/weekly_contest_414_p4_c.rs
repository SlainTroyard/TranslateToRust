use std::collections::VecDeque;
use std::io;

struct Position {
    x: i32,
    y: i32,
}

struct Solution {
    pos: Vec<Position>,
    dist: Vec<Vec<i32>>,
    n: usize,
}

struct Queue {
    queue: VecDeque<Position>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, p: Position) {
        self.queue.push_back(p);
    }

    fn dequeue(&mut self) -> Position {
        self.queue.pop_front().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < 50 && y >= 0 && y < 50
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
            let mut queue = Queue::new();
            let start = sol.pos[i];
            queue.enqueue(start);
            let mut seen = vec![vec![false; 50]; 50];
            seen[start.x as usize][start.y as usize] = true;
            let mut steps = 0;

            while !queue.is_empty() {
                let size = queue.queue.len();
                for _ in 0..size {
                    let current = queue.dequeue();
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
                        break;
                    }
                    for &(dx, dy) in &directions {
                        let nx = current.x + dx;
                        let ny = current.y + dy;
                        if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                            seen[nx as usize][ny as usize] = true;
                            queue.enqueue(Position { x: nx, y: ny });
                        }
                    }
                }
                steps += 1;
            }
        }
    }
}

fn dfs(sol: &Solution, i: usize, m: usize, turn: i32, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m][turn as usize] != -1 {
        return memo[i][m][turn as usize];
    }

    let mut ans = if turn == 0 { 0 } else { i32::MAX };
    for k in 0..sol.n {
        if !(m & (1 << k)) {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }
    memo[i][m][turn as usize] = ans;
    ans
}

fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let n = positions.len();
    let mut sol = Solution {
        pos: positions.iter().map(|p| Position { x: p[0], y: p[1] }).collect(),
        dist: vec![vec![0; n]; n + 1],
        n: n,
    };
    sol.pos.push(Position { x: kx, y: ky });

    calculate_distances(&mut sol);

    let mut memo = vec![vec![vec![-1; 2]; 1 << n]; n + 1];
    dfs(&sol, n, 0, 0, &mut memo)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty());

    let first_line = lines.next().unwrap();
    let kx_ky: Vec<i32> = first_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let kx = kx_ky[0];
    let ky = kx_ky[1];

    let positions_size: usize = lines.next().unwrap().parse().unwrap();
    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        let line = lines.next().unwrap();
        let coords: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        positions.push(vec![coords[0], coords[1]]);
    }

    let result = max_moves(kx, ky, positions);
    println!("{}", result);
}