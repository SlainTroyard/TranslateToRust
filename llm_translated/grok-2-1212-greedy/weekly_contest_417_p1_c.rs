use std::io::{self, Read};

fn kth_character(k: usize) -> char {
    let mut l = vec![0; k + 1];
    for a in 0.. {
        for i in 0..(1 << a) {
            let j = 1 << a;
            l[i + j] = l[i] + 1;
            if i + j >= k - 1 {
                return (97 + l[k - 1] % 26) as u8 as char;
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let k: usize = input.trim().parse().expect("Invalid input");

    let result = kth_character(k);
    println!("{}", result);

    Ok(())
}