fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    if input.len() < 2 {
        println!("0");
        return;
    }
    
    let mut s = input.chars()
        .map(|c| c.to_digit(10).expect("Input must be digits"))
        .collect::<Vec<u32>>();
    
    let mut len = s.len();
    while len > 1 {
        len -= 1;
        if len <= 1 {
            break;
        }
        for i in 0..len {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
    }
    
    println!("{}", if s[0] == s[1] { 1 } else { 0 });
}