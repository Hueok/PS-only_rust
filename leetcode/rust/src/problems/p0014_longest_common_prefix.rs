pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        
        let mut i = 0;
        loop {
            let mut prefix = strs[0].as_bytes[i];
            if strs.iter().all(|&s| s.as_bytes()[i] == prefix) {
                result.push_str(prefix);
            } else{
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let result = Solution::longest_common_prefix(input);
        assert_eq!(result, "fl");
    }
}