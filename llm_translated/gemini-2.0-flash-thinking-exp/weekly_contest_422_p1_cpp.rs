fn is_balanced(num: &str) -> bool {
    let mut even_sum = 0;
    let mut odd_sum = 0;

    for (i, digit_char) in num.chars().enumerate() {
        // Convert char to digit
        let digit = digit_char.to_digit(10).unwrap() as i32; // Assuming valid input digits

        if i % 2 == 0 {
            even_sum += digit;
        } else {
            odd_sum += digit;
        }
    }

    even_sum == odd_sum
}

fn main() {
    use std::io;

    let mut num = String::new();

    // Input the number as a string from stdin
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num = num.trim(); // Remove trailing newline

    // Call the is_balanced function
    let result = is_balanced(&num);

    // Output the result to stdout
    if result {
        println!("true");
    } else {
        println!("false");
    }
}