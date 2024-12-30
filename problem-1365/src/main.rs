use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_hashmap: HashMap<i32, usize> = HashMap::new();
    let mut nums_sorted = nums.clone();
    nums_sorted.sort();
    for (index, num) in nums_sorted.iter().enumerate() {
        if !nums_hashmap.contains_key(num) {
            nums_hashmap.insert(*num, index);
        }
    }

    let result: Vec<i32> = nums
        .into_iter()
        .map(|num| {
            if let Some(index) = nums_hashmap.get(&num) {
                *index as i32
            } else {
                0
            }
        })
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(
            smaller_numbers_than_current(vec!(6, 5, 4, 8)),
            vec!(2, 1, 0, 3)
        );
        assert_eq!(
            smaller_numbers_than_current(vec!(7, 7, 7, 7)),
            vec!(0, 0, 0, 0)
        );
    }
}
