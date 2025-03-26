use std::io::{self, BufRead};

fn results_array(queries: &[Vec<i32>], k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let k_usize = k as usize;
    let mut result = vec![-1; queries_size];
    // Allocate distance array with size k+1, initialized to 0
    let mut distance_arr = vec![0; k_usize + 1];
    let mut distance_size = 0;

    for (i, query) in queries.iter().enumerate() {
        let x = query[0];
        let y = query[1];
        let distance = x.abs() + y.abs();

        let mut j = distance_size;
        // Shift elements greater than the new distance to the right
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k_usize {
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }

        // Insert the new distance if within the k limit
        if j < k_usize {
            distance_arr[j] = distance;
            // Increase the size only if not exceeding k
            if distance_size < k_usize {
                distance_size += 1;
            }
        }

        // Update result if we have at least k elements
        if distance_size == k_usize {
            result[i] = distance_arr[k_usize - 1];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();

    // Read first line for queries_size and k
    stdin_lock.read_line(&mut line).unwrap();
    let mut parts = line.split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    line.clear();

    // Read each query's coordinates
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        line.clear();
        stdin_lock.read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    // Compute results and print
    let result = results_array(&queries, k);
    for num in result {
        print!("{} ", num);
    }
    println!();
}