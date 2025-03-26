use std::io::{self, BufRead};

const MOD: i64 = 1000000007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721;   // 80*9=720 plus one extra for indexing
const MAX_COUNT: usize = 81;  // maximum count of digits

// The Solver struct holds all precomputed arrays and dp table.
struct Solver {
    cnt: [usize; MAX_DIGITS],
    left_s: [usize; MAX_DIGITS], // cumulative sums from current digit to 0
    left_c: [usize; MAX_DIGITS], // cumulative count from current digit to 0
    dp: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
    r1: [i64; MAX_DIGITS + 1],
    // Pascal's triangle for combinations: cb[i][j] = C(i, j) mod MOD.
    cb: [[i64; MAX_COUNT]; MAX_COUNT],
    n: usize, // length of the number string
}

impl Solver {
    // Initialize a new Solver instance with default values.
    fn new() -> Self {
        Self {
            cnt: [0; MAX_DIGITS],
            left_s: [0; MAX_DIGITS],
            left_c: [0; MAX_DIGITS],
            dp: [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
            r1: [0; MAX_DIGITS + 1],
            cb: [[0; MAX_COUNT]; MAX_COUNT],
            n: 0,
        }
    }

    // Build Pascal's triangle (combination table) modulo MOD.
    fn pascal(&mut self) {
        // initialize all to 0
        for i in 0..MAX_COUNT {
            for j in 0..MAX_COUNT {
                self.cb[i][j] = 0;
            }
        }
        self.cb[0][0] = 1;
        for i in 1..MAX_COUNT {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % MOD;
            }
        }
    }

    // The recursive DP function.
    // Arguments:
    //   i - current digit (from 0 to MAX_DIGITS-1)
    //   s - remaining sum to achieve
    //   c - remaining digit count to pick
    fn dfs(&mut self, i: usize, s: usize, c: usize) -> i64 {
        // Base case: if no sum and no count left, return the precomputed value.
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        // If we've reached the end of digits, no valid arrangement exists.
        if i == MAX_DIGITS {
            return 0;
        }
        // Use precomputed left_s and left_c to prune invalid states.
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }
        // Return memoized result if already computed.
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }

        let mut res = 0;
        // For digit i, we try to use x of them (thus y = cnt[i] - x not used for the left side).
        // Note that in the original code, "a" represents s - x * i.
        for x in 0..=self.cnt[i] {
            // s' = s - x * i
            // If s' would be negative, break out since further increases in x only decrease s further.
            if x * i > s {
                break;
            }
            // Ensure we have enough digits left for the left side.
            if c < x {
                break;
            }
            let a = s - x * i;
            // y is the number of this digit not used in the left side.
            let y = self.cnt[i] - x;
            // Check the condition from the original code: if y > left_c[i] - c, skip.
            if y > self.left_c[i] - c {
                continue;
            }
            // Recurrence:
            // Multiply the result from using x digits of value i on the left side.
            // Use combinations: choose x positions from c positions and choose y positions from the remainder.
            let sub = self.dfs(i + 1, a, c - x);
            let comb1 = self.cb[c][x];
            let comb2 = self.cb[self.left_c[i] - c][y];
            let b = (sub * comb1) % MOD;
            res = (res + (b * comb2) % MOD) % MOD;
        }

        self.dp[i][s][c] = res;
        res
    }

    // Main function to calculate the count of balanced permutations.
    // Takes the input number as a string slice.
    fn count_balanced_permutations(&mut self, num: &str) -> i32 {
        let mut s: usize = 0;
        // Zero out the count array.
        self.cnt = [0; MAX_DIGITS];

        // Count the occurrences of each digit and compute the total sum.
        for ch in num.chars() {
            // Skip non-digit characters (should not happen based on problem format)
            if !ch.is_digit(10) {
                continue;
            }
            let digit = ch.to_digit(10).unwrap() as usize;
            s += digit;
            self.cnt[digit] += 1;
        }

        // If the sum is odd, no balanced permutation is possible.
        if s % 2 == 1 {
            return 0;
        }

        // Initialize Pascal's triangle.
        self.pascal();

        // Precompute r1 array.
        // r1[MAX_DIGITS] is set to 1 as base.
        self.r1[MAX_DIGITS] = 1;
        // Precompute left_s and left_c.
        let mut ls: usize = 0;
        let mut lc: usize = 0;
        // Loop from digit 9 down to 0.
        for i in (0..MAX_DIGITS).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            // Multiply the previous value in r1 by the number of ways to choose positions for cnt[i]
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i]][self.cnt[i]]) % MOD;
        }

        // Store n = length of input number.
        self.n = num.len();
        // Reset dp table to -1 is already done in new(), but if calling multiple times, ensure reset.
        for i in 0..MAX_DIGITS {
            for j in 0..MAX_SUM {
                for k in 0..MAX_COUNT {
                    self.dp[i][j][k] = -1;
                }
            }
        }

        // The balanced sum is s/2 and the balanced count of digits is n/2.
        let half_sum = s / 2;
        let half_count = self.n / 2;
        let ans = self.dfs(0, half_sum, half_count);
        ans as i32
    }
}

fn main() {
    // Prepare to read from standard input.
    let stdin = io::stdin();
    let mut input = String::new();
    // Read entire input from stdin.
    // The original C code expects one input string via scanf("%s", num)
    // but the problem statement mentions the input may consist of multiple lines.
    while let Ok(n) = stdin.lock().read_line(&mut input) {
        if n == 0 {
            break;
        }
    }
    let input = input.trim(); // remove whitespace around

    // Check input length according to C code requirements.
    if input.len() > MAX_NUM_LENGTH - 1 {
        println!(
            "Input too long, maximum allowed length is {}",
            MAX_NUM_LENGTH - 1
        );
        return;
    }

    let mut solver = Solver::new();
    let result = solver.count_balanced_permutations(input);
    // Print result in the exact same format (just the number followed by newline)
    println!("{}", result);
}