use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pinochle {}

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
}

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
    use super::*;
    use crate::localization::Named;
    use crate::types::rank::Rank;

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
        assert_eq!(ranks[0].fluent_name_string(), "ace");
        assert_eq!(ranks[0].weight, 5);
        assert_eq!(ranks[1].fluent_name_string(), "ten");
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
}
