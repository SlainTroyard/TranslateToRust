use std::io::{self, BufRead};

fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    for c in target.chars() {
        return_size += (c as u8 - b'a' + 1) as usize;
    }

    let mut ans = Vec::with_capacity(return_size);
    let mut l = 0;
    for (i, &c) in target.as_bytes().iter().enumerate() {
        for ch in b'a'..=c {
            let mut s = String::with_capacity(i + 1);
            s.push_str(&target[..i]);
            s.push(ch as char);
            ans.push(s);
        }
        l += 1;
    }
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the target string
    let target = lines.next().unwrap()?.trim().to_string();

    // Generate the sequence
    let ans = string_sequence(&target);

    // Print the result
    for s in ans {
        print!("{} ", s);
    }
    println!();

    Ok(())
}