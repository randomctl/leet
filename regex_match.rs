fn main() {}

fn is_match(s: String, p: String) -> bool {
    let mut s_iter = s.chars();
    let mut p_iter = p.chars().peekable();
    while let Some(pattern) = p_iter.next() {
        match p_iter.peek() {
            Some(&'*') => {
                // If next char is *, then iterate S while it matches the pattern.
                p_iter.next(); // Advance p_iter to skip
                if pattern == '.' { // if pattern is any
                    match p_iter.peek() {
                        Some(&ch) => { // exists more values after
                            while let Some(s_char) = s_iter.next() {
                            }
                        },
                        None => {
                            return true;
                        }
                    }
                }
                match s_iter.next() {
                    Some(ch) => {
                        if pattern == '.' {

                        }
                    },
                    None => {}
                }
            },
            Some(&other) => {
                if other != s_iter.next() {
                    return false;
                }

            },
            None => {
                if
            }
        }
    }
}
