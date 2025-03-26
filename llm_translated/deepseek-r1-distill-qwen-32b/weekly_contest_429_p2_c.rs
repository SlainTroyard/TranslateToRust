use std::cmp;
use std::io;

fn max_distinct_elements(mut arr: Vec<i32>, diff: i32) -> i32 {
    arr.sort();
    let mut prev = i32::MIN;
    let mut count = 0;

    for &x in arr.iter() {
        let x_val = cmp::max(prev + 1, x - diff);
        if x_val <= x + diff {
            count += 1;
            prev = x_val;
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut tokens = buffer.split_whitespace();
    let n: usize = tokens.next().unwrap().parse()?;
    let diff: i32 = tokens.next().unwrap().parse()?;

    let arr: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();

    let result = max_distinct_elements(arr, diff);
    println!("{}", result);

    Ok(())
}