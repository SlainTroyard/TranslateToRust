use std::error::Error;
use std::io;

/// Translates the C function "resultsArray" to Rust.
/// This function processes queries of (x, y) coordinates, computing the Manhattan distance of each.
/// It keeps track of the top k largest distances so far in a descending order array.
/// Once k distances have been tracked, it outputs the k-th largest distance at each step.
/// If fewer than k distances are available, it outputs -1.
fn results_array(queries: &[[i32; 2]], k: usize) -> Vec<i32> {
    let qsize = queries.len();
    // Prepare the result vector, initialized to -1.
    let mut result = vec![-1; qsize];
    // This will store the top k distances so far, in descending order.
    // The array is k+1 in size to allow room for shifting elements.
    let mut distance_arr = vec![0; k + 1];
    let mut distance_size = 0;

    for (i, &coords) in queries.iter().enumerate() {
        // Compute Manhattan distance.
        let distance = coords[0].abs() + coords[1].abs();

        // Insert the new distance into the correct position in distance_arr, descending order.
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

        // If we have k distances, record the k-th largest in result.
        if distance_size == k {
            result[i] = distance_arr[k - 1];
        }
    }

    result
}

/// Main function reproducing the C-style I/O format exactly:
/// 1) Reads "queriesSize" and "k" from stdin
/// 2) Reads each query (two integers) from stdin
/// 3) Processes them with "results_array"
/// 4) Prints each result followed by a space
fn main() -> Result<(), Box<dyn Error>> {
    // Read the first line to get queriesSize and k
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let queries_size = parts
        .next()
        .ok_or("Missing queriesSize")?
        .parse::<usize>()?;
    let k = parts.next().ok_or("Missing k")?.parse::<usize>()?;

    // Read "queriesSize" lines, each containing two integers
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let mut parts = line.split_whitespace();
        let x = parts.next().ok_or("Missing x")?.parse::<i32>()?;
        let y = parts.next().ok_or("Missing y")?.parse::<i32>()?;
        queries.push([x, y]);
    }

    // Process queries and get results
    let result = results_array(&queries, k);

    // Print results (C code prints each result followed by a space, no newline at the end)
    for val in result {
        print!("{} ", val);
    }

    Ok(())
}