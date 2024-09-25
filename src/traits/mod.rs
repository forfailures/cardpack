pub trait Ranked {
    fn rank_chars() -> Vec<char>;

    fn rank_names() -> Vec<&'static str>;

    #[must_use]
    fn is_valid_rank_char(c: &char) -> bool {
        Self::rank_chars().contains(c)
    }
}

pub trait Suited {
    fn suit_chars() -> Vec<char>;

    fn suit_names() -> Vec<&'static str>;

    #[must_use]
    fn is_valid_char(c: &char) -> bool {
        Self::suit_chars().contains(c)
    }
}
