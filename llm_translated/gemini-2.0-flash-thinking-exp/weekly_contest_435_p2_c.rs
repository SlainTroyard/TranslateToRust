fn abs_val(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    let length = s.len();

    for i in 0..length {
        match s.chars().nth(i).unwrap() {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => {} // Handle other characters if needed, though problem statement implies only 'N', 'S', 'E', 'W'
        }

        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }

    ans
}

fn main() {
    use std::io;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");

    let parts: Vec<&str> = input_line.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid integer input");

    let result = max_distance(s, k);
    println!("{}", result);
}