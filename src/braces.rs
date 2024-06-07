const BRACES: [char; 3] = ['(', '[', '{'];
fn is_valid(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    s.chars().all(|c| match c {
        ')' => stack.pop() == Some('('),
        '}' => stack.pop() == Some('{'),
        ']' => stack.pop() == Some('['),
        _ => {
            BRACES.contains(&c).then(|| stack.push(c));
            true
        }
    }) && stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("{()()()[][][]}"));
        assert!(!is_valid("{()"));
    }
}
