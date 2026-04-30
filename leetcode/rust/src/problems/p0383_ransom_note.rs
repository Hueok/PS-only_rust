pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = [0; 26];

        for c in magazine.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        for c in ransom_note.chars() {
            let idx = (c as u8 - b'a') as usize;

            if count[idx] == 0{
                return false
            }
            count[idx] -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1(){
        let ransomNote = String::from("a");
        let magazine= String::from("b");
        let result = Solution::can_construct(ransomNote, magazine);
        assert_eq!(result, false);
    }
    #[test]
    fn test_case_2(){
        let ransomNote = String::from("aa");
        let magazine= String::from("ab");
        let result = Solution::can_construct(ransomNote, magazine);
        assert_eq!(result, false);
    }
    #[test]
    fn test_case_3(){
        let ransomNote = String::from("aa");
        let magazine= String::from("aab");
        let result = Solution::can_construct(ransomNote, magazine);
        assert_eq!(result, true);
    }

}