use std::io;

fn results_array(queries: &[(i32, i32)], k: usize) -> Vec<i32> {
    let queries_size = queries.len();
    let mut result = vec![-1; queries_size];
    if k == 0 {
        return result;
    }

    let mut distance_vec = vec![0; k];
    let mut distance_size = 0;

    for (i, &(x, y)) in queries.iter().enumerate() {
        let distance = x.abs() + y.abs();
        let mut j = distance_size;
        while j > 0 && distance_vec[j - 1] < distance {
            distance_vec[j] = distance_vec[j - 1];
            j -= 1;
        }
        if j < k {
            distance_vec[j] = distance;
            if distance_size < k {
                distance_size += 1;
            }
        }
        if distance_size == k {
            result[i] = distance_vec[k - 1];
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input line");
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        panic!("Invalid input format");
    }

    let queries_size: usize = parts[0].parse().expect("Invalid integer for queries size");
    let k: usize = parts[1].parse().expect("Invalid integer for k");

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read query line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid integer in query"))
            .collect();
        if nums.len() != 2 {
            panic!("Invalid query format");
        }
        queries.push((nums[0], nums[1]));
    }

    let result = results_array(&queries, k);
    for &num in &result {
        print!("{} ", num);
    }
    println!();
}