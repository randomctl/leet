fn main() {

}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut existing_values = std::collections::HashMap::new();
    for (idx, val) in nums.iter().enumerate() {
        if *val < target {
            let diff = target - val;
            match existing_values.get(&diff) {
                Some(diff_idx) => {
                    return vec![*diff_idx, idx as i32];
                }
                None => {
                    existing_values.insert(val, idx as i32);
                }
            }
        }
    }
    return vec![];
}
