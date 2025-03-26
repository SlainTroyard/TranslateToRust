use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
        pizzas.sort_by(|a, b| b.cmp(a));
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }
        for i in 0..days / 2 {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of pizzas
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the pizza weights
    let mut pizzas: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let solution = Solution;
    println!("{}", solution.max_weight(&mut pizzas));

    Ok(())
}