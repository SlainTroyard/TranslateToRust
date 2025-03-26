use std::io::{self, BufRead, Write};

fn gcd_values(nums: &Vec<i32>, queries: &Vec<i64>) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap_or(&0);

    // Count occurrences of each number
    let mut cnt_x = vec![0; (mx + 1) as usize];
    for &num in nums.iter() {
        cnt_x[num as usize] += 1;
    }

    // Calculate the number of pairs with GCD >= i
    let mut cnt_gcd = vec![0; (mx + 1) as usize];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i as usize) {
            c += cnt_x[j as usize];
            cnt_gcd[i as usize] -= cnt_gcd[j as usize];
        }
        cnt_gcd[i as usize] += (c * (c - 1) / 2) as i64;
    }

    // Prefix sum to get the number of pairs with GCD exactly i
    for i in 1..=mx {
        cnt_gcd[i as usize] += cnt_gcd[(i - 1) as usize];
    }

    // Find the answer for each query
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries.iter() {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid as usize] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left);
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the size of nums
    let nums_size = read_line_as_i32(&mut stdin, "Enter the size of nums: ").unwrap();
    let mut nums = Vec::with_capacity(nums_size as usize);
    for _ in 0..nums_size {
        nums.push(read_line_as_i32(&mut stdin, "Enter a number: ").unwrap());
    }

    // Read the size of queries
    let queries_size = read_line_as_i32(&mut stdin, "Enter the size of queries: ").unwrap();
    let mut queries = Vec::with_capacity(queries_size as usize);
    for _ in 0..queries_size {
        queries.push(read_line_as_i64(&mut stdin, "Enter a query: ").unwrap());
    }

    // Get the results
    let ans = gcd_values(&nums, &queries);

    // Print the results
    for num in ans {
        writeln!(&mut stdout, "{}", num).unwrap();
    }
}

// Helper function to read a line as an i32
fn read_line_as_i32<R: BufRead>(reader: &mut R, prompt: &str) -> Result<i32, std::num::ParseIntError> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input)?;
    input.trim().parse::<i32>()
}

// Helper function to read a line as an i64
fn read_line_as_i64<R: BufRead>(reader: &mut R, prompt: &str) -> Result<i64, std::num::ParseIntError> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input)?;
    input.trim().parse::<i64>()
}