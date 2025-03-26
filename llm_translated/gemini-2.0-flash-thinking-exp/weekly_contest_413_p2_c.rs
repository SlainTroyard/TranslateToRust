use std::io;

fn results_array(queries: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    let mut distance_arr = vec![0; k as usize]; // Use size k as it's sufficient based on logic
    let mut distance_size = 0;

    for i in 0..queries_size {
        let distance = queries[i][0].abs() + queries[i][1].abs();

        let mut j = distance_size;
        while j > 0 && distance_arr[(j - 1) as usize] < distance {
            if j < k as usize {
                distance_arr[j as usize] = distance_arr[(j - 1) as usize];
            }
            j -= 1;
        }
        if j < k as usize {
            distance_arr[j as usize] = distance;
            if distance_size < k as usize {
                distance_size += 1;
            }
        }

        if distance_size == k as usize {
            result[i] = distance_arr[(k - 1) as usize];
        }
    }
    result
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let mut iter = input_line.trim().split_whitespace();
    let queries_size: usize = iter.next().unwrap().parse().expect("Failed to parse queriesSize");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut query_line = String::new();
        io::stdin().read_line(&mut query_line).expect("Failed to read line");
        let mut query_iter = query_line.trim().split_whitespace();
        let qx: i32 = query_iter.next().unwrap().parse().expect("Failed to parse query x");
        let qy: i32 = query_iter.next().unwrap().parse().expect("Failed to parse query y");
        queries.push(vec![qx, qy]);
    }

    let result = results_array(&queries, k);

    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
}