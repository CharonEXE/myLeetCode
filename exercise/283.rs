impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let count = nums.len();
        let mut left = 0;
        let mut i = 0;
        while i < count - 1 {
            if count - i - 1 > left {
                match nums[i] {
                    0 => {
                        nums.remove(i);
                        nums.push(0);
                        i -= 1;
                        left += 1;
                    },
                _ => (),
                }
            }
            i += 1;
        }
    }
}