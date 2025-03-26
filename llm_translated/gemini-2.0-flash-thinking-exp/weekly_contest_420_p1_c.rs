use std::io;
use std::process;

fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    for t in target.chars() {
        return_size += (t as u32) - ('a' as u32) + 1;
    }

    let mut ans: Vec<String> = Vec::with_capacity(return_size as usize);
    let mut l = 0;
    for t in target.chars() {
        for c_val in 'a' as u32..=(t as u32) {
            let c = char::from_u32(c_val).unwrap();
            let prefix = &target[0..l];
            let new_string = format!("{}{}", prefix, c);
            ans.push(new_string);
        }
        l += 1;
    }
    ans
}

fn main() {
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target = target.trim();

    let ans = string_sequence(target);

    for s in ans {
        print!("{} ", s);
    }
    println!();
}