#[cfg(test)]
mod tests {
    use crate::answer001;

    #[test]
    fn test_answer001() {
        let ans = answer001();
        assert_eq!(ans, 128);
    }
}