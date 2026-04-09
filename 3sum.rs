fn main() {
    println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
    println!("{:?}", three_sum(vec![0, 1, 1]));
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    vec![vec![]]
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
        assert_eq!(three_sum(arr), []);
    }

    #[test]
    fn test_3() {
        let arr = vec![0, 0, 0];
        assert_eq!(three_sum(arr), [[0, 0, 0]]);
    }
}
