pub trait Ranked {
    fn chars() -> Vec<char>;

    fn names() -> Vec<&'static str>;

    #[must_use]
    fn is_valid_char(c: &char) -> bool {
        Self::chars().contains(c)
    }
}
