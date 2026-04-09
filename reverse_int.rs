fn main() {
    println!("{}", reverse(123i32));
    println!("{}", reverse(-123i32));
    println!("{}", reverse(120));
}

fn reverse(x: i32) -> i32 {
    let mut curr = x;
    let mut new: i32 = 0;
    while curr != 0 {
        let next_digit = curr % 10;
        curr = curr / 10;
        let next = match new.checked_mul(10) {
            Some(i) => i,
            None => {
                return 0i32;
            }
        };
        new = match next.checked_add(next_digit) {
            Some(n) => n,
            None => {
                return 0i32;
            }
        }
    }
    new
}
