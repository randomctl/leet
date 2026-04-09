fn main() {
    let ex1 = find_uniq_bin_str(vec!["01".to_owned(), "10".to_owned()]);
    println!("{}", ex1);
    let ex2 = find_uniq_bin_str(vec!["00".to_owned(), "01".to_owned()]);
    println!("{}", ex2);
    let ex3 = find_uniq_bin_str(vec!["111".to_owned(), "011".to_owned(), "001".to_owned()]);
    println!("{}", ex3);
}

fn find_uniq_bin_str(nums: Vec<String>) -> String {
    nums.iter()
        .enumerate()
        .map(|(idx, val)| {
            if val.as_bytes()[idx] == b'0' {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}
