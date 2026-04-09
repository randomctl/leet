fn main() {
    println!("{}", atoi("+1".to_string()));
    println!("{}", atoi("-042".to_string()));
    println!("{}", atoi("1337c0d3".to_string()));
    println!("{}", atoi("0-1".to_string()));
    println!("{}", atoi("words and 987".to_string()));
}

fn atoi(s: String) -> i32 {
    let mut res = 0i32;

    let no_leading_whitespace = s.trim_start();
    let mut chars_iter = no_leading_whitespace.chars().peekable();
    let sign: i32 = match chars_iter.peek() {
        Some(&'-') => {
            chars_iter.next();
            -1
        },
        Some(&'+') => {
            chars_iter.next();
            1
        },
        _ => 1,
    };

    while let Some(c) = chars_iter.next() {
        match c.to_digit(10) {
            Some(digit) => {
                let next_num = match res.checked_mul(10) {
                    Some(num) => num,
                    None => {
                        if sign > 0 {
                            return 2i32.pow(31) - 1i32;
                        }
                        return -(2i32.pow(31));
                    }
                };
                res = match next_num.checked_add(digit as i32) {
                    Some(num) => num,
                    None => {
                        if sign > 0 {
                            return 2i32.pow(31) - 1i32;
                        }
                         return -(2i32.pow(31));
                    }
                };
            },
            None => {
                return res * sign;
            }
        }
    }
    res * sign
}
