use std::io;

struct Solution;

impl Solution {
    fn string_sequence(&self, target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        
        for c in target.chars() {
            let new_s = s.clone() + "a";
            
            let start = 'a' as u8;
            let end = c as u8;
            
            for j in start..=end {
                let j_char = j as char;
                let mut temp = new_s.clone();
                
                if !temp.is_empty() {
                    let len = temp.len();
                    temp.replace_range(len - 1..len, &j_char.to_string());
                }
                
                ans.push(temp);
            }
            
            // Update s to be new_s with last character as c
            let mut s_final = new_s.clone();
            if !s_final.is_empty() {
                let len = s_final.len();
                s_final.replace_range(len - 1..len, &c.to_string());
            }
            s = s_final;
        }
        
        ans
    }
}

fn main() {
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read input");
    let target = target.trim().split_whitespace().next().unwrap_or_default().to_string();
    
    let solution = Solution;
    let ans = solution.string_sequence(target);
    
    for s in ans {
        print!("{} ", s);
    }
    println!();
}