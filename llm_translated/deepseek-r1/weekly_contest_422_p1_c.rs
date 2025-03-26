// Check if a number is balanced according to the problem's criteria
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        let digit = c.to_digit(10).expect("Invalid digit");
        total += -(digit as i32); // Equivalent to '0' - c in C
        total = -total; // Invert the sign after each digit
    }
    total == 0
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // Read the first whitespace-separated token, matching the original C code's scanf("%s", num)
    let num = input.split_whitespace().next().unwrap_or("");
    if is_balanced(num) {
        println!("true");
    } else {
        println!("false");
    }
}