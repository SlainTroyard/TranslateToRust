use std::io::{self, Read};

fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();
    
    // Process diagonals starting from the first column (sorted descending)
    for i in 0..n {
        let mut diagonal = Vec::new();
        let (mut row, mut col) = (i, 0);
        while row < n && col < n {
            diagonal.push(grid[row][col]);
            row += 1;
            col += 1;
        }
        diagonal.sort_by(|a, b| b.cmp(a));
        let mut idx = 0;
        let (mut row, mut col) = (i, 0);
        while row < n && col < n {
            grid[row][col] = diagonal[idx];
            idx += 1;
            row += 1;
            col += 1;
        }
    }
    
    // Process diagonals starting from the top row (sorted ascending)
    for i in 1..n {
        let mut diagonal = Vec::new();
        let (mut row, mut col) = (0, i);
        while row < n && col < n {
            diagonal.push(grid[row][col]);
            row += 1;
            col += 1;
        }
        diagonal.sort();
        let mut idx = 0;
        let (mut row, mut col) = (0, i);
        while row < n && col < n {
            grid[row][col] = diagonal[idx];
            idx += 1;
            row += 1;
            col += 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    
    let n = tokens.next().unwrap() as usize;
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = tokens.next().unwrap();
        }
    }
    
    sort_matrix(&mut grid);
    
    for row in grid {
        for num in &row {
            print!("{} ", num);
        }
        println!();
    }
}