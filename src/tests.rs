#[cfg(test)]
mod tests {
    use crate::{calc001};

    #[test]
    fn test_answer001() {
        let ans = calc001(7);
        assert_eq!(ans, 12);
    }
}