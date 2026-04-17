pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i+=1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums = vec![3,2,2,3];
        let result = Solution::remove_element(&mut nums, 3);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_case_2() {
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let result = Solution::remove_element(&mut nums, 2);
        assert_eq!(result, 5);
    }
}