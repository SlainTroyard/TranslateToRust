use std::io;

fn process_matrix(matrix: &mut Vec<Vec<i32>>, n: usize) {
    // Process diagonals starting from the left edge (row i, column 0) in descending order
    for i in 0..n {
        let mut elements = Vec::new();
        let (mut k, mut m) = (i, 0);
        while k < n && m < n {
            elements.push(matrix[k][m]);
            k += 1;
            m += 1;
        }
        // Sort in descending order using a custom comparator
        elements.sort_unstable_by(|a, b| b.cmp(a));
        // Write back the sorted elements
        let (mut k, mut m) = (i, 0);
        for &val in &elements {
            matrix[k][m] = val;
            k += 1;
            m += 1;
        }
    }

    // Process diagonals starting from the top edge (row 0, column i) for i from 1 onwards, sorted ascending
    for i in 1..n {
        let mut elements = Vec::new();
        let (mut k, mut m) = (0, i);
        while k < n && m < n {
            elements.push(matrix[k][m]);
            k += 1;
            m += 1;
        }
        // Sort in ascending order
        elements.sort_unstable();
        // Write back the sorted elements
        let (mut k, mut m) = (0, i);
        for &val in &elements {
            matrix[k][m] = val;
            k += 1;
            m += 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().expect("No value for N").parse().expect("Invalid N");
    let mut grid = vec![vec![0; n]; n];

    for row in &mut grid {
        for num in row {
            *num = iter.next().expect("Not enough numbers").parse().expect("Invalid number");
        }
    }

    process_matrix(&mut grid, n);

    // Output the matrix in the required format
    for row in &grid {
        for &num in row {
            print!("{} ", num);
        }
        println!();
    }
}