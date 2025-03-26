use std::io::{self, Write};

fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    for c in target.chars() {
        return_size += (c as u8 - b'a' + 1) as usize;
    }

    let mut ans = Vec::with_capacity(return_size);
    let mut i = 0;

    for (l, &t) in target.as_bytes().iter().enumerate() {
        for c in b'a'..=t {
            let mut s = target[0..l].to_string();
            s.push(c as char);
            ans.push(s);
            i += 1;
        }
    }

    ans
}

fn main() {
    let mut target = String::new();
    print!("Enter the target string: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read line");

    target = target.trim().to_string(); // Remove any trailing newline or whitespace

    let ans = string_sequence(&target);

    for s in ans.iter() {
        println!("{}", s);
    }
}