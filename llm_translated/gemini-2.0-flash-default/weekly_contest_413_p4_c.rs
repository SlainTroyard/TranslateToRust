use std::cmp::max;
use std::io;

fn maximum_subarray_xor(
    nums: &[i32],
    queries: &[[i32; 2]],
) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();

    // subarrayXors[i][j] stores the XOR of subarray from index i to j
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }
    for sub_length in 2..=nums_size {
        for i in 0..(nums_size - sub_length + 1) {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // maxScores[i][j] stores the maximum XOR value of subarray from index i to j
    let mut max_scores = vec![vec![0; nums_size]; nums_size];
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }
    for sub_length in 2..=nums_size {
        for i in 0..(nums_size - sub_length + 1) {
            let j = i + sub_length - 1;
            max_scores[i][j] = max(
                subarray_xors[i][j],
                max(max_scores[i][j - 1], max_scores[i + 1][j]),
            );
        }
    }

    let mut answer = Vec::with_capacity(queries_size);
    for i in 0..queries_size {
        let start = queries[i][0] as usize;
        let end = queries[i][1] as usize;
        answer.push(max_scores[start][end]);
    }

    answer
}

fn main() -> io::Result<()> {
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str)?;
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str)?;
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut queries_size_str = String::new();
    io::stdin().read_line(&mut queries_size_str)?;
    let queries_size: usize = queries_size_str.trim().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str)?;
        let query: Vec<i32> = query_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push([query[0], query[1]]);
    }

    let answer = maximum_subarray_xor(&nums, &queries);

    for &val in &answer {
        print!("{} ", val);
    }
    println!();

    Ok(())
}