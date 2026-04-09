fn main() {
    println!("{}", roman_to_int("III".to_owned()));
    println!("{}", roman_to_int("LVIII".to_owned()));
    println!("{}", roman_to_int("MCMXCIV".to_owned()));
}

fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let mut is_prev_less = 0;
    let mut it = s.chars();
    while let Some(ch) = it.next() {
        match ch {
            'I' => {
                is_prev_less = 1;
                res += 1;
            }
            'V' => {
                if is_prev_less == 1 {
                    res += 3;
                    is_prev_less = 0;
                } else {
                    res += 5;
                }
            }
            'X' => {
                if is_prev_less == 1 {
                    res += 8;
                    is_prev_less = 0;
                } else {
                    is_prev_less = 10;
                    res += 10;
                }
            }
            'L' => {
                if is_prev_less == 10 {
                    res += 30;
                    is_prev_less = 0;
                } else {
                    res += 50;
                }
            }
            'C' => {
                if is_prev_less == 10 {
                    res += 80;
                    is_prev_less = 0;
                } else {
                    is_prev_less = 100;
                    res += 100;
                }
            }
            'D' => {
                if is_prev_less == 100 {
                    res += 300;
                    is_prev_less = 0;
                } else {
                    res += 500;
                }
            }
            'M' => {
                if is_prev_less == 100 {
                    res += 800;
                    is_prev_less = 0;
                } else {
                    res += 1000;
                }
            }
            _ => {}
        }
    }
    res
}
