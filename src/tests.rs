#[cfg(test)]
mod tests {
    use crate::{calc001, calc002, calc003, calc004, calc005, calc006, calc007, calc008, calc009, calc010, calc011, calc012, calc013, calc014, calc015, calc016, calc017, calc018, calc019, calc020, calc021, calc022, calc023, calc024, calc025, factorial, isPrime};

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

    #[test]
    fn test_answer012() {
        let ans1 = calc012(53);
        assert_eq!(ans1, true);
        let ans2 = calc012(77);
        assert_eq!(ans2, false);
        let ans3 = calc012(472249589291);
        assert_eq!(ans3, true);
    }

    #[test]
    fn test_answer013() {
        let mut ans1 = calc013(12);
        ans1.sort();
        assert_eq!(ans1, vec![1, 2, 3, 4, 6, 12]);
        let mut ans2 = calc013(827847039317);
        ans2.sort();
        assert_eq!(ans2, vec![1, 909859, 909863, 827847039317]);
    }

    #[test]
    fn test_answer014() {
        let mut ans1 = calc014(10);
        assert_eq!(ans1, vec![2, 5]);
        let mut ans1 = calc014(36);
        assert_eq!(ans1, vec![2, 2, 3, 3]);
    }

    #[test]
    fn test_answer015() {
        let mut ans1 = calc015(33, 88);
        assert_eq!(ans1, 11);
        let mut ans2 = calc015(123, 777);
        assert_eq!(ans2, 3);
    }

    #[test]
    fn test_answer016() {
        let mut ans1 = calc016(vec![12, 18, 24]);
        assert_eq!(ans1, 6);
    }

    #[test]
    fn test_answer017() {
        let mut ans1 = calc017(vec![12, 18, 14]);
        assert_eq!(ans1, 252);
    }

    #[test]
    fn test_answer018() {
        let mut ans1 = calc018(vec![100, 300, 400, 400, 200, 100]);
        assert_eq!(ans1, 5);
        let mut ans2 = calc018(vec![200, 300]);
        assert_eq!(ans2, 1);
    }

    #[test]
    fn test_answer019() {
        let mut ans1 = calc019(vec![1, 3, 2, 1, 1, 2]);
        assert_eq!(ans1, 4);
    }

    #[test]
    fn test_answer020() {
        let mut ans1 = calc020(vec![100, 150, 200, 250, 300]);
        assert_eq!(ans1, 1);
        let mut ans2 = calc020(vec![243, 156, 104, 280, 142, 286, 196, 132, 128, 195, 265, 300, 130]);
        assert_eq!(ans2, 4);
    }

    #[test]
    fn test_factorial() {
        let mut ans1 = factorial(5);
        assert_eq!(ans1, 120);
    }

    #[test]
    fn test_answer021() {
        let mut ans1 = calc021(6, 2);
        assert_eq!(ans1, 15);
    }

    #[test]
    fn test_answer022() {
        let mut ans1 = calc022(vec![40000, 50000, 20000, 80000, 50000, 30000, 1, 99999, 49999, 50001]);
        assert_eq!(ans1, 4);
        let mut ans2 = calc022(vec![50000, 50000, 50000, 50000, 50000, 50000, 50000]);
        assert_eq!(ans2, 21);
        let mut ans3 = calc022(vec![0, 50000, 49999]);
        assert_eq!(ans3, 0);
    }

    #[test]
    fn test_answer023() {
        let mut ans1 = calc023(vec![1.0, 2.0, 3.0], vec![10.0, 20.0, 30.0]);
        assert_eq!(ans1, 22.0);
    }

    #[test]
    fn test_answer024() {
        let mut ans1 = calc024(vec![(2, 50), (4, 100)]);
        assert_eq!(ans1, 50.0);
    }

    #[test]
    fn test_answer025() {
        let mut ans1 = calc025(vec![3, 1, 4, 1, 5], vec![9, 2, 6, 5, 3]);
        assert_eq!((ans1 * 1000.0).round(), 21333.0);
    }
}