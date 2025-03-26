use std::io::{self, BufRead, Write};

fn length_after_transformations(s: &str, t: usize) -> i32 {
    let mut lst = [0; 26];
    for c in s.chars() {
        if let Some(index) = c.to_ascii_lowercase().checked_sub('a' as u8) {
            lst[index as usize] += 1;
        }
    }

    for _ in 0..t {
        let z = lst[25];
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % 1_000_000_007;
        lst[0] = z;
    }

    lst.iter().fold(0, |acc, &x| (acc + x) % 1_000_000_007)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the string and the integer from stdin
    let mut s = String::new();
    stdin.lock().read_line(&mut s)?;
    let s = s.trim();

    let mut t_str = String::new();
    stdin.lock().read_line(&mut t_str)?;
    let t: usize = t_str.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse integer"))?;

    // Calculate the result and print it to stdout
    let result = length_after_transformations(s, t);
    writeln!(stdout, "{}", result)?;

    Ok(())
}