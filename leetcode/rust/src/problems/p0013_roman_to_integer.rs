pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman: Vec<char> = s.chars().collect();
        let mut i = 0;

        let mut val = 0;
        let mut result = 0;
        loop {
            if i >= roman.len() {break;}
            if roman[i] == 'I' {
                val = 1;
                if i < roman.len()-1 {
                    match roman[i+1] {
                        'V' => {
                            val = 4;
                            i+=1;
                        },
                        'X' => {
                            val = 9;
                            i+=1;
                        },
                        _ => {}
                    }
                }
            } else if roman[i] == 'V' {
                val = 5;
            } else if roman[i] == 'X' {
                val = 10;
                if i<roman.len()-1 {
                    match roman[i+1] {
                        'L' => {
                            val = 40;
                            i+=1;
                        },
                        'C' => {
                            val = 90;
                            i+=1;
                        },
                        _ => {}
                    }
                }
            } else if roman[i] == 'L' {
                val = 50;
            } else if roman[i] == 'C' {
                val = 100;
                if i<roman.len()-1 {
                    match roman[i+1] {
                        'D' => {
                            val = 400;
                            i+=1;
                        },
                        'M' => {
                            val = 900;
                            i+=1;
                        },
                        _ => {}
                    }
                }
            } else if roman[i] == 'D' {
                val = 500;
            } else if roman[i] == 'M' {
                val = 1000;
            }
            result += val;
            i+=1;
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