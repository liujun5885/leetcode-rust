struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {

    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci_number::Solution;

    #[test]
    fn it_works() {
        let actual = Solution::fib(n: i32)(3);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
