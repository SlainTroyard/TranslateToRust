fn main() {
    let mut date = String::new();
    std::io::stdin()
        .read_line(&mut date)
        .expect("Failed to read input");
    
    // Trim whitespace and newlines
    let date = date.trim();
    
    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}

fn convert_date_to_binary(date: &str) -> String {
    // Create a buffer to build our result
    // We'll build the result from right to left, then adjust at the end
    let mut result = vec![0u8; 25];
    let mut position = 24; // Start at the end of the buffer
    
    // Parse day (last 2 characters of the date string)
    let d = date[8..].parse::<i32>().unwrap_or(0);
    if d == 0 {
        position -= 1;
        result[position] = b'0';
    } else {
        let mut d_copy = d;
        while d_copy > 0 {
            position -= 1;
            result[position] = (d_copy & 1) as u8 + b'0';
            d_copy >>= 1;
        }
    }
    
    // Add separator
    position -= 1;
    result[position] = b'-';
    
    // Parse month (characters 5-6 of the date string)
    let m = date[5..7].parse::<i32>().unwrap_or(0);
    if m == 0 {
        position -= 1;
        result[position] = b'0';
    } else {
        let mut m_copy = m;
        while m_copy > 0 {
            position -= 1;
            result[position] = (m_copy & 1) as u8 + b'0';
            m_copy >>= 1;
        }
    }
    
    // Add separator
    position -= 1;
    result[position] = b'-';
    
    // Parse year (first 4 characters of the date string)
    let y = date[0..4].parse::<i32>().unwrap_or(0);
    if y == 0 {
        position -= 1;
        result[position] = b'0';
    } else {
        let mut y_copy = y;
        while y_copy > 0 {
            position -= 1;
            result[position] = (y_copy & 1) as u8 + b'0';
            y_copy >>= 1;
        }
    }
    
    // Convert the used portion of the buffer to a string
    let used_portion = &result[position..24];
    String::from_utf8(used_portion.to_vec()).unwrap()
}