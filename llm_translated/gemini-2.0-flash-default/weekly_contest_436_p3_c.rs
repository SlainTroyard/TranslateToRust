use std::io;
use std::io::Read;

fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut f: [[i64; 9]; 10] = [[0; 9]; 10];

    let len = s.len();
    for i in 0..len {
        let d = (s.chars().nth(i).unwrap() as u32 - '0' as u32) as usize;

        for m in 1..10 {
            let mut nf: [i64; 9] = [0; 9];
            nf[d % m] = 1;

            for rem in 0..m {
                nf[(rem * 10 + d) % m] += f[m][rem];
            }

            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        ans += f[d][0];
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let s: String = input.trim().parse()?;

    let result = count_substrings(&s);

    println!("{}", result);

    Ok(())
}