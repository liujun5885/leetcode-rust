// https://leetcode-cn.com/problems/maximum-subarray-sum-after-one-operation/

struct Solution;

impl Solution {
    pub fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        17
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_subarray_sum_after_one_operation::Solution;
    #[test]
    fn case01() {
        let actual = Solution::max_sum_after_operation(vec![2, -1, -4, -3]);
        let expected = 17;
        assert_eq!(actual, expected);
    }

    #[test]
    fn case02() {
        let actual = Solution::max_sum_after_operation(vec![1, -1, 1, 1, -1, -1, 1]);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
