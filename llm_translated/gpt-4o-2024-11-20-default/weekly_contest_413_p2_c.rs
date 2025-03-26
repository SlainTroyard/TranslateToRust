use std::io::{self, BufRead};
use std::cmp::Ordering;

fn results_array(queries: &Vec<[i32; 2]>, k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut distance_arr = Vec::with_capacity(k);

    for (i, query) in queries.iter().enumerate() {
        let distance = query[0].abs() + query[1].abs();

        // Insert this distance in the appropriate place in a sorted manner
        let mut j = distance_arr.len();
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k {
                if j == distance_arr.len() {
                    distance_arr.push(0);
                }
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }

        if j < k {
            if j == distance_arr.len() {
                distance_arr.push(0);
            }
            distance_arr[j] = distance;
        }

        // If enough distances are accumulated, set `k-th largest` distance
        if distance_arr.len() == k {
            result[i] = distance_arr[k - 1];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read `queriesSize` and `k`
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let queries_size: usize = first_line_split.next().unwrap().parse().unwrap();
    let k: usize = first_line_split.next().unwrap().parse().unwrap();

    // Read `queries`
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        let x: i32 = split.next().unwrap().parse().unwrap();
        let y: i32 = split.next().unwrap().parse().unwrap();
        queries.push([x, y]);
    }

    // Compute the result array
    let result = results_array(&queries, k);

    // Print the result in the exact output format
    for (i, res) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res);
    }
    println!();
}