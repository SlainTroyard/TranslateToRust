use std::io;

/// Calculates the maximum distance achievable after moving along the path 's' with up to 'k' extra moves.
///
/// # Arguments
/// * `s` - A string slice representing the path of moves ('N', 'S', 'E', 'W').
/// * `k` - The number of extra moves allowed.
///
/// # Returns
/// The maximum possible distance as an integer.
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (),
        }
        // Calculate the current maximum possible distance
        let current_max = (x.abs() + y.abs() + 2 * k).min(i as i32 + 1);
        ans = ans.max(current_max);
    }
    ans
}

fn main() {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    }

    // Split input into two parts: the string and the integer
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    let s = parts[0];
    let k = match parts[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    let result = max_distance(s, k);
    println!("{}", result);
}