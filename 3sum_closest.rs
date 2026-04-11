fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums_clone = nums.clone();
    nums_clone.sort();
    let n = nums.len();
    let mut res: i32 = 10i32.pow(5) + 1;
    for i in 0..n {
        // Skip duplicate first values
        if i > 0 && nums_clone[i] == nums_clone[i - 1] {
            continue;
        }

        let mut l = i + 1;
        let mut r = n.saturating_sub(1);
        while l < r {
            let sum = nums_clone[i] + nums_clone[l] + nums_clone[r];
            if sum == target {
                return sum;
            }

            if target.abs_diff(sum) < target.abs_diff(res) {
                res = sum;
            }

            if sum < target {
                l += 1;
            } else if sum > target {
                r -= 1;
            }
        }
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![-1, 2, 1, -4];
        assert_eq!(three_sum_closest(arr, 1), 2);
    }

    #[test]
    fn test_2() {
        let arr = vec![0, 0, 0];
        assert_eq!(three_sum_closest(arr, 1), 0);
    }

    #[test]
    fn test_3() {
        let arr = vec![1, 1, 1, 1];
        assert_eq!(three_sum_closest(arr, 0), 3);
    }
}
