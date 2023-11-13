impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let count = nums.len();
        nums.retain(|&x| x != 0);
        nums.resize(count, 0);
    }
}