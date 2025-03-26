use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // Get standard input and output handles
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // Read numsSize (number of elements in nums)
    let nums_size = match next_token(&mut reader)?.parse::<usize>() {
        Ok(v) => v,
        Err(e) => {
            writeln!(io::stderr(), "Failed to parse numsSize: {:?}", e)?;
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid numsSize"));
        }
    };

    // Read the nums vector
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let token = next_token(&mut reader)?;
        let num = token.parse::<i32>().expect("Invalid number in nums");
        nums.push(num);
    }

    // Read queriesSize (number of queries)
    let queries_size = match next_token(&mut reader)?.parse::<usize>() {
        Ok(v) => v,
        Err(e) => {
            writeln!(io::stderr(), "Failed to parse queriesSize: {:?}", e)?;
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid queriesSize"));
        }
    };

    // Read the queries vector
    let mut queries: Vec<i64> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let token = next_token(&mut reader)?;
        let query = token.parse::<i64>().expect("Invalid number in queries");
        queries.push(query);
    }

    // Compute the answer using our function translating gcdValues from C
    let ans = gcd_values(&nums, &queries);

    // Print the answer
    for a in ans {
        write!(writer, "{} ", a)?;
    }
    writeln!(writer)?;
    writer.flush()?;

    Ok(())
}

// Helper function to get the next token from the input
fn next_token<T: BufRead>(lines: &mut std::io::Lines<T>) -> io::Result<String> {
    // This function reads tokens from input.
    // It will keep reading until it finds a valid non-empty token.
    loop {
        if let Some(line) = lines.next() {
            let line = line?;
            // If the line is empty, continue reading
            if line.trim().is_empty() {
                continue;
            }
            // Split the line into tokens and return the first token
            let mut tokens = line.split_whitespace();
            if let Some(token) = tokens.next() {
                // If there are remaining tokens, they will be buffered in the iterator.
                // To simulate the C code we simply process tokens one by one.
                // For simplicity, we put back the remaining tokens by creating a new line.
                // In our case, it's okay if multiple tokens are processed across iterations.
                // We create a new "line" with the remaining tokens and push it onto the iterator.
                // However, since we cannot "put back" in our implementation,
                // we process tokens one by one by joining the remaining ones back to a string
                // and making a new iterator that yields that string as the next line.
                // This ensures that we exactly read tokens as in the original C code.
                let remaining: Vec<&str> = tokens.collect();
                if !remaining.is_empty() {
                    // Prepend the remaining tokens as a new line for subsequent calls.
                    // Note: This is a hack to simulate token buffering.
                    let new_line = remaining.join(" ");
                    // Create a new iterator that produces this new line first.
                    // Then chain with the existing lines iterator.
                    let new_iter = std::iter::once(Ok(new_line));
                    *lines = new_iter.chain(lines.by_ref());
                }
                return Ok(token.to_string());
            }
        } else {
            break;
        }
    }
    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more tokens"))
}

/// Function that implements the logic of C's gcdValues function.
/// It computes for each query the minimal value such that the cumulative sum
/// of the number of pairs with gcd greater than that value is more than the query.
fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    // Determine the maximum value in nums.
    let mx = *nums.iter().max().unwrap_or(&0) as usize;

    // cnt_x: frequency of each number in nums.
    // We use mx+1 to account for 0-indexing and the fact that numbers go from 1..mx.
    let mut cnt_x = vec![0; mx + 1];
    for &num in nums {
        // Since numbers are positive, indexing is safe.
        cnt_x[num as usize] += 1;
    }

    // cnt_gcd: For each possible gcd value i, store the number of pairs with gcd exactly i.
    let mut cnt_gcd = vec![0i64; mx + 1];
    // Iterate in decreasing order from mx down to 1.
    for i in (1..=mx).rev() {
        let mut c = 0;
        // For every multiple j of i (j = i, i*2, i*3, ... <= mx),
        // add frequency and subtract counts from multiples (inclusion-exclusion)
        let mut j = i;
        while j <= mx {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        // Add the number of pairs that can be formed from c numbers:
        // c choose 2 = c*(c-1)/2.
        cnt_gcd[i] += (c as i64 * (c as i64 - 1)) / 2;
    }

    // Compute the prefix sum on cnt_gcd: each position accumulates all pairs with gcd <= current value.
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Answer the queries. For each query, find the smallest index such that cnt_gcd[index] > query.
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx;
        // Binary search similar to the C code:
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left as i32);
    }

    ans
}