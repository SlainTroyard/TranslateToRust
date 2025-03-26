```rust
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};

// Define the Solution struct to encapsulate our logic
struct Solution;

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut all = 0;
        let mut mask2 = 0;
        let mut g: Vec<Vec<i32>> = vec![vec![]; 26];

        for s in words {
            let x = (s.chars().next().unwrap() as i32) - ('a' as i32);
            let y = (s.chars().nth(1).unwrap() as i32) - ('a' as i32);
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x as usize].push(y);
        }

        // Function to determine if there's a cycle in the graph
        let has_cycle = |sub: i32| -> bool {
            let mut color = [0; 26];
            let mut dfs = |x: i32, dfs: &mut dyn FnMut(i32, &mut dyn FnMut(i32, &mut dyn FnMut(i32, &mut dyn FnMut(i32, &mut dyn FnMut<i32,i32...
   