use std::io;

struct Solution;

impl Solution {
    pub fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = if n == 0 { 0 } else { grid[0].len() };
        let mut cnt = 0;
        let mut vec = Vec::new();
        for i in 0..n {
            if i % 2 == 0 {
                // Even row: left to right
                for j in 0..m {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            } else {
                // Odd row: right to left
                for j in (0..m).rev() {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            }
        }
        vec
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = numbers[0] as usize;
    let m = numbers[1] as usize;
    let total = n * m;
    assert_eq!(numbers.len(), 2 + total);

    let grid = numbers[2..]
        .chunks(m)
        .map(|row| row.to_vec())
        .collect::<Vec<Vec<i32>>>();

    let solution = Solution;
    let ans = solution.zigzag_traversal(&grid);

    for num in ans {
        print!("{} ", num);
    }
    println!();
}