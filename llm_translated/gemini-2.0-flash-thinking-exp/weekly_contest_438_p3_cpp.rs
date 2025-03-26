fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
    // Extended Euclidean Algorithm to find x, y such that ax + by = gcd(a, b)
    if b == 0 {
        *x = 1;
        *y = 0;
        return;
    }
    exgcd(b, a % b, y, x);
    *y -= a / b * *x;
}

fn has_same_digits(s: String) -> bool {
    let n = s.len();

    // Precalculate powers of 2 and 5 modulo 10
    let mut p2 = vec![0; n + 1];
    let mut p5 = vec![0; n + 1];
    p2[0] = 1;
    p5[0] = 1;
    for i in 1..=n {
        p2[i] = (p2[i - 1] * 2) % 10;
        p5[i] = (p5[i - 1] * 5) % 10;
    }

    // Closure to calculate a value based on a substring of s
    let calc = |l: usize, r: usize| -> i32 {
        let len = r - l;
        let mut c = 1;
        let mut two = 0;
        let mut five = 0;
        let mut sum = 0;
        let mut j = 0;
        for i in l..=r {
            // Calculate the contribution of the current digit
            sum = (sum + (s.chars().nth(i).unwrap() as i32 - '0' as i32) * p2[two] * p5[five] * c) % 10;
            if i == r {
                break;
            }
            // Update two, five, and c for the next digit
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
            let mut t = j + 1;
            while t % 2 == 0 {
                two -= 1;
                t /= 2;
            }
            while t % 5 == 0 {
                five -= 1;
                t /= 5;
            }
            let mut x = 0;
            let mut y = 0;
            exgcd(t as i32, 10, &mut x, &mut y);
            c = (c * (x % 10 + 10)) % 10; // Ensure x is positive for modulo
            j += 1;
        }
        sum
    };

    // Compare the results of calc for two substrings
    calc(0, n - 2) == calc(1, n - 1)
}

fn main() {
    // Read input string from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim(); // Remove trailing newline

    // Call has_same_digits function and print the result
    let result = has_same_digits(s.to_string());
    println!("{}", if result { 1 } else { 0 }); // Output 1 for true, 0 for false
}