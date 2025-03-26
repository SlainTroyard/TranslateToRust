use std::io::{self, BufRead};

fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
    let mut prev = i32::MIN;
    let mut c = std::collections::BTreeSet::new();
    arr.sort_unstable();
    for &num in arr {
        let x = prev + 1.max(num - diff);
        if x <= num + diff {
            c.insert(x);
            prev = x;
        }
    }
    c.len()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input array size and difference
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let diff: i32 = lines.next().unwrap()?.parse().unwrap();

    // Input array elements
    let mut arr: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);

    Ok(())
}