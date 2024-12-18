use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pinochle {}

impl Pinochle {
    pub const DECK_NAME: &'static str = "Pinochle";

    pub const ACE: &'static str = "pinochle-ace";
    pub const TEN: &'static str = "pinochle-ten";
    pub const KING: &'static str = "pinochle-king";
    pub const QUEEN: &'static str = "pinochle-queen";
    pub const JACK: &'static str = "pinochle-jack";
    pub const NINE: &'static str = "pinochle-nine";
}

impl Decked<Pinochle, Standard52> for Pinochle {
    #[must_use]
    fn deck() -> Pile<Pinochle, Standard52> {
        let ranks = Rank::<Pinochle>::ranks_from_array(&Pinochle::rank_names());
        let suits = Suit::<Standard52>::suits();

        let mut pile = Pile::<Pinochle, Standard52>::new(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                pile.push(Card::<Pinochle, Standard52>::new(
                    rank.clone(),
                    suit.clone(),
                ));
            }
        }

        pile
    }

    fn pack(&self) -> Pile<Pinochle, Standard52> {
        Pinochle::deck()
    }
}

impl Ranked for Pinochle {
    fn rank_chars() -> Vec<char> {
        vec!['9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Pinochle::ACE,
            Pinochle::TEN,
            Pinochle::KING,
            Pinochle::QUEEN,
            Pinochle::JACK,
            Pinochle::NINE,
        ]
    }

    fn type_name() -> &'static str {
        Pinochle::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__pinochle__tests {
    use super::*;
    use crate::localization::Named;
    use crate::types::rank::Rank;
    use std::str::FromStr;

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<Pinochle>::new_with_weight("ace", 20);

        assert_eq!(rank.weight, 20);
        assert_eq!(rank.prime, 41);
        assert_eq!(rank.fluent_name().prime(), 41);
        assert_eq!(rank.fluent_name_string(), "ace");
    }

    #[test]
    fn rank__ranks_from_array() {
        let ranks = Rank::<Pinochle>::ranks_from_array(&*Pinochle::rank_names());

        assert_eq!(ranks.len(), 6);
        assert_eq!(ranks[0].fluent_name_string(), "pinochle-ace");
        assert_eq!(ranks[0].weight, 5);
        assert_eq!(ranks[1].fluent_name_string(), "pinochle-ten");
        assert_eq!(ranks[1].weight, 4);
    }

    #[test]
    fn decked__sort() {
        let deck = Pinochle::deck();
        let mut shuffled = deck.shuffle_default();
        shuffled.sort_in_place();

        let expected = "A♠ T♠ K♠ Q♠ J♠ 9♠ A♥ T♥ K♥ Q♥ J♥ 9♥ A♦ T♦ K♦ Q♦ J♦ 9♦ A♣ T♣ K♣ Q♣ J♣ 9♣";

        assert_eq!(deck.to_string(), expected);
        assert_eq!(shuffled.to_string(), expected);
    }

    #[test]
    fn pile__sort() {
        let deck = Pinochle::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Pinochle::deck();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = Pile::<Pinochle, Standard52>::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
