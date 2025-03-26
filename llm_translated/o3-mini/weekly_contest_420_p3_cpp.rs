use std::io::{self, BufRead, Write};

// Maximum limit (1_000_001) as in the C++ code.
const MX: usize = 1_000_001;

// Precompute the least prime factor (LPF) for every number from 0 to MX-1.
// This function is equivalent to the C++ lambda 'init'.
fn init_lpf() -> Vec<usize> {
    let mut lpf = vec![0; MX];
    // Start from 2 as 0 and 1 are not prime.
    for i in 2..MX {
        if lpf[i] == 0 {
            // Mark multiples of the prime i.
            for j in (i..MX).step_by(i) {
                if lpf[j] == 0 {
                    lpf[j] = i;
                }
            }
        }
    }
    lpf
}

// The Solution struct simulates the C++ class.
struct Solution {
    lpf: Vec<usize>,
}

impl Solution {
    // Constructor to create a new Solution with precomputed LPF array.
    fn new(lpf: Vec<usize>) -> Self {
        Self { lpf }
    }

    // Equivalent to the minOperations method in C++.
    // It takes a mutable vector of numbers and returns the minimum number of operations,
    // or -1 if it's impossible.
    fn min_operations(&self, nums: &mut [usize]) -> isize {
        let mut ans = 0;
        // Loop from the second-to-last element down to the first.
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                // Replace nums[i] with its least prime factor.
                nums[i] = self.lpf[nums[i]];
                // If the condition still does not hold, return -1.
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Set up input reading.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();
    // Read the first line which contains n (the number of elements).
    let n_line = reader
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing input for n"))??;
    let n: usize = n_line
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to parse n"))?;

    // Read the next n numbers. They might be space-separated in one line or provided in separate lines.
    // We'll collect them by reading lines until we have n numbers.
    let mut nums: Vec<usize> = Vec::with_capacity(n);
    while nums.len() < n {
        let line = reader
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing numbers input"))??;
        // Split the line by whitespace and try parsing each token.
        for token in line.split_whitespace() {
            if nums.len() < n {
                let num = token
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to parse a number"))?;
                nums.push(num);
            } else {
                break;
            }
        }
    }

    // Precompute the least prime factors.
    let lpf = init_lpf();

    // Create a new instance of Solution.
    let sol = Solution::new(lpf);

    // Compute the answer using the solution's min_operations method.
    let result = sol.min_operations(&mut nums);

    // Write the output to stdout.
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result)?;
    Ok(())
}