use std::io;

fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<(usize, usize)>) -> Vec<i32> {
    let n = nums.len();
    let mut subarray_xors = vec![vec![0; n]; n];
    let mut max_scores = vec![vec![0; n]; n];
    
    // Initialize subarray_xors and max_scores for single elements
    for i in 0..n {
        subarray_xors[i][i] = nums[i];
        max_scores[i][i] = nums[i];
    }
    
    // Fill subarray_xors for subarrays of length >= 2
    for sub_length in 2..=n {
        for i in 0..=n - sub_length {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j-1] ^ subarray_xors[i+1][j];
        }
    }
    
    // Fill max_scores for subarrays of length >= 2
    for sub_length in 2..=n {
        for i in 0..=n - sub_length {
            let j = i + sub_length - 1;
            max_scores[i][j] = std::cmp::max(
                subarray_xors[i][j],
                std::cmp::max(max_scores[i][j-1], max_scores[i+1][j])
            );
        }
    }
    
    // Process queries
    queries.iter().map(|(l, r)| max_scores[*l][*r]).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    let nums_size: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens
        .take(nums_size)
        .map(|x| x.parse().unwrap())
        .collect();
    
    let queries_size: usize = tokens.next().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let l: usize = tokens.next().unwrap().parse().unwrap();
        let r: usize = tokens.next().unwrap().parse().unwrap();
        queries.push((l, r));
    }
    
    let answer = maximum_subarray_xor(nums, queries);
    println!("{}", answer.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}