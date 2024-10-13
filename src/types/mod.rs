use fluent_templates::fluent_bundle::memoizer::MemoizerKind;
use crate::types::pile::Pile;

pub mod card;
pub mod card_error;
pub mod pile;
pub mod rank;
pub mod suit;

pub trait Decked<SuitType: Suited + Clone + Ord, RankType: Ranked + Clone + Ord> {
    fn deck() -> Pile<RankType, SuitType>;
}

pub trait Ranked {
    fn rank_chars() -> Vec<char>;

    fn rank_names() -> Vec<&'static str>;

    #[must_use]
    fn is_valid_rank_char(c: &char) -> bool {
        Self::rank_chars().contains(c)
    }
}

pub trait Suited {
    #[must_use]
    fn is_valid_suit_char(c: &char) -> bool {
        Self::suit_chars().contains(c)
    }

    fn suit_chars() -> Vec<char>;

    fn suit_names() -> Vec<&'static str>;


}
