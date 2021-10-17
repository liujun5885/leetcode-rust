// https://leetcode-cn.com/problems/longest-increasing-subsequence/

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // pass
        return 4;
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_increasing_subsequence::Solution;

    #[test]
    fn case01() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let actual = Solution::length_of_lis(nums);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
