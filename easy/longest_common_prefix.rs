fn main() {
    println!("{}", longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("")]));
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = 200;
    // Knowing that strs will have at least one String.
    for i in 0..strs.len() {
        let curr = common_prefix_len(strs[0].as_str(), strs[i].as_str());
        if curr == 0 {
            return String::new();
        }

        if curr < res {
            res = curr;
        }
    }
    String::from(&strs[0][0..res as usize])
}

fn common_prefix_len(s1: &str, s2: &str) -> u32 {
    let min_len = std::cmp::min(s1.len(), s2.len());
    for i in 0..min_len {
        if s1.as_bytes()[i] != s2.as_bytes()[i] {
            return i as u32; // Index value if pos 0 is different, return size is 0 and so on.
        }
    }
    min_len as u32
}
