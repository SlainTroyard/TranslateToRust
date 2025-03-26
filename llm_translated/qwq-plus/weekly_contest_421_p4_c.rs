use std::io;

struct Matrix {
    m: [[i32; 26]; 26],
}

impl Matrix {
    fn identity() -> Matrix {
        let mut m = [[0; 26]; 26];
        for i in 0..26 {
            m[i][i] = 1;
        }
        Matrix { m }
    }
}

const MOD: i32 = 1_000_000_007;

fn multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..26 {
        for y in 0..26 {
            let mut sum = 0i64;
            for z in 0..26 {
                sum += (a.m[x][z] as i64) * (b.m[z][y] as i64);
                sum %= MOD as i64;
            }
            result.m[x][y] = sum as i32;
        }
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut src = [0; 26];
    for c in s.chars() {
        let idx = (c as usize) - ('a' as usize);
        src[idx] += 1;
    }

    let mut init = Matrix { m: [[0; 26]; 26] };
    for x in 0..26 {
        let max_y = nums[x] as i32;
        for y_val in 1..=max_y {
            let z = (x as i32 + y_val) % 26;
            let z = z as usize;
            init.m[z][x] = 1;
        }
    }

    let mut dp = [Matrix::identity(), Matrix { m: [[0; 26]; 26] }];
    let mut current = 0;

    let mut bits = vec![];
    let mut temp = t;
    while temp > 0 {
        bits.push(temp & 1);
        temp >>= 1;
    }
    bits.reverse();

    for &bit in &bits {
        multiply(&dp[current], &dp[current], &mut dp[1 - current]);
        if bit == 1 {
            multiply(&dp[1 - current], &init, &mut dp[current]);
        } else {
            current = 1 - current;
        }
    }

    let final_matrix = &dp[current];
    let mut result = 0i64;
    for x in 0..26 {
        for y in 0..26 {
            result = (result + (final_matrix.m[x][y] as i64) * (src[y] as i64)) % MOD as i64;
        }
    }
    result as i32 % MOD
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let s_len: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().to_string();
    let t: i32 = iter.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = (0..26).map(|_| iter.next().unwrap().parse().unwrap()).collect();

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}