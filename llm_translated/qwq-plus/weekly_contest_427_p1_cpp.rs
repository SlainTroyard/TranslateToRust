use std::io;

pub fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let a_i = a[i];
        let mod_n = a_i % (n as i32);
        let temp = i as i32 + mod_n + (n as i32);
        let index = temp % (n as i32);
        let idx = index as usize;
        res.push(a[idx]);
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = vec![];
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        tokens.extend(line.split_whitespace());
    }

    let n: usize = tokens[0].parse().unwrap();
    let a: Vec<i32> = tokens[1..1 + n]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let transformed = construct_transformed_array(&a);

    for num in &transformed {
        print!("{} ", num);
    }
    println!();
}