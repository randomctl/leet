fn main() {}

fn longest_palindrome_substr(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = s.chars().collect();

    let mut t: Vec<char> = Vec::with_capacity(chars.len() * 2 + 3);
    t.push('^');
    t.push('#');
    for &ch in &chars {
        t.push(ch);
        t.push('#');
    }
    t.push('$');

    let mut p = vec![0usize; t.len()];
    let mut center = 0usize;
    let mut right = 0usize;

    for i in 1..t.len() - 1 {
        let mirror = 2 * center as isize - i as isize;

        if i < right {
            let mirror_idx = mirror as usize;
            p[i] = p[mirror_idx].min(right - i);
        }

        while t[i + 1 + p[i]] == t[i - 1 - p[i]] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

    let (max_center, max_len) = p
        .iter()
        .enumerate()
        .max_by_key(|&(_, &radius)| radius)
        .map(|(idx, &radius)| (idx, radius))
        .unwrap();

    let start = (max_center - max_len) / 2;
    chars[start..start + max_len].iter().collect()
}
