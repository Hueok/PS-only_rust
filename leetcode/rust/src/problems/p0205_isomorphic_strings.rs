pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {

        let s = s.as_bytes();
        let t = t.as_bytes();
        
        let diff: i32 = s[0] as i32 -t[0] as i32;
        
        for i in 1..s.len() {
            if s[i] as i32 - t[i] as i32 != diff {
                return false;
            }
        }
        return true;

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("egg");
        let t = String::from("add");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("f11");
        let t = String::from("b23");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, false);
    }
    #[test]
    fn test_case_3() {
        let s = String::from("paper");
        let t = String::from("title");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }
}