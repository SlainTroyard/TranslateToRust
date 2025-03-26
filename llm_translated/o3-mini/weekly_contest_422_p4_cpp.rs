use std::io::{self, BufRead, Write};
use std::process;

// Constant modulo value
const MOD: i64 = 1_000_000_007;

// Solver struct encapsulating all necessary arrays and values.
struct Solver {
    dp: Vec<Vec<Vec<i64>>>,    // 3D DP: dp[i][s][c] with dimensions [10][721][81]
    cb: Vec<Vec<i64>>,         // Combination table, dimensions: [81][81]
    cnt: [usize; 10],          // Count of each digit 0-9
    left_s: [usize; 10],       // Cumulative digit sum starting from digit i to 9
    left_c: [usize; 10],       // Cumulative digit count starting from digit i to 9
    r1: [i64; 11],             // r1 array of size 11 for base cases
    n: usize,                  // Length of input string
}

impl Solver {
    // Create a new Solver given the input string.
    fn new(num: &str) -> Self {
        // Allocate dp[10][721][81] initialized with -1.
        let dp = vec![vec![vec![-1; 81]; 721]; 10];
        // Allocate cb as 81 x 81 table filled initially with 0.
        let cb = vec![vec![0; 81]; 81];
        Self {
            dp,
            cb,
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            r1: [0; 11],
            n: 0,
        }
    }

    // Build Pascal's triangle for combinations modulo MOD.
    fn build_pascal(&mut self) {
        // Initialize the combination table.
        // cb[i][j] will hold C(i, j) mod MOD for i, j in 0..=80.
        self.cb[0][0] = 1;
        for i in 1..=80 {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % MOD;
            }
        }
    }

    // The DFS function performing the DP computation recursively.
    // i: current digit index (0-9)
    // s: remaining sum (should be even ultimately, we work with half sum)
    // c: remaining count (should be half of total count)
    fn dfs(&mut self, i: usize, s: usize, c: usize) -> i64 {
        // Base: when no sum or count remains, return r1[i]
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        // If we have processed all digits, return 0 (invalid state)
        if i == 10 {
            return 0;
        }
        // If remaining sum or count is greater than what is possible, return 0.
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }
        // If already computed, return cached value.
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }
        let mut res = 0;
        // For x from 0 to count of digit i:
        // x: number of digits chosen to contribute to one side.
        // y: the remaining count (implicitly cnt[i] - x) for the other side.
        for x in 0..=self.cnt[i] {
            // If we don't have enough count available, break.
            if c < x {
                break;
            }
            // Compute the new remaining sum after choosing x copies of digit i.
            // Using isize to check for negativity.
            let a_val = s as isize - (i as isize * x as isize);
            if a_val < 0 {
                break; // further increasing x will only decrease a_val further.
            }
            let y = self.cnt[i] - x;
            // Validate that there is enough capacity in the remaining count for y.
            if y > self.left_c[i] - c {
                continue;
            }
            // Multiply with combination factor: choose x of c positions.
            let ways_choose_x = self.cb[c][x];
            // Recursive call: process next digit i+1 with updated remaining sum and count.
            let next = self.dfs(i + 1, a_val as usize, c - x);
            // Multiply result with ways_choose_x.
            let b = (next * ways_choose_x) % MOD;
            // Multiply b with ways to assign y into the remaining positions.
            res = (res + b * self.cb[self.left_c[i] - c][y]) % MOD;
        }
        // Memoize the result.
        self.dp[i][s][c] = res;
        res
    }

    // Main function to count balanced permutations.
    // Returns result as computed, modulo MOD.
    fn count_balanced_permutations(&mut self, num: &str) -> i64 {
        // Compute frequency of each digit and total sum.
        let mut total_sum: usize = 0;
        // Ensure we are iterating over the trimmed input string.
        for ch in num.trim().chars() {
            if let Some(digit) = ch.to_digit(10) {
                let d = digit as usize;
                total_sum += d;
                self.cnt[d] += 1;
            }
        }
        // If total sum is odd, return 0 immediately.
        if total_sum % 2 != 0 {
            return 0;
        }
        // Build the combination table.
        self.build_pascal();

        // Pre-calculate r1 and cumulative left_s and left_c.
        // r1[10] is set to 1.
        self.r1[10] = 1;
        let mut ls = 0; // cumulative sum
        let mut lc = 0; // cumulative count
        // Iterate backwards from digit 9 down to 0.
        for i in (0..10).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            // Multiply with the number of ways to choose positions for the cnt[i] digits among lc places.
            self.r1[i] = (self.r1[i + 1] * self.cb[lc][self.cnt[i]]) % MOD;
        }
        // Set n to the length of num.
        self.n = num.trim().len();
        // Clear the DP table by re-initializing (it was already initialized in new()).
        // Start the DFS with half of total sum and half of total count.
        self.dfs(0, total_sum / 2, self.n / 2)
    }
}

fn main() {
    // Setup buffered reader for input.
    let stdin = io::stdin();
    let input = stdin.lock().lines();

    // We need to read a single token/string from the input.
    // The original C++ code uses "cin >> num" so we take the first non-empty token.
    let mut token = String::new();
    for line in input {
        if let Ok(line_str) = line {
            for word in line_str.split_whitespace() {
                token = word.to_string();
                break;
            }
        }
        if !token.is_empty() {
            break;
        }
    }

    // Check maximum allowed length.
    const MAX_LENGTH: usize = 80;
    if token.len() > MAX_LENGTH {
        // Write error message to stdout.
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        process::exit(1);
    }

    // Create a new Solver instance.
    let mut solver = Solver::new(&token);
    // Compute the balanced permutation count.
    let result = solver.count_balanced_permutations(&token);
    // Print the result.
    println!("{}", result);
}