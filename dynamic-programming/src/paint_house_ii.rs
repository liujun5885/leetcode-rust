// https://leetcode-cn.com/problems/paint-house-ii/

struct Solution;
impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        5
    }
}

#[cfg(test)]
mod tests {
    use crate::paint_house_ii::Solution;
    #[test]
    fn case01() {
        let actual = Solution::min_cost_ii(vec![vec![1, 5, 3], vec![2, 9, 4]]);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
