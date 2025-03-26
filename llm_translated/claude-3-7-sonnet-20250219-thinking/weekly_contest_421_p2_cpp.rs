use std::io;

struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut cnt = [0u64; 26];
        
        // Count frequency of each character
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        
        // Perform transformations for t iterations
        for _ in 1..=t {
            let mut nxt = [0u64; 26];
            // Shift all characters to the next one (a->b, b->c, etc.)
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            // Special case for 'z': transforms to 'a' and adds a bonus 'b'
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            
            // Update counts for next iteration
            cnt = nxt;
        }
        
        // Sum up all character counts
        let ans = cnt.iter().sum::<u64>() % MOD;
        ans as i32
    }
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();
    
    // Read string s and integer t from stdin
    io::stdin().read_line(&mut s).unwrap();
    io::stdin().read_line(&mut t).unwrap();
    
    let s = s.trim().to_string();
    let t: i32 = t.trim().parse().unwrap();
    
    let solution = Solution;
    println!("{}", solution.length_after_transformations(s, t));
}