fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    for &t in target.as_bytes() {
        return_size += (t - b'a' + 1) as usize;
    }

    let mut ans: Vec<String> = Vec::with_capacity(return_size);
    let mut l = 0;

    for &t in target.as_bytes() {
        for c in b'a'..=t {
            let mut s = String::with_capacity(l + 1);
            s.push_str(&target[0..l]);
            s.push(c as char);
            ans.push(s);
        }
        l += 1;
    }

    ans
}

fn main() {
    use std::io;

    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target = target.trim();

    let ans = string_sequence(target);

    for s in &ans {
        print!("{} ", s);
    }
    println!();
}