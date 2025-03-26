use std::io;
use std::collections::BinaryHeap;

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    if rows == 0 {
        return 0;
    }
    let cols = move_time[0].len();
    if cols == 0 {
        return 0;
    }

    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();

    heap.push((0, 0, 0));
    visited[0][0] = true;

    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    while let Some((time_neg, r, c)) = heap.pop() {
        let time = -time_neg;
        let r = r as usize;
        let c = c as usize;

        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        for i in 0..4 {
            let nrow = r as i32 + drow[i];
            let ncol = c as i32 + dcol[i];

            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                let nrow_usize = nrow as usize;
                let ncol_usize = ncol as usize;

                if !visited[nrow_usize][ncol_usize] {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let current_time = time;
                    let cell_move_time = move_time[nrow_usize][ncol_usize];
                    let new_time = if cell_move_time >= current_time {
                        cell_move_time + next_cost
                    } else {
                        current_time + next_cost
                    };

                    heap.push((-new_time, nrow, ncol));
                    visited[nrow_usize][ncol_usize] = true;
                }
            }
        }
    }

    0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let first_line = lines.next().unwrap();
    let mut dims = first_line.split_whitespace();
    let rows = dims.next().unwrap().parse::<i32>().unwrap();
    let cols = dims.next().unwrap().parse::<i32>().unwrap();

    let mut move_time = Vec::with_capacity(rows as usize);
    for _ in 0..rows {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let row = nums.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        move_time.push(row);
    }

    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}