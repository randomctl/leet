fn main () {
    println!("{}", is_valid(String::from("()")));
    println!("{}", is_valid(String::from("()[]{}")));
    println!("{}", is_valid(String::from("(]")));
    println!("{}", is_valid(String::from("([])")));
    println!("{}", is_valid(String::from("([)]")));
}

fn is_valid(s: String) -> bool {
    let mut queue: Vec<char> = Vec::new();
    let mut it = s.chars();
    while let Some(curr) = it.next() {
        match curr {
            '(' => queue.push(')'),
            '{' => queue.push('}'),
            '[' => queue.push(']'),
            _ => {
                if let Some(ch) = queue.pop() {
                    if ch != curr {
                        return false;
                    } else {
                        return false;
                    }
                }
            }
        }
    }
    queue.is_empty()
}
