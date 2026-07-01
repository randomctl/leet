fn main() {
    println!("{:?}", four_sum(vec![1, 0, -1, 0, -2, 2], 0));
}

fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums_clone = nums.clone();
    nums_clone.sort();
    let n = nums.len();
    let mut res: Vec<Vec<i32>> = Vec::new();

    for i in 0..n {
        if i > 0 && nums_clone[i] == nums_clone[i - 1] {
            continue;
        }

        if nums_clone[i] > target {
            break;
        }
        let mut l = i + 1;
        let mut r = n.saturating_sub(1);
        while l < r
    }
}
