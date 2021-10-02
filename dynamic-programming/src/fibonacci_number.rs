struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        return Solution::fib(n - 1) + Solution::fib(n - 2);
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci_number::Solution;

    #[test]
    fn it_works() {
        let actual = Solution::fib(3);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
