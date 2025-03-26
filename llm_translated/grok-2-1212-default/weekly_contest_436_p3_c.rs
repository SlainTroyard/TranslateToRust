use std::io::{self, Read};

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0;
    let mut f = [[0; 9]; 10]; // Initialize counting array to 0

    let len = s.len();
    for i in 0..len {
        let d = s.as_bytes()[i] - b'0'; // Current digit

        for m in 1..10 {
            let mut nf = [0; 9]; // Temporary array to store new count results
            nf[d as usize % m] = 1; // Single digit d modulo m

            for rem in 0..m {
                // Update count: add current digit d to existing remainder rem to form new remainder
                nf[(rem * 10 + d as usize) % m] += f[m][rem];
            }

            // Update f array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // Current digit divisible by itself, increment result count
        ans += f[d as usize][0] as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Process each line of input
    for line in input.lines() {
        let result = count_substrings(line);

        // Output result
        println!("{}", result);
    }

    Ok(())
}