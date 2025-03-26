use std::io::{self, Read};

/// A structure representing the solution.
struct Solution;

impl Solution {
    /// Computes the maximum XOR in any subarray for given queries.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to the vector of integers. This vector gets updated in-place.
    /// * `queries` - A slice of queries, where each query is a vector of two usize values representing start and end indices.
    ///
    /// # Returns
    ///
    /// Returns a vector of integers, where each element is the answer to the corresponding query.
    ///
    /// # Details
    ///
    /// This function replicates the C++ logic:
    /// For each starting index i (in reverse order), it computes cumulative XOR values from i to j.
    /// The maximum among the current cumulative XOR, the maximum from a lower start index, and the maximum from a previous j index is stored.
    fn maximum_subarray_xor(f: &mut Vec<i32>, queries: &Vec<Vec<usize>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];

        // Iterate over starting indices in reverse order (from last index to first)
        for i in (0..n).rev() {
            // The subarray containing only the element f[i]
            mx[i][i] = f[i];
            // Iterate j from i+1 to n-1 to update cumulative XOR and track the maximum
            for j in i + 1..n {
                // Update the cumulative XOR value at index j
                f[j] ^= f[j - 1];
                // Calculate maximum among three candidates:
                // 1. The cumulative XOR of subarray from i to j (f[j])
                // 2. The maximum from subarray starting at i+1 for index j (mx[i+1][j])
                // 3. The maximum for subarray ending at j-1 starting from i (mx[i][j-1])
                let candidate1 = f[j];
                let candidate2 = mx[i + 1][j];
                let candidate3 = mx[i][j - 1];
                mx[i][j] = candidate1.max(candidate2).max(candidate3);
            }
        }

        // Answer the queries by accessing the precomputed maximums from the mx table.
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            // The query indices are assumed valid.
            ans.push(mx[q[0]][q[1]]);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Read entire input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input by whitespace into an iterator over tokens
    let mut tokens = input.split_whitespace();

    // Read the number of elements in the nums array
    let nums_size: usize = tokens
        .next()
        .expect("Expected a number for numsSize")
        .parse()
        .expect("Failed to parse numsSize");

    // Read the nums array elements
    let mut nums: Vec<i32> = (0..nums_size)
        .map(|_| {
            tokens
                .next()
                .expect("Expected a number for an element in nums")
                .parse()
                .expect("Failed to parse an integer in nums")
        })
        .collect();

    // Read the number of queries
    let queries_size: usize = tokens
        .next()
        .expect("Expected a number for queriesSize")
        .parse()
        .expect("Failed to parse queriesSize");

    // Read each query: each query consists of two numbers
    let queries: Vec<Vec<usize>> = (0..queries_size)
        .map(|_| {
            let start: usize = tokens
                .next()
                .expect("Expected a number for a query start index")
                .parse()
                .expect("Failed to parse query start index");
            let end: usize = tokens
                .next()
                .expect("Expected a number for a query end index")
                .parse()
                .expect("Failed to parse query end index");
            vec![start, end]
        })
        .collect();

    // Compute the result using the provided algorithm
    let result = Solution::maximum_subarray_xor(&mut nums, &queries);

    // Output the results exactly as in the C++ program (space-separated integers)
    for num in result {
        print!("{} ", num);
    }
    Ok(())
}