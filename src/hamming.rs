fn calc_hamming_distance(s1: &str, s2: &str) -> Option<u32> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(
        s1.chars()
            .zip(s2.chars())
            .map(|(a, b)| u32::from(a != b))
            .sum(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_hamming_distance() {
        assert_eq!(calc_hamming_distance("abc", "abc"), Some(0));
        assert_eq!(calc_hamming_distance("abc", "cba"), Some(2));
    }
}
