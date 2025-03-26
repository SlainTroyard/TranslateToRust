use std::io;

fn maximum_subarray_xor(nums: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut subarray_xors = vec![vec![0; nums_size]; nums_size];
    let mut max_scores = vec![vec![0; nums_size]; nums_size];

    // Initialize diagonals (as per C code's redundant loops)
    for row in 0..nums_size {
        subarray_xors[row][row] = nums[row];
    }
    for row in 0..nums_size {
        subarray_xors[row][row] = nums[row];
    }

    for row in 0..nums_size {
        max_scores[row][row] = nums[row];
    }
    for row in 0..nums_size {
        max_scores[row][row] = nums[row];
    }

    // Compute subarray_xors for subLength >= 2
    for sub_length in 2..=nums_size {
        let mut i = 0;
        let mut j = sub_length - 1;
        while j < nums_size {
            subarray_xors[i][j] = subarray_xors[i][j - 1] ^ subarray_xors[i + 1][j];
            i += 1;
            j += 1;
        }
    }

    // Compute max_scores for subLength >= 2
    for sub_length in 2..=nums_size {
        let mut i = 0;
        let mut j = sub_length - 1;
        while j < nums_size {
            let current = subarray_xors[i][j];
            let left = max_scores[i][j - 1];
            let right = max_scores[i + 1][j];
            max_scores[i][j] = current.max(left.max(right));
            i += 1;
            j += 1;
        }
    }

    // Generate answers
    let mut answer = Vec::with_capacity(queries.len());
    for query in queries {
        let start = query[0] as usize;
        let end = query[1] as usize;
        answer.push(max_scores[start][end]);
    }
    answer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.split_whitespace();

    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .by_ref()
        .take(nums_size)
        .map(|s| s.parse().unwrap())
        .collect();

    let queries_size: usize = lines.next().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let a: i32 = lines.next().unwrap().parse().unwrap();
        let b: i32 = lines.next().unwrap().parse().unwrap();
        queries.push([a, b]);
    }

    let answers = maximum_subarray_xor(&nums, &queries);
    for ans in answers {
        print!("{} ", ans);
    }
    println!();
}