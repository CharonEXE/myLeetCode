impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let counter: i32 = 0b0000;
         
        // fold is powerful tool
        
        // After iterated through the array,
        // provide the initial result value, [the 0]
        // apply the tag to all iterated numbers [the |curent, new|]
        // apply the function/calculation on the tagged numbers
        // return the first tagged value.
        return nums.into_iter().fold(0, |current, new| current ^ new);
    }
}