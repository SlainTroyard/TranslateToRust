use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn construct_transformed_array(a: Vec<i32>) -> Vec<i32> {
        let n = a.len() as i32;
        let mut res = vec![0; a.len()];
        for i in 0..a.len() {
            let i_i32 = i as i32;
            let index = (i_i32 + a[i] % n + n) % n;
            res[i] = a[index as usize];
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    let transformed_array = solution.construct_transformed_array(a);

    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}