use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_hashmap: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    for (index, num) in nums.iter().enumerate() {
        let difference = target - num;
        if let Some(another_index) = nums_hashmap.get(&difference) {
            result = vec![index as i32, *another_index as i32];
        } else {
            nums_hashmap.insert(*num, index);
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
