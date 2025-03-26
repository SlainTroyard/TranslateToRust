use std::io::{self, BufRead, Write};

fn maximum_subarray_xor(nums: &Vec<i32>, queries: &Vec<(usize, usize)>) -> Vec<i32> {
    let nums_size = nums.len();
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    
    // Initialize the subarray xors for single elements
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }
    
    // Compute the XOR for all subarrays
    for sub_length in 2..=nums_size {
        for i in 0..nums_size - sub_length + 1 {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }
    
    let mut max_scores = vec![vec![0; nums_size]; nums_size];
    
    // Initialize the max scores for single elements
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }
    
    // Compute the maximum score for all subarrays
    for sub_length in 2..=nums_size {
        for i in 0..nums_size - sub_length + 1 {
            let j = i + sub_length - 1;
            max_scores[i][j] = subarray_xors[i][j].max(max_scores[i][j - 1].max(max_scores[i + 1][j]));
        }
    }
    
    // Prepare the answers for the queries
    let mut answer = Vec::with_capacity(queries.len());
    for (start, end) in queries {
        answer.push(max_scores[*start][*end]);
    }
    
    answer
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read the size of the nums array
    stdin_lock.read_line(&mut buffer).unwrap();
    let nums_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the nums array
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().unwrap());
    }

    // Read the number of queries
    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let queries_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        let query: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push((query[0], query[1]));
    }

    // Get the results for the queries
    let answer = maximum_subarray_xor(&nums, &queries);

    // Print the results
    for (i, &result) in answer.iter().enumerate() {
        if i > 0 {
            write!(&mut stdout, " ").unwrap();
        }
        write!(&mut stdout, "{}", result).unwrap();
    }
    writeln!(&mut stdout).unwrap();
}