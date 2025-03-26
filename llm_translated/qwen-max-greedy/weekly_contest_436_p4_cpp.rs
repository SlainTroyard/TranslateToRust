use std::io::{self, BufRead, Write};

fn max_score(points: &Vec<i32>, m: i32) -> i64 {
    let check = |low: i64| -> bool {
        let n = points.len();
        let mut rem = m;
        let mut pre = 0;
        for (i, &point) in points.iter().enumerate() {
            let k = ((low - 1) / point as i64 + 1 - pre as i64).max(1);
            if i == n - 1 && k <= 0 {
                break;
            }
            rem -= (k * 2 - 1) as i32;
            if rem < 0 {
                return false;
            }
            pre = (k - 1) as usize;
        }
        true
    };

    let mut left = 0;
    let right = (m as i64 + 1) / 2 * *points.iter().min().unwrap() as i64 + 1;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_to_string(&mut buffer).expect("Failed to read from stdin");

    // Parse input
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let m: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let points: Vec<i32> = iter.map(|x| x.parse().expect("Invalid input")).collect();

    // Create solution and compute the result
    let result = max_score(&points, m);

    // Write output to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result).expect("Failed to write to stdout");
}