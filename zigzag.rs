fn main() {
    let input = String::from("A");
    println!("{}", convert(input, 1i32));
}

fn convert(s: String, num_rows: i32) -> String {
    let cycle = 2 * (num_rows - 1);
    let capacity = s.len();
    let mut res: String = String::with_capacity(capacity);
    let mut first_hop: i32 = cycle;
    let mut second_hop: i32 = 0;
    if num_rows == 1 {
        return s;
    }
    for idx in 0..num_rows {
        let mut i = idx as usize;
        let mut is_first_hop: bool = true;
        while i < capacity {
            res.push_str(&s[i..i + 1]);
            if first_hop == 0 || second_hop == 0 {
                i += cycle as usize;
            } else if is_first_hop {
                i += first_hop as usize;
                is_first_hop = false;
            } else {
                i += second_hop as usize;
                is_first_hop = true;
            }
        }
        first_hop -= 2;
        second_hop += 2;
    }
    return res;
}
