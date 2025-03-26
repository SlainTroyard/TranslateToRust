use std::io::{self, BufRead};

fn results_array(queries: &Vec<(i32, i32)>, k: usize) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut distance_arr = vec![0; k + 1];
    let mut distance_size = 0;

    for (i, &(x, y)) in queries.iter().enumerate() {
        let distance = x.abs() + y.abs();

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

    // Read the number of queries and k
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push((x, y));
    }

    // Get the result
    let result = results_array(&queries, k);

    // Print the result
    for value in result {
        print!("{} ", value);
    }
}