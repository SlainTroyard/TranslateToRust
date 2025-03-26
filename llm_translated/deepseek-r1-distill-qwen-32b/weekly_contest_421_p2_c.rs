const MOD: i64 = 1_000_000_007;

fn length_after_transformations(s: &str, t: i32) -> i64 {
    let mut lst = [0; 26];
    for c in s.chars() {
        let idx = c.to_lowercase().next().unwrap() as usize - 'a' as usize;
        lst[idx] += 1;
    }
    let mut lst = lst.map(|x| x as i64);
    
    for _ in 0..t {
        let z = lst[25];
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % MOD;
        lst[0] = z;
    }
    
    lst.iter().sum::<i64>() % MOD
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let t = parts[1].parse::<i32>().expect("Invalid t");
    println!("{}", length_after_transformations(s, t));
}