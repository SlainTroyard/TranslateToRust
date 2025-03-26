use std::io::{self, Read};

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0;
    let mut f = [[0; 9]; 10]; // Initialize counting array to 0

    let len = s.len();
    for i in 0..len {
        let d = s.chars().nth(i).unwrap() as u8 - b'0'; // Current digit

        for m in 1..10 {
            let mut nf = [0; 9]; // Temporary array to store new count results
            nf[(d % m as u8) as usize] = 1; // Single digit d modulo m

            for rem in 0..m {
                // Update count: add current digit d to existing remainder rem to form new remainder
                nf[((rem as u8 * 10 + d) % m as u8) as usize] += f[m][rem];
            }

            // Update f array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // Current digit divides itself, increment result count
        ans += f[d as usize][0] as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Trim whitespace and process each line
    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }

        // Call function to calculate result
        let result = count_substrings(s);

        // Output result
        println!("{}", result);
    }

    Ok(())
}