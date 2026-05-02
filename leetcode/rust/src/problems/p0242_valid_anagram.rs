pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return false;}

        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut freq: [i16; 26] = [0; 26];

        for i in 0..s.len() {
            freq[(s[i]-b'a') as usize] += 1;
            freq[(t[i]-b'a') as usize] -= 1;
        }

        for v in freq {
            if v != 0 {
                return false;
            }
        }

        true

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        let result = Solution::is_anagram(s, t);
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("rat");
        let t = String::from("car");
        let result = Solution::is_anagram(s, t);
        assert_eq!(result, false);
    }
}