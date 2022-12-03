pub fn score(c: &u8) -> u64 {
    (match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("bad char {c}"),
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score(&b'a'), 1);
        assert_eq!(score(&b'z'), 26);
        assert_eq!(score(&b'A'), 27);
        assert_eq!(score(&b'Z'), 52);
    }
}
