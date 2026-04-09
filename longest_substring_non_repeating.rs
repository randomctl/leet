fn main() {
    println!("{}", longest_substring_non_repeating("au".to_owned()));
}

fn longest_substring_non_repeating(s: String) -> i32 {
    let mut last_seen_at: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    s.chars()
        .fold((0, 0, 0), |(mut curr_len, mut start, mut end), ch| {
            let curr_str = &s[start..end];
            println!("Evaluating: {}", curr_str);
            if last_seen_at.contains_key(&ch) && start <= last_seen_at[&ch] && last_seen_at[&ch] <= end {
                start = last_seen_at[&ch] + 1;
                println!("Duplicate char: {}. Next start index: {}", ch, start);
            } else {
                if curr_len < end - start {
                    curr_len = end - start;
                    println!("New max len: {}", curr_len);
                }
            }
            last_seen_at.insert(ch, end);
            end += 1;
            (curr_len, start, end)
        })
        .0 as i32
        + 1i32
}
