fn main() {
    println!("{}", is_palindrome( 121i32));
    println!("{}", is_palindrome( -121i32));
    println!("{}", is_palindrome( 10i32));
    println!("{}", is_palindrome( 1i32));
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut reversed = 0i32;

    while x != 0 {
        let digit: i32 = x % 10;
        reversed = (reversed * 10) + digit;
        x = x / 10;
    }

    reversed == x
}
