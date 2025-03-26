use std::io;
use std::error::Error;

fn construct_transformed_array(A: Vec<i32>) -> Vec<i32> {
    let n = A.len();
    let mut res = vec![0; n];
    for i in 0..n {
        let idx = (i as i32 + A[i] % n as i32) % n as i32;
        res[i] = A[idx as usize];
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let n: usize = tokens[0].parse()?;
    let A: Vec<i32> = tokens[1..n+1].iter().map(|s| s.parse().unwrap()).collect();
    let transformed = construct_transformed_array(A);
    println!("{}", transformed.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    Ok(())
}