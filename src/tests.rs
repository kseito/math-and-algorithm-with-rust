#[cfg(test)]
mod tests {
    use crate::{calc001, calc002, calc003, calc004, calc005, calc006, calc007, calc008, calc009, calc010, calc011, isPrime};

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
        let ans = calc003(vec![1, 2, 3, 4, 5]);
        assert_eq!(ans, 15);
    }

    #[test]
    fn test_answer004() {
        let ans = calc004(2, 8, 8);
        assert_eq!(ans, 128);
    }

    #[test]
    fn test_answer005() {
        let ans = calc005(vec![30, 50, 70]);
        assert_eq!(ans, 50);
    }

    #[test]
    fn test_answer006() {
        let ans = calc006(100);
        assert_eq!(ans, 203);
    }

    #[test]
    fn test_answer007() {
        let ans = calc007(15, 3, 5);
        assert_eq!(ans, 7);
    }

    #[test]
    fn test_answer008() {
        let ans = calc008(3, 4);
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_answer009_1() {
        let ans = calc009(vec![2, 5, 9], 11);
        assert_eq!(ans, true);
    }

    #[test]
    fn test_answer009_2() {
        let ans = calc009(vec![3, 1, 4, 5], 11);
        assert_eq!(ans, false);
    }

    #[test]
    fn test_answer010() {
        let ans = calc010(13);
        assert_eq!(ans, 6227020800i64);
    }

    #[test]
    fn test_answer011() {
        let ans = calc011(10);
        assert_eq!(ans, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_isPrime() {
        let ans = isPrime(3);
        assert_eq!(ans, true)
    }
}