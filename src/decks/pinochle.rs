use crate::decks::standard52::Standard52;
use crate::types::traits::{Decked, Ranked};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pinochle {}

impl Decked<Standard52, Pinochle> for Pinochle {}

impl Ranked for Pinochle {
    fn rank_chars() -> Vec<char> {
        vec!['9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Standard52::ACE,
            Standard52::TEN,
            Standard52::KING,
            Standard52::QUEEN,
            Standard52::JACK,
            Standard52::NINE,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__pinochle__tests {
    use crate::localization::Named;
    use crate::types::rank::Rank;
    use super::*;

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<Pinochle>::new_with_weight("Ace", 20);

        assert_eq!(rank.weight, 20);
        assert_eq!(rank.prime, 0);
        assert_eq!(rank.fluent_name().prime(), 1);
        assert_eq!(rank.fluent_name_string(), "Ace");
    }
}