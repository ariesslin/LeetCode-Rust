use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let j = target - nums[i];
        if hm.contains_key(&j) {
            return vec![*hm.get(&j).unwrap(), i as i32];
        } else {
            hm.insert(*num, i as i32);
        }
    }
    vec![-1, -1]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_indices_of_the_two_numbers_such_that_they_add_up_to_target() {
        let vec_of_interger = vec![2, 7, 11, 15];
        let target = 9;

        let result = two_sum(vec_of_interger, target);

        assert_eq!(result, vec![0, 1]);
    }
}
