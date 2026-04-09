fn main() {
    println!("{}", int_to_roman(3749i32));
    println!("{}", int_to_roman(58i32));
    println!("{}", int_to_roman(1994i32));
}

fn int_to_roman(num: i32) -> String {
    let array_5 = vec!["V", "L", "D"];
    let array_1 = vec!["I", "X", "C", "M"];
    let mut res: String = String::new();
    let mut curr_num: i32 = num;
    let power = num.ilog10();
    for i in 0..=power {
        let curr_power = power - i;
        let digit: i32 = curr_num / 10i32.pow(curr_power);
        curr_num = curr_num % 10i32.pow(curr_power);

        if digit == 4 {
            res.push_str(array_1[curr_power as usize]);
            res.push_str(array_5[curr_power as usize]);
        } else if digit == 9 {
            res.push_str(array_1[curr_power as usize]);
            res.push_str(array_1[curr_power as usize + 1 as usize]);
        } else {
            if digit >= 5 {
                res.push_str(array_5[curr_power as usize]);
                for _ in 0..digit-5 {
                    res.push_str(array_1[curr_power as usize]);
                }
            } else {
                for _ in 0..digit {
                    res.push_str(array_1[curr_power as usize]);
                }
            }
        }
    }
    res
}
