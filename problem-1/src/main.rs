fn main() {
    println!("Hello, world!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for (index, num) in nums.iter().enumerate() {
        for (inner_index, inner_num) in nums.iter().enumerate() {
            if index != inner_index {
                if num + inner_num == target {
                    result = vec![inner_index as i32, index as i32];
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(two_sum(vec!(3, 2, 3), 6), vec!(2, 0));
    }
}
