use std::io::{self, BufRead, Write};

fn main() {
    // Read the input size for vectors a and b
    let (a_size, b_size) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        (nums[0], nums[1])
    };

    // Initialize vectors a and b with the given sizes
    let mut a = vec![0; a_size];
    let mut b = vec![0; b_size];

    // Read the elements of vector a
    for i in 0..a_size {
        a[i] = read_integer();
    }

    // Read the elements of vector b
    for i in 0..b_size {
        b[i] = read_integer();
    }

    // Create an instance of the Solution struct and compute the max score
    let solution = Solution;
    let result = solution.max_score(a, b);

    // Print the result
    println!("{}", result);
}

// Helper function to read an integer from stdin
fn read_integer() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

struct Solution;

impl Solution {
    /// Computes the maximum score based on the given algorithm.
    ///
    /// # Arguments
    ///
    /// * `a` - A vector of integers.
    /// * `b` - A vector of integers.
    ///
    /// # Returns
    ///
    /// * The maximum score as a 64-bit signed integer.
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut f = [0i64; 5];
        f.iter_mut().for_each(|f_i| *f_i = i64::MIN / 2);

        for &y in &b {
            for j in (0..=3).rev() {
                f[j + 1] = f[j + 1].max(f[j] + (a[j as usize] as i64) * (y as i64));
            }
        }

        f[4]
    }
}