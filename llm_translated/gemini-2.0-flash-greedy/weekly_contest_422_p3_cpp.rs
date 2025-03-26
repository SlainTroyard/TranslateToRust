use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

struct Solution {}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<int>>) -> int {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut vis = vec![vec![0; cols]; rows];
        let mut pq: BinaryHeap<Reverse<(int, (int, int))>> = BinaryHeap::new();

        pq.push(Reverse((0, (0, 0))));
        let drow: [int; 4] = [-1, 0, 1, 0];
        let dcol: [int; 4] = [0, 1, 0, -1];
        let mut time = 0;
        vis[0][0] = 1;

        while !pq.is_empty() {
            let Reverse((current_time, (r, c))) = pq.pop().unwrap();
            time = current_time;

            if r == (rows - 1) as int && c == (cols - 1) as int {
                return time;
            }

            for i in 0..4 {
                let nrow = r + drow[i];
                let ncol = c + dcol[i];

                if nrow >= 0
                    && nrow < rows as int
                    && ncol >= 0
                    && ncol < cols as int
                    && vis[nrow as usize][ncol as usize] == 0
                {
                    let next_cost = 2 - ((nrow + ncol) % 2);
                    let nrow_usize = nrow as usize;
                    let ncol_usize = ncol as usize;

                    let next_time = if move_time[nrow_usize][ncol_usize] >= time {
                        move_time[nrow_usize][ncol_usize] + next_cost
                    } else {
                        time + next_cost
                    };

                    pq.push(Reverse((next_time, (nrow, ncol))));
                    vis[nrow_usize][ncol_usize] = 1;
                }
            }
        }
        time
    }
}

fn main() {
    let sol = Solution {};

    // 读取矩阵行列数
    let mut rows_str = String::new();
    io::stdin().read_line(&mut rows_str).unwrap();
    let mut rows_cols = rows_str.split_whitespace();
    let rows: usize = rows_cols.next().unwrap().parse().unwrap();
    let cols: usize = rows_cols.next().unwrap().parse().unwrap();

    // 读取矩阵元素
    let mut move_time: Vec<Vec<int>> = Vec::new();
    for _ in 0..rows {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let row: Vec<int> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        move_time.push(row);
    }

    // 计算结果
    let result = sol.min_time_to_reach(move_time);
    println!("{}", result);
}