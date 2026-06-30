fn main() {
    println!("{:?}", letter_combinations("23".to_string()));
    println!("{:?}", letter_combinations("234".to_string()));
}

fn letter_combinations(digits: String) -> Vec<String> {
    let mut map: std::collections::HashMap<char, Vec<char>> = std::collections::HashMap::new();
    map.insert('2', vec!['a', 'b', 'c']);
    map.insert('3', vec!['d', 'e', 'f']);
    map.insert('4', vec!['g', 'h', 'i']);
    map.insert('5', vec!['j', 'k', 'l']);
    map.insert('6', vec!['m', 'n', 'o']);
    map.insert('7', vec!['p', 'q', 'r', 's']);
    map.insert('8', vec!['t', 'u', 'v']);
    map.insert('9', vec!['w', 'x', 'y', 'z']);

    let mut res = Vec::new();
    let digits_chars: Vec<char> = digits.chars().collect();
    let mut curr = String::new();
    backtrack(0, &mut curr, &mut res, &map, &digits_chars);
    res
}

fn backtrack(
    index: usize,
    curr: &mut String,
    res: &mut Vec<String>,
    map: &std::collections::HashMap<char, Vec<char>>,
    digits_as_char: &Vec<char>,
) {
    if index == digits_as_char.len() {
        res.push(curr.to_string());
        return;
    }

    let digit = digits_as_char[index];
    let options = map.get(&digit).unwrap();
    for &option in options {
        curr.push(option);
        backtrack(index + 1, curr, res, map, digits_as_char);
        curr.pop();
    }
}
