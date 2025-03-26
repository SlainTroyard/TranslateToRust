fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    for t in target.chars() {
        return_size += (t as u32) - 96;
    }

    let mut ans: Vec<String> = Vec::with_capacity(return_size as usize);
    let mut i = 0;
    let mut l = 0;

    for t in target.chars() {
        for c in 'a'..=(t) {
            let mut s = target[0..l].to_string();
            s.push(c);
            ans.push(s);
            i += 1;
        }
        l += 1;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Read};

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let target = buffer.trim();

    let ans = string_sequence(target);

    for s in &ans {
        print!("{} ", s);
    }
    println!();

    Ok(())
}