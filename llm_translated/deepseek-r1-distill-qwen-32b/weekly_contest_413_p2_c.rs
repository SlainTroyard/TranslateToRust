// Problem: Weekly Contest 413 Problem 2
use std::io;

fn process_queries(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut distances = Vec::with_capacity(k as usize);
    let k = k as usize;

    for (i, query) in queries.iter().enumerate() {
        let distance = query[0].abs() + query[1].abs();

        let mut j = distances.len();
        while j > 0 && distances[j - 1] < distance {
            if j < k {
                distances[j] = distances[j - 1];
            }
            j -= 1;
        }

        if j < k {
            if j < distances.len() {
                distances[j] = distance;
            } else {
                distances.push(distance);
            }
            if distances.len() < k {
                // Ensure the vector doesn't exceed capacity
                while distances.len() < k {
                    distances.push(0);
                }
            }
        }

        if distances.len() == k {
            result[i] = distances[k - 1];
        }
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut parts = input.trim().split_whitespace();
    let queries_size: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let mut nums = line.trim().split_whitespace();
        let x: i32 = nums.next().unwrap().parse().unwrap();
        let y: i32 = nums.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    let result = process_queries(queries, k);
    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(())
}