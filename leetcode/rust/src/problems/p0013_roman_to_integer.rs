pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            }
        }
        let mut result = 0;
        let string: Vec<char> = s.chars().collect();
        let mut i = 0;

        loop {
            if i >= s.len()-1 {break;}
            if value(string[i]) < value(string[i+1]) {
                result += value(string[i+1]) - value(string[i]);
                i+=1;
            } else {
                result += value(string[i]);
            }
            i+=1;
        }
        if i<=s.len()-1{
            result += value(string[s.len()-1]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = String::from("III");
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_case_2() {
        let input = String::from("LVIII");
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 58);
    }
    #[test]
    fn test_case_3() {
        let input = String::from("MCMXCIV");
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 1994);
    }
}