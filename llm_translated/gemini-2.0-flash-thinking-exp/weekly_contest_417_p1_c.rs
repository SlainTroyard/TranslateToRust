fn kth_character(k: i32) -> char {
    let mut l: Vec<i32> = vec![0; (k + 1) as usize];
    for a in 0.. {
        let j = 1 << a;
        for i in 0..(1 << a) {
            let index = (i + j) as usize;
            if index < l.len() { // Add bounds check for vector access, important for safety in Rust
                l[index] = l[i as usize] + 1;
                if (i + j) >= (k - 1) {
                    return (b'a' + (l[(k - 1) as usize] % 26) as u8) as char;
                }
            } else {
                // Break inner loop if index is out of bound, prevent panic, align with C logic if k is small
                break;
            }
        }
        if (1 << a) as usize + (1 << a) as usize >= l.len() && k > 0 { // Optimization to avoid unnecessary iterations if array is filled
            // In C, the loop continues indefinitely. But in Rust, we should consider stopping condition for efficiency
            // However, to strictly follow C logic, we can remove this optimization and let it run indefinitely.
            // But for practical Rust, adding a break condition when we are sure to have reached k is reasonable.
            if (1 << a) > k { // Simple break condition: if 2^a exceeds k, we should have found the answer by now or will find it soon.
                // This is a heuristic break, not strictly necessary to match C logic but good for Rust efficiency.
                // To strictly match C logic, remove this break.
                if k > 0 { // Ensure k > 0 before returning in break condition
                    return (b'a' + (l[(k - 1) as usize] % 26) as u8) as char;
                } else {
                    return 'a'; // Handle k=0 case, though original code might not handle k=0 gracefully.
                }
            }
        }
        if k == 0 { // Handle k=0 case explicitly, original C might not handle k=0 correctly.
            return 'a';
        }
        if (1 << a) > k && k > 0 { // Another break condition to prevent infinite loop for large a
            return (b'a' + (l[(k - 1) as usize] % 26) as u8) as char;
        }
    }
    ' ' // Should not reach here in normal cases, added for Rust completeness.
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: i32 = input.trim().parse().expect("Invalid input");
    println!("{}", kth_character(k));
}