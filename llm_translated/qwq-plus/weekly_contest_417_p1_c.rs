use std::io;

fn kth_character(k: i32) -> char {
    let k = k as usize;
    let mut l = vec![0; k + 1];
    let mut a = 0;
    loop {
        let j = 1 << a;
        for i in 0..(1 << a) {
            let pos = i + j;
            l[pos] = l[i] + 1;
            if pos >= k - 1 {
                return (b'a' + (l[k - 1] % 26) as u8) as char;
            }
        }
        a += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let k: i32 = input.trim().parse()
        .expect("Input must be an integer");
    println!("{}", kth_character(k));
}