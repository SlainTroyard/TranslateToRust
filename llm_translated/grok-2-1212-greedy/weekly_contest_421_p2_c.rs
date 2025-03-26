use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn length_after_transformations(s: &str, t: i32) -> i64 {
    let mut lst = [0; 26];
    for c in s.chars() {
        lst[(c as u8 - b'a') as usize] += 1;
    }

    for _ in 0..t {
        let z = lst[25];
        for i in (2..26).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % MOD;
        lst[0] = z;
    }

    lst.iter().map(|&x| x as i64).sum::<i64>() % MOD
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