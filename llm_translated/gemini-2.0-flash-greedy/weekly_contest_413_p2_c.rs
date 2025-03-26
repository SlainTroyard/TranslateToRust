use std::io;
use std::io::Read;
use std::cmp::max;

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    let mut distance_arr = vec![0; (k + 1) as usize];
    let mut distance_size = 0;

    for i in 0..queries_size {
        let distance = queries[i][0].abs() + queries[i][1].abs();

        let mut j = distance_size;
        while j > 0 && distance_arr[(j - 1) as usize] < distance {
            if j < k {
                distance_arr[j as usize] = distance_arr[(j - 1) as usize];
            }
            j -= 1;
        }
        if j < k {
            distance_arr[j as usize] = distance;
            if distance_size < k {
                distance_size += 1;
            }
        }

        if distance_size == k {
            result[i] = distance_arr[(k - 1) as usize];
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let queries_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_line_iter.next().unwrap().parse().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..queries_size {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        queries.push(vec![x, y]);
    }

    let result = results_array(&queries, k);

    for &val in &result {
        print!("{} ", val);
    }
    println!();
}