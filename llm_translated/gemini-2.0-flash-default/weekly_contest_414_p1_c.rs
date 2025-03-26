use std::io;
use std::io::Read;
use std::mem::MaybeUninit;

fn convert_date_to_binary(date: &str) -> String {
    let d = date[8..].parse::<i32>().unwrap();
    let m = date[5..7].parse::<i32>().unwrap();
    let y = date[0..4].parse::<i32>().unwrap();

    let mut ans: [u8; 25] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut ptr = 24;

    let mut write_binary = |mut val: i32, ptr_mut: &mut usize| {
        if val == 0 {
            *ptr_mut -= 1;
            ans[*ptr_mut] = b'0';
        } else {
            while val > 0 {
                *ptr_mut -= 1;
                ans[*ptr_mut] = (val & 1) as u8 + b'0';
                val >>= 1;
            }
        }
    };

    write_binary(y, &mut ptr);
    ptr -= 1;
    ans[ptr] = b'-';

    write_binary(m, &mut ptr);
    ptr -= 1;
    ans[ptr] = b'-';

    write_binary(d, &mut ptr);

    let len = 25 - ptr;
    let mut result = String::from_utf8(ans[ptr..25].to_vec()).unwrap();
    result.shrink_to_fit();
    result
}

fn main() {
    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();
    let date = date.trim();

    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}