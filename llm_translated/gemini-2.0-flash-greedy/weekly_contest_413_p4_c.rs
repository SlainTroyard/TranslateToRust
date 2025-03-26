use std::cmp::max;
use std::io;
use std::io::Read;

fn maximum_subarray_xor(
    nums: &[i32],
    queries: &[[i32; 2]],
) -> Vec<i32> {
    let nums_size = nums.len();
    let queries_size = queries.len();

    // subarrayXors[i][j] stores the XOR of subarray nums[i..=j]
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];

    // Initialize subarrayXors for subarrays of length 1
    for i in 0..nums_size {
        subarray_xors[i][i] = nums[i];
    }

    // Calculate XORs for subarrays of length 2 and greater
    for sub_length in 2..=nums_size {
        for i in 0..=nums_size - sub_length {
            let j = i + sub_length - 1;
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
        }
    }

    // maxScores[i][j] stores the maximum XOR value of any subarray within nums[i..=j]
    let mut max_scores = vec![vec![0; nums_size]; nums_size];

    // Initialize maxScores for subarrays of length 1
    for i in 0..nums_size {
        max_scores[i][i] = nums[i];
    }

    // Calculate maximum XOR scores for subarrays of length 2 and greater
    for sub_length in 2..=nums_size {
        for i in 0..=nums_size - sub_length {
            let j = i + sub_length - 1;
            max_scores[i][j] = max(
                subarray_xors[i][j],
                max(max_scores[i][j - 1], max_scores[i + 1][j]),
            );
        }
    }

    // Retrieve the answers for each query
    let mut answer = Vec::with_capacity(queries_size);
    for query in queries {
        answer.push(max_scores[query[0] as usize][query[1] as usize]);
    }

    answer
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let queries_size: usize = lines.next().unwrap().parse().unwrap();
    let mut queries: Vec<[i32; 2]> = Vec::with_capacity(queries_size);

    for _ in 0..queries_size {
        let query: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push([query[0], query[1]]);
    }

    let answer = maximum_subarray_xor(&nums, &queries);

    for val in answer {
        print!("{} ", val);
    }
    println!();

    Ok(())
}