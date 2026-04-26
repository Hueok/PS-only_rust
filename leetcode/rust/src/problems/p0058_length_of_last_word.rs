pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut words: Vec<_> = s.split_whitespace().collect();
        let size = words[words.len()-1].len();
        size as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = String::from("Hello World");
        let result = Solution::length_of_last_word(input);
        assert_eq!(result, 5);
    }
}