use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut values = HashMap::new();
        for (i, elem) in nums.iter().enumerate() {
            match values.get(elem) {
                Some(&index) => {
                    return vec![index, i as i32]
                },
                None => {
                    values.insert(target - *elem, i as i32);
                }
            }
        };
        panic!();
    }
}
