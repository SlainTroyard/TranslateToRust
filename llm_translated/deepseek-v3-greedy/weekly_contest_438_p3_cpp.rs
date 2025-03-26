use std::io;

// Extended Euclidean Algorithm to find the modular inverse
fn exgcd(a: i32, b: i32) -> (i32, i32) {
    if b == 0 {
        return (1, 0);
    }
    let (x, y) = exgcd(b, a % b);
    (y, x - (a / b) * y)
}

fn has_same_digits(s: &str) -> bool {
    let n = s.len();

    // Precompute powers of 2 and 5 modulo 10
    let mut p2 = vec![1; n + 1];
    let mut p5 = vec![1; n + 1];
    for i in 1..=n {
        p2[i] = (p2[i - 1] * 2) % 10;
        p5[i] = (p5[i - 1] * 5) % 10;
    }

    // Lambda function to calculate the sum for a given range
    let calc = |l: usize, r: usize| -> i32 {
        let len = r - l;
        let mut c = 1;
        let mut two = 0;
        let mut five = 0;
        let mut sum = 0;
        let mut i = l;
        let mut j = 0;
        loop {
            sum = (sum + (s.chars().nth(i).unwrap() as i32 - '0' as i32) * p2[two] * p5[five] * c) % 10;
            if i == r {
                break;
            }
            let mut t = len - j;
            while t % 2 == 0 {
                two += 1;
                t /= 2;
            }
            while t % 5 == 0 {
                five += 1;
                t /= 5;
            }
            c = (c * t) % 10;
            t = j + 1;
            while t % 2 == 0 {
                two -= 1;
                t /= 2;
            }
            while t % 5 == 0 {
                five -= 1;
                t /= 5;
            }
            let (x, _) = exgcd(t, 10);
            c = (c * (x % 10 + 10)) % 10;
            i += 1;
            j += 1;
        }
        sum
    };

    // Compare the sums of the two ranges
    calc(0, n - 2) == calc(1, n - 1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();
    let result = has_same_digits(s);
    println!("{}", result);
}