// Problem: Weekly Contest 417 Problem 1

fn kth_character(k: i32) -> char {
    let mut l = vec![0; (k + 1) as usize];
    let k_minus_1 = (k - 1) as usize;
    for a in 0.. {
        let j = 1 << a;
        for i in 0..j {
            let index = i as usize + j as usize;
            if index >= l.len() {
                l.resize(index + 1, 0);
            }
            l[index] = l[i as usize] + 1;
            if index >= k_minus_1 {
                let c = (97 + (l[k_minus_1] % 26)) as u8;
                return c as char;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let k = input.trim().parse::<i32>().expect("Please enter a valid integer");
    let c = kth_character(k);
    println!("{}", c);
    Ok(())
}