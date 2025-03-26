use std::io;
use std::io::Read;

// This function implements the logic of the C++ minZeroArray function.
// It uses a difference array "d" to simulate range updates and then tracks
// the prefix sum until it becomes positive, adjusting it per query.
fn min_zero_array(nums: &Vec<i32>, queries: &Vec<[i32; 3]>) -> i32 {
    let n = nums.len();
    // Create difference array "d" with length n+1, initialized to zeros.
    let mut d = vec![0; n + 1];

    // Set up the difference array based on the original nums.
    d[0] = nums[0];
    for i in 1..n {
        d[i] = nums[i] - nums[i - 1];
    }

    // acc holds the accumulated prefix sum.
    // cur is our current index (starting before array at -1).
    let mut acc = 0;
    let mut cur: isize = -1;
    // Move cur forward until the prefix sum becomes positive or we reach the end.
    while acc <= 0 && cur < n as isize {
        cur += 1; // Move to the next index.
        acc += d[cur as usize];
    }
    // If we reached the end, then no queries are required.
    if cur == n as isize {
        return 0;
    }

    // Process each query.
    // Each query is in the form [l, r, x] meaning we add 'x' to each element in range [l, r]
    // when thinking in terms of the difference array, this translates to:
    //   d[r+1] += x and d[l] -= x.
    for (i, query) in queries.iter().enumerate() {
        let l = query[0] as usize;
        let r = query[1] as usize;
        let x = query[2];
        // Update the difference array.
        d[r + 1] += x;
        d[l] -= x;
        // If our current index "cur" is within the affected query range,
        // update the accumulated sum.
        if (cur as usize) >= l && (cur as usize) <= r {
            acc -= x;
            // Advance "cur" until the accumulated sum becomes positive again or we run off the end.
            while acc <= 0 && cur < n as isize {
                cur += 1;
                acc += d[cur as usize];
            }
            // If after processing this query we have reached the end, return the (1-indexed) query count.
            if cur == n as isize {
                return (i as i32) + 1;
            }
        }
    }
    // If the accumulated sum never reached positive after any query, return -1.
    -1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input on whitespace.
    let mut tokens = input.split_whitespace();

    // Read the size of the "nums" array.
    let n: usize = tokens
        .next()
        .ok_or("Expected a number for n")?
        .parse()
        .map_err(|_| "Invalid integer for n")?;

    // Read the "nums" array.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens
            .next()
            .ok_or("Expected a number in nums array")?
            .parse::<i32>()
            .map_err(|_| "Invalid integer in nums")?;
        nums.push(num);
    }

    // Read the number of queries.
    let m: usize = tokens
        .next()
        .ok_or("Expected a number for m")?
        .parse()
        .map_err(|_| "Invalid integer for m")?;

    // Read the queries. Each query consists of three integers.
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let l = tokens
            .next()
            .ok_or("Expected a query integer (l)")?
            .parse::<i32>()
            .map_err(|_| "Invalid query integer")?;
        let r = tokens
            .next()
            .ok_or("Expected a query integer (r)")?
            .parse::<i32>()
            .map_err(|_| "Invalid query integer")?;
        let x = tokens
            .next()
            .ok_or("Expected a query integer (x)")?
            .parse::<i32>()
            .map_err(|_| "Invalid query integer")?;
        queries.push([l, r, x]);
    }

    // Compute the result using our solution function and print it.
    let result = min_zero_array(&nums, &queries);
    println!("{}", result);

    Ok(())
}