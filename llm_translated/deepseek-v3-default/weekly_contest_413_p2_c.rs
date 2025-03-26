use std::io::{self, Write};

fn results_array(queries: &[Vec<i32>], k: usize) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    let mut distance_arr = vec![0; k + 1];
    let mut distance_size = 0;

    for (i, query) in queries.iter().enumerate() {
        let distance = query[0].abs() + query[1].abs();

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    let result = results_array(&queries, k);

    for res in result {
        print!("{} ", res);
    }
    println!();
}