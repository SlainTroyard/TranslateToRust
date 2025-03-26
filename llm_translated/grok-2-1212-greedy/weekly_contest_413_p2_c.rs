use std::io::{self, BufRead};

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut distance_arr = Vec::with_capacity((k + 1) as usize);
    let mut distance_size = 0;

    for (i, query) in queries.iter().enumerate() {
        let distance = query[0].abs() + query[1].abs();

        let mut j = distance_size;
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k as usize {
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }
        if j < k as usize {
            distance_arr[j] = distance;
            if distance_size < k as usize {
                distance_size += 1;
            }
        }

        if distance_size == k as usize {
            result[i] = distance_arr[k as usize - 1];
        }
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read queriesSize and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    // Process queries and get result
    let result = results_array(&queries, k);

    // Print result
    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}