impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s_temp = s.as_str();
        let mut result: i32 = 0;
        let mut c: Vec<char> = vec![];

        for char in s_temp.chars()
        {
            match char {
                'M' => {
                    match c.pop() {
                    Some('C') => result += 800,
                    _ => result += 1000,
                    };
                    c.push(char);
                },
                'D' => {
                    match c.pop() {
                    Some('C') => result += 300,
                    _ => result += 500,
                    };
                    c.push(char);
                },
                'C' => {
                    match c.pop() {
                        Some('X') => result += 80,
                        _ => result += 100,
                    };
                    c.push(char);
                },
                'L' => {
                    match c.pop() {
                        Some('X') => result += 30,
                        _ => result += 50,
                    };
                    c.push(char);
                },
                'X' => {
                    match c.pop() {
                        Some('I') => result += 8,
                        _ => result += 10,
                    };
                    c.push(char);
                },
                'V' => {
                    match c.pop() {
                        Some('I') => result += 3,
                        _ => result += 5,
                    };
                    c.push(char);
                },
                'I' => {
                    result += 1;
                    c.push(char);
                }
                _ => (),
            }
        }

        return result;
    }
}