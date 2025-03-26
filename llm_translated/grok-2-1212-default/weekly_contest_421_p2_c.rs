use std::io::{self, Read};

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst = [0; 26];
    for c in s.chars() {
        lst[c as usize - 'a' as usize] += 1;
    }

    let mut t = t;
    while t > 0 {
        let z = lst[25];
        for i in (2..26).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % 1_000_000_007;
        lst[0] = z;
        t -= 1;
    }

    let mut ans = 0;
    for i in 0..26 {
        ans = (ans + lst[i]) % 1_000_000_007;
    }
    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let t: i32 = lines.next().unwrap().trim().parse().unwrap();

    let result = length_after_transformations(s, t);
    println!("{}", result);

    Ok(())
}