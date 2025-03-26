use std::io::{self, BufRead};

fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort_unstable();

    for &num in arr {
        let x = if prev + 1 > num - diff { prev + 1 } else { num - diff };

        if x <= num + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read array size and difference
    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    let diff: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Read array elements
    let mut arr: Vec<i32> = lines
        .take(n as usize)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);

    Ok(())
}