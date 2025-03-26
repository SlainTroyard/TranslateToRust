use std::io::{self, Read};

/// Calculates GCD values based on the given nums and queries
/// 
/// This function implements the algorithm from the original C code
fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    // Find the maximum value in nums
    let mx = *nums.iter().max().unwrap_or(&0);
    
    // Count occurrences of each number
    let mut cnt_x = vec![0; mx as usize + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }
    
    // Calculate cnt_gcd values
    let mut cnt_gcd = vec![0i64; mx as usize + 1];
    for i in (1..=mx as usize).rev() {
        let mut c = 0;
        let mut j = i;
        while j <= mx as usize {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        cnt_gcd[i] += (c as i64 * (c - 1) as i64) / 2;
    }
    
    // Cumulative sum
    for i in 1..=mx as usize {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }
    
    // Binary search for each query
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx as usize;
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut iter = input.split_whitespace();
    
    // Read nums_size and nums
    let nums_size: usize = iter.next().expect("Missing nums_size").parse()
        .expect("nums_size must be an integer");
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(iter.next().expect("Not enough nums")
            .parse::<i32>().expect("nums must be integers"));
    }
    
    // Read queries_size and queries
    let queries_size: usize = iter.next().expect("Missing queries_size").parse()
        .expect("queries_size must be an integer");
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        queries.push(iter.next().expect("Not enough queries")
            .parse::<i64>().expect("queries must be integers"));
    }
    
    // Calculate and print answers
    let ans = gcd_values(&nums, &queries);
    for (i, &val) in ans.iter().enumerate() {
        print!("{}", val);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    println!();
    
    Ok(())
}