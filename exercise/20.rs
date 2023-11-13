impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s_temp = s.as_str();
        let mut index: Vec<i32> = vec![];
        let mut i: usize = 0;

        for char in s_temp.chars()
        {
            match char {
                '(' => index.push(5),
                ')' => index.push(-5),
                '{' => index.push(7),
                '}' => index.push(-7),
                '[' => index.push(9),
                ']' => index.push(-9),
                _ => return true,
            }
            
            if index[i] < 0 {
                if i == 0 {
                    return false;
                } else if index[i] + index[i - 1] != 0 {
                    return false;
                } else {
                    index.pop();
                    index.pop();
                }
            }

            i = index.len() as usize;
        }
        if i != 0 {
            return false;
        }

        return true;
    }
}