fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", max_area(vec![1, 1]));
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut right_idx: usize = height.len() - 1;
    let mut left_idx: usize = 0;
    let mut width: i32 = height.len() as i32 - 1;
    let mut max_area: i32 = if height[right_idx] > height[left_idx] {
        width * height[left_idx]
    } else {
        width * height[right_idx]
    };

    for _ in 0..height.len() {
        if height[right_idx] > height[left_idx] {
            left_idx += 1;
        } else {
            right_idx = match right_idx.checked_sub(1) {
                Some(num) => num,
                None => {
                    return max_area;
                }
            };
        }

        width -= 1; // Width always goes down by 1

        if height[left_idx] > height[right_idx] {
            if max_area < (width * height[right_idx]) {
                max_area = width * height[right_idx];
            }
        } else {
            if max_area < (width * height[left_idx]) {
                max_area = width * height[left_idx];
            }
        }
    }

    max_area
}
