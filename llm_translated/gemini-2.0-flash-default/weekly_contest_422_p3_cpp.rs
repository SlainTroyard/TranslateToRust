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
            let top = pq.pop().unwrap().0;
            time = top.0;
            let r = top.1.0;
            let c = top.1.1;

            if r == (rows as int) - 1 && c == (cols as int) - 1 {
                return time;
            }
            for i in 0..4 {
                let nrow = r + drow[i];
                let ncol = c + dcol[i];
                if nrow >= 0 && nrow < (rows as int) && ncol >= 0 && ncol < (cols as int) && vis[nrow as usize][ncol as usize] == 0 {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let nrow_usize = nrow as usize;
                    let ncol_usize = ncol as usize;

                    if move_time[nrow_usize][ncol_usize] >= time {
                        pq.push(Reverse((move_time[nrow_usize][ncol_usize] + next_cost, (nrow, ncol))));
                    } else {
                        pq.push(Reverse((time + next_cost, (nrow, ncol))));
                    }
                    vis[nrow_usize][ncol_usize] = 1;
                }
            }
        }
        return time;
    }
}

fn main() -> io::Result<()> {
    let mut sol = Solution {};

    // 读取矩阵行列数
    let mut rows_str = String::new();
    io::stdin().read_line(&mut rows_str)?;
    let mut rows_cols = rows_str.split_whitespace();

    let rows: usize = rows_cols.next().unwrap().parse().unwrap();
    let cols: usize = rows_cols.next().unwrap().parse().unwrap();

    // 读取矩阵元素
    let mut move_time: Vec<Vec<int>> = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let mut nums = line.split_whitespace();
        for j in 0..cols {
            move_time[i][j] = nums.next().unwrap().parse().unwrap();
        }
    }

    // 计算结果
    let result = sol.min_time_to_reach(move_time);
    println!("{}", result);

    Ok(())
}