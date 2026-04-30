pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0{return true;}
        let mut i=0;
        let mut j =0;

        let s = s.as_bytes();
        let t = t.as_bytes();

        while j < t.len() && i < s.len() {
            if s[i] == t[j] {
                i+=1;
            }
            j+=1;
        }

        return i == s.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, false);
    }
}