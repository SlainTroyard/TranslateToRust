use std::io;

fn has_same_digits(s: &mut [u8]) -> bool {
    let mut len = s.len();
    let mut i = 0;
    
    // Convert ASCII digits to their numeric values by masking with 0x0f
    while i < len {
        s[i] &= 0x0f;
        i += 1;
    }
    
    // Repeatedly apply transformation until only two digits remain
    while {
        len -= 1;
        len > 1
    } {
        for i in 0..len {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
    }
    
    // Check if the two remaining digits are equal
    s[0] == s[1]
}

fn main() {
    // Read input string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Convert input to byte array and remove trailing newline
    let mut bytes = input.trim().as_bytes().to_vec();
    
    // Call the function and print the result (1 for true, 0 for false)
    let result = has_same_digits(&mut bytes);
    println!("{}", if result { 1 } else { 0 });
}