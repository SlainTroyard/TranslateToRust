fn to_binary_string(x: i32) -> String {
    let binary_string = format!("{:b}", x);
    let trimmed_string = binary_string.trim_start_matches('0');
    if trimmed_string.is_empty() {
        "0".to_string() // Handle the case where x is 0, bitset returns all zeros which after trim becomes empty
    } else {
        trimmed_string.to_string()
    }
}

struct Solution {}

impl Solution {
    fn convert_date_to_binary(&self, date: String) -> String {
        let year_str = &date[0..4];
        let month_str = &date[5..7];
        let day_str = &date[8..10];

        let year = year_str.parse::<i32>().unwrap();
        let month = month_str.parse::<i32>().unwrap();
        let day = day_str.parse::<i32>().unwrap();

        format!("{}-{}-{}", to_binary_string(year), to_binary_string(month), to_binary_string(day))
    }
}

fn main() {
    use std::io;

    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();
    let date = date.trim(); // Remove trailing newline

    let sol = Solution {};
    println!("{}", sol.convert_date_to_binary(date.to_string()));
}