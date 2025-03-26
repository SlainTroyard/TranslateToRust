fn has_same_digits(s: &mut String) -> bool {
    let len = s.len();
    for i in 0..len {
        // Convert characters to their numeric values by masking with 0x0f (equivalent to subtracting '0')
        s.as_bytes_mut()[i] &= 0x0f;
    }

    let mut len = len; // shadow outer len because it's modified

    while len > 1 {
        for i in 0..len - 1 {
            // Note: Indices and mutability are handled carefully here.
            let a = s.as_bytes()[i] as u32;
            let b = s.as_bytes()[i + 1] as u32;
            s.as_bytes_mut()[i] = ((a + b) % 10) as u8;
        }
        len -= 1;
    }

    s.as_bytes()[0] == s.as_bytes()[1]
}

fn main() {
    use std::io;

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();

    let result = has_same_digits(&mut s);
    println!("{}", if result { 1 } else { 0 });
}