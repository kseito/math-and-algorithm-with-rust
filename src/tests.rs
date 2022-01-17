#[cfg(test)]
mod tests {
    use crate::{calc001, calc002, calc003};

    #[test]
    fn test_answer001() {
        let ans = calc001(7);
        assert_eq!(ans, 12);
    }

    #[test]
    fn test_answer002() {
        let ans = calc002(5, 6, 7);
        assert_eq!(ans, 18);
    }

    #[test]
    fn test_answer003() {
        let ans = calc003(vec![1,2,3,4,5]);
        assert_eq!(ans, 15);
    }
}