use std::collections::BTreeSet;
use std::io::{self, BufRead, Write};

const MOD: u64 = 1070777777;

struct HashSet {
    data: BTreeSet<u32>,
}

impl HashSet {
    fn new() -> Self {
        HashSet {
            data: BTreeSet::new(),
        }
    }

    fn add(&mut self, hash: u32) {
        self.data.insert(hash);
    }

    fn contains(&self, hash: u32) -> bool {
        self.data.contains(&hash)
    }
}

fn get_random_base() -> u64 {
    800000000 + (rand::random::<u64>() % 100000000)
}

fn min_valid_strings(words: &Vec<String>, target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

    let base = get_random_base();

    // Precompute powers of the base modulo MOD
    let mut pow_base = vec![1; n + 1];
    for i in 1..=n {
        pow_base[i] = (pow_base[i - 1] * base) % MOD;
    }

    // Compute prefix hashes for the target string
    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD;
    }

    // Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet> = vec![HashSet::new(); max_len];

    // Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0;
        for (j, &c) in word.as_bytes().iter().enumerate().take(len) {
            h = (h * base + c as u64) % MOD;
            sets[j].add(h as u32);
        }
    }

    // Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from the current position i
        while nxt_r < n && (nxt_r - i) < max_len {
            if nxt_r + 1 > n {
                break;
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as u32;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len {
                break;
            }

            // Binary search in the sorted HashSet for this prefix length
            if sets[prefix_len].contains(sh) {
                nxt_r += 1;
            } else {
                break;
            }
        }

        // If we've reached the end of the current bridge, update the bridge
        if i == cur_r {
            if i == nxt_r {
                return -1;
            }
            cur_r = nxt_r;
            ans += 1;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    let words_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }

    let target = lines.next().unwrap().unwrap();

    let res = min_valid_strings(&words, &target);
    writeln!(stdout, "{}", res).unwrap();
}