impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums.iter().sum::<i32>();
        let mut previous_sum: i32 = 0;
        
        match nums.iter().position(|&x| {
            let current_sum: i32 = sum - x - previous_sum;
            if current_sum == previous_sum {
                true
            } else {
                previous_sum += x;
                false
            }
        }) {
            Some(y) => y as i32,
            None => (-1 as i32),
        }
    }
}