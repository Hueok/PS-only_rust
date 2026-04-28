pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        let mut strs = strs.clone();
        strs.sort_unstable();

        for i in 0..(strs[0].len().min(strs[strs.len()-1].len())){
            if strs[0].as_bytes()[i] as char != strs[strs.len()-1].as_bytes()[i] as char {
                break;
            }
            result.push_str(&(strs[0].as_bytes()[i] as char).to_string());
        }
        result

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