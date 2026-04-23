pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;
        for i in 1..prices.len(){
            if prices[i] < buy {
                buy = prices[i];
            } else {
                profit = profit.max(prices[i]-buy);
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let prices = vec![7,1,5,3,6,4];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 5);
    }
    #[test]
    fn test_case_2() {
        let prices = vec![7,6,4,3,1];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 0);
    }
    #[test]
    fn test_case_3() {
        let prices = vec![4,1,2];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_case_4() {
        let prices = vec![4,7,1,2];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 3);
    }
}