use lazy_static::lazy_static;

const MOD: i64 = 1000000007;
const MX: usize = 201;

lazy_static! {
    static ref LCM: Vec<Vec<i32>> = {
        let mut lcms = vec![vec![0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                let g = gcd(i as i32, j as i32);
                lcms[i][j] = (i as i32 * j as i32) / g;
            }
        }
        lcms
    };

    static ref POW2: Vec<i64> = {
        let mut pow2 = vec![1; MX];
        for i in 1..MX {
            pow2[i] = (pow2[i-1] * 2) % MOD;
        }
        pow2
    };

    static ref POW3: Vec<i64> = {
        let mut pow3 = vec![1; MX];
        for i in 1..MX {
            pow3[i] = (pow3[i-1] * 3) % MOD;
        }
        pow3
    };

    static ref MU: Vec<i32> = {
        let mut mu = vec![0; MX];
        mu[1] = 1;
        for i in 1..MX {
            for j in (2*i..MX).step_by(i) {
                mu[j] -= mu[i];
            }
        }
        mu
    };
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let m = *nums.iter().max().unwrap();
    if m == 0 {
        return 0;
    }

    let mut cnt = vec![0; (m as usize) + 1];
    for &num in nums {
        cnt[num as usize] += 1;
    }

    for i in 1..=m {
        for j in (2*i..=m).step_by(i as usize) {
            cnt[i as usize] += cnt[j as usize];
        }
    }

    let mut f = vec![vec![0; (m as usize) + 1]; (m as usize) + 1];

    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = LCM[g1 as usize][g2 as usize];
            let c = if l <= m { cnt[l as usize] } else { 0 };
            let c1 = cnt[g1 as usize];
            let c2 = cnt[g2 as usize];
            let term1 = (POW3[c as usize] * POW2[(c1 + c2 - 2 * c) as usize]) % MOD;
            let term2 = (term1 - POW2[c1 as usize] - POW2[c2 as usize] + 1 + MOD) % MOD;
            f[g1 as usize][g2 as usize] = term2 as i32;
        }
    }

    let mut ans = 0i64;
    for i in 1..=m {
        let max_jk = m / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                if gj > m || gk > m {
                    continue;
                }
                ans += (MU[j as usize] as i64) * (MU[k as usize] as i64) * (f[gj as usize][gk as usize] as i64);
                ans %= MOD;
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;
    ans as i32
}

fn read_all_input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let data = read_all_input();
    if data.is_empty() {
        println!("0");
        return;
    }
    let n = data[0] as usize;
    let nums = &data[1..n+1];
    let result = subsequence_pair_count(nums);
    println!("{}", result);
}