use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

/// Performs an upper_bound search on a sorted slice.
/// Finds the first index where the element is greater than `val`.
fn upper_bound(v: &[i64], val: i64) -> usize {
    let mut low = 0;
    let mut high = v.len();
    while low < high {
        let mid = (low + high) / 2;
        // If the mid element is less than or equal the value,
        // then the answer must be on the right side.
        if v[mid] <= val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Computes the gcdValues similar to the C++ solution.
/// Given `nums` and `queries`, for each query it finds
/// the smallest gcd value whose cumulative count is greater than the query.
fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<usize> {
    // Find the maximum value in nums.
    let mx = nums
        .iter()
        .copied()
        .max()
        .expect("There should be at least one number in nums") as usize;

    // cnt_x[x] will store how many times the number x appears.
    let mut cnt_x = vec![0; mx + 1];
    for &x in nums.iter() {
        cnt_x[x as usize] += 1;
    }

    // cnt_gcd will store the number of pairs with GCD equal to i.
    let mut cnt_gcd = vec![0i64; mx + 1];
    // Iterate backwards from mx down to 1.
    for i in (1..=mx).rev() {
        let mut c = 0;
        // Sum up all counts for numbers which are multiples of i.
        // Also, subtract counts already computed for higher gcd values.
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        // Add the new pairs count where the GCD is exactly i.
        cnt_gcd[i] += c * (c - 1) / 2;
    }

    // Compute the partial sum (cumulative count) of the cnt_gcd array.
    // This is a standard partial_sum over the array.
    for i in 1..cnt_gcd.len() {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Process each query by finding the first index in cnt_gcd
    // such that the value is greater than the query value.
    let mut ans = Vec::with_capacity(queries.len());
    for &q in queries.iter() {
        ans.push(upper_bound(&cnt_gcd, q));
    }
    ans
}

/// Reads input from standard input and returns a iterator of tokens split by whitespace.
fn read_tokens() -> io::Result<Vec<String>> {
    // Read all input into a string.
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    // Split the input by whitespace and collect tokens.
    Ok(input.split_whitespace().map(|s| s.to_string()).collect())
}

fn main() -> io::Result<()> {
    // Get all tokens from the input.
    let tokens = read_tokens()?;
    let mut iter = tokens.into_iter();

    // Parse first integer: n (number of elements in nums).
    let n: usize = iter
        .next()
        .expect("Expected n")
        .parse()
        .expect("n should be an integer");

    // Parse n integers to build nums.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter
            .next()
            .expect("Expected a number in nums")
            .parse()
            .expect("nums should contain integers");
        nums.push(num);
    }

    // Parse the next integer: q (number of queries).
    let q: usize = iter
        .next()
        .expect("Expected q")
        .parse()
        .expect("q should be an integer");

    // Parse q integers to build queries.
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let query: i64 = iter
            .next()
            .expect("Expected a query number")
            .parse()
            .expect("queries should contain integers");
        queries.push(query);
    }

    // Compute the answer using the gcd_values function.
    let ans = gcd_values(nums, queries);

    // Write the result to standard output.
    // The output is a single line with numbers separated by a space.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for x in ans {
        write!(out, "{} ", x)?;
    }
    writeln!(out)?;
    Ok(())
}