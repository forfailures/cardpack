use crate::old::decks::french::French;
use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::traits::{Decked, Ranked};
use std::str::FromStr;

/// This deck represents the most common 24 card form of
/// [Euchre](https://en.wikipedia.org/wiki/Euchre) with
/// `A K Q J T 9` ranks.
///
/// This means that they are made up of the [`French`]
/// implementation of the [`Suited`](crate::types::traits::Suited) trait that's declared in the
/// [`French`] deck and the `Euchre24` implementation of the
/// [`Ranked`] trait.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Euchre24 {}

impl Euchre24 {
    pub const DECK_NAME: &'static str = "Euchre24";

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Euchre24, French>, CardError> {
        Pile::<Euchre24, French>::from_str(index)
    }
}

impl Decked<Euchre24, French> for Euchre24 {
    fn blank() -> Card<Euchre24, French> {
        Card::<Euchre24, French>::default()
    }

    fn guide() -> Option<String> {
        None
    }
}

impl Ranked for Euchre24 {
    fn rank_chars() -> Vec<char> {
        vec!['9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            French::ACE,
            French::KING,
            French::QUEEN,
            French::JACK,
            French::TEN,
            French::NINE,
        ]
    }

    fn type_name() -> &'static str {
        Euchre24::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__euchre__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::types::rank::Rank;

    #[test]
    fn new() {
        let rank = Rank::<Euchre24>::new(French::ACE);

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Euchre24>::new_with_weight(French::ACE, 13);

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Euchre24>::new(French::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(French::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<Euchre24>::is_valid_rank_char(&'A'));
        assert!(!Rank::<Euchre24>::is_valid_rank_char(&'Z'));
    }

    #[test]
    fn from_char() {
        let rank = Rank::<Euchre24>::from('A');

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Euchre24>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Euchre24>::new(French::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(French::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Euchre24>::new(French::QUEEN);

        assert_eq!(rank.fluent_name_string(), French::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Euchre24>::new(French::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Euchre24>::rank_names();

        assert_eq!(names.len(), 6);
        assert_eq!(names[0], French::ACE);
        assert_eq!(names[1], French::KING);
        assert_eq!(names[2], French::QUEEN);
        assert_eq!(names[3], French::JACK);
        assert_eq!(names[4], French::TEN);
        assert_eq!(names[5], French::NINE);
    }

    #[test]
    fn pile__sort() {
        let deck = Euchre24::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Euchre24::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = Euchre24::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
