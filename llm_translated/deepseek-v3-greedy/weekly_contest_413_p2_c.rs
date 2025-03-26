use std::io::{self, BufRead};

fn results_array(queries: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    let mut distance_arr = vec![0; k + 1];
    let mut distance_size = 0;

    for i in 0..queries_size {
        let distance = queries[i][0].abs() + queries[i][1].abs();

        let mut j = distance_size;
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k {
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }
        if j < k {
            distance_arr[j] = distance;
            if distance_size < k {
                distance_size += 1;
            }
        }

        if distance_size == k {
            result[i] = distance_arr[k - 1];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for queriesSize and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    // Compute the results
    let result = results_array(&queries, k);

    // Print the results
    for res in result {
        print!("{} ", res);
    }
    println!();
}