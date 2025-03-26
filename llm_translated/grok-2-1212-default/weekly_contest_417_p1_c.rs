use std::io::{self, Read};

/// Calculates the kth character in the sequence
fn kth_character(k: i32) -> char {
    // Initialize the vector with k+1 zeros
    let mut l = vec![0; (k + 1) as usize];
    
    for a in 0.. {
        let j = 1 << a;
        for i in 0..j {
            // Update the vector and check if we've reached the kth character
            l[i + j] = l[i] + 1;
            if (i + j) as i32 >= k - 1 {
                return char::from_u32(97 + (l[(k - 1) as usize] % 26)).unwrap();
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Parse the input
    let k: i32 = input.trim().parse().expect("Failed to parse input");
    
    // Calculate and print the result
    let result = kth_character(k);
    println!("{}", result);
    
    Ok(())
}