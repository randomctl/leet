fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_clone = nums.clone();
    nums_clone.sort();
    let n = nums.len();
    let mut res: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        // Skip duplicate first values
        if i > 0 && nums_clone[i] == nums_clone[i - 1] {
            continue;
        }
        // Optional early stop: once nums[i] > 0, sum can't be 0
        if nums_clone[i] > 0 {
            break;
        }
        let mut l = i + 1;
        let mut r = n.saturating_sub(1);
        while l < r {
            let sum = nums_clone[i] + nums_clone[l] + nums_clone[r];
            if sum < 0 {
                l += 1;
            } else if sum > 0 {
                r -= 1;
            } else {
                res.push(vec![nums_clone[i], nums_clone[l], nums_clone[r]]);
                l += 1;
                r -= 1;
                // Skip duplicates for left pointer
                while l < r && nums_clone[l] == nums_clone[l - 1] {
                    l += 1;
                }
                // Skip duplicates for right pointer
                while l < r && nums_clone[r] == nums_clone[r + 1] {
                    r -= 1;
                }
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
        let arr = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(arr), [[-1, -1, 2], [-1, 0, 1]]);
    }

    #[test]
    fn test_2() {
        let arr = vec![0, 1, 1];
        assert_eq!(three_sum(arr), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_3() {
        let arr = vec![0, 0, 0];
        assert_eq!(three_sum(arr), [[0, 0, 0]]);
    }

    #[test]
    fn test_4() {
        let arr = vec![0, 0, 0, 0];
        assert_eq!(three_sum(arr), [[0, 0, 0]]);
    }
}
