use std::io;

fn results_array(queries: &Vec<Vec<isize>>, k: isize) -> Vec<isize> {
    let mut result = vec![-1; queries.len()];
    let mut distance_arr: Vec<isize> = Vec::with_capacity(k as usize);
    
    for (i, query) in queries.iter().enumerate() {
        let distance = (query[0].abs() + query[1].abs()) as isize;
        
        let mut j = distance_arr.len();
        while j > 0 && distance_arr[j - 1] < distance {
            if j < k as usize {
                distance_arr[j] = distance_arr[j - 1];
            }
            j -= 1;
        }
        if j < k as usize {
            if distance_arr.len() < k as usize {
                distance_arr.push(distance);
            } else {
                distance_arr[j] = distance;
            }
        }
        
        if distance_arr.len() == k as usize {
            result[i] = distance_arr[k as usize - 1];
        }
    }
    
    result
}

fn main() {
    // Read the number of queries and k from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<isize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    let (queries_size, k) = (parts[0] as usize, parts[1]);

    // Initialize the queries vector
    let mut queries: Vec<Vec<isize>> = Vec::with_capacity(queries_size);

    // Read each query
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let query: Vec<isize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        queries.push(query);
    }

    // Get the result array
    let result = results_array(&queries, k);

    // Print the result
    for value in result {
        print!("{} ", value);
    }
}