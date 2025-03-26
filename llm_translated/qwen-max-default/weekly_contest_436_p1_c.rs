use std::io::{self, BufRead};

fn cmp_asc(a: &i32, b: &i32) -> i32 {
    b.cmp(a).into()
}

fn cmp_desc(a: &i32, b: &i32) -> i32 {
    a.cmp(b).into()
}

fn sort_matrix(grid: &mut Vec<Vec<i32>>, grid_size: usize) -> (Vec<usize>, Vec<Vec<i32>>) {
    let mut return_column_sizes = vec![grid_size; grid_size];
    for i in 0..grid_size {
        let len = grid_size - i;
        let mut a = Vec::with_capacity(len);
        for k in i..grid_size {
            a.push(grid[k][k - i]);
        }
        a.sort_unstable_by(|a, b| cmp_desc(a, b));
        for k in i..grid_size {
            grid[k][k - i] = a[k - i];
        }
    }
    for i in 1..grid_size {
        let len = grid_size - i;
        let mut a = Vec::with_capacity(len);
        for k in 0..grid_size - i {
            a.push(grid[k][k + i]);
        }
        a.sort_unstable_by(|a, b| cmp_asc(a, b));
        for k in 0..grid_size - i {
            grid[k][k + i] = a[k];
        }
    }
    (return_column_sizes, grid.clone())
}

fn main() {
    // Read the size of the grid
    let stdin = io::stdin();
    let mut n = String::new();
    stdin.lock().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Initialize the grid
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Failed to read line");
        for (j, val) in line.trim().split_whitespace().enumerate() {
            grid[i][j] = val.parse().expect("Please type a number!");
        }
    }

    // Sort the matrix
    let (return_column_sizes, result) = sort_matrix(&mut grid, n);

    // Print the result
    for row in result {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}