const MODULO_VAL: i32 = 1000000007;
const RADIX_VAL: usize = 26;

#[derive(Clone)]
struct Matrix {
    m: [[i32; RADIX_VAL]; RADIX_VAL],
}

fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] };
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            let mut sum = 0;
            for z in 0..RADIX_VAL {
                sum = (sum + (a.m[x][z] as i64) * (b.m[z][y] as i64)) % MODULO_VAL as i64;
            }
            result.m[x][y] = sum as i32;
        }
    }
    result
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut src = [0; RADIX_VAL];
    for c in s.chars() {
        let idx = c as usize - 'a' as usize;
        if idx < RADIX_VAL {
            src[idx] += 1;
        }
    }

    let mut init = Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] };
    for x in 0..RADIX_VAL {
        for y in 1..=nums[x] as usize {
            let z = (x + y) % RADIX_VAL;
            init.m[z][x] = 1;
        }
    }

    let mut digits = Vec::new();
    let mut temp = t;
    if temp == 0 {
        digits.push(0);
    } else {
        while temp > 0 {
            digits.push(temp % 2);
            temp /= 2;
        }
        digits.reverse();
    }

    let mut dp = [
        Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] },
        Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] },
    ];
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
    }

    let mut z = 0;
    for i in 0..digits.len() {
        let bit = digits[i];
        dp[1 - z] = multiply(&dp[z], &dp[z]);
        if bit == 1 {
            dp[z] = multiply(&dp[1 - z], &init);
        } else {
            z = 1 - z;
        }
    }

    let final_matrix = &dp[z];
    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (result + (final_matrix.m[x][y] as i64) * (src[y] as i64)) % MODULO_VAL as i64;
        }
    }
    result as i32
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s_len: usize = input.trim().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    let mut nums = Vec::new();
    for _ in 0..26 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();
        nums.push(num);
    }

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}