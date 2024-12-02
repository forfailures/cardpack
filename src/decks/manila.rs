use crate::decks::standard52::Standard52;
use crate::types::pile::Pile;
use crate::types::traits::{Decked, Ranked};

/// [Manila, aka Six Plus aka Short-deck](https://en.wikipedia.org/wiki/Six-plus_hold_%27em)
/// is a version of Texas Hold'em where the card Ranks of 2 through 5
/// are removed from the deck.
///
/// This means that they are made up of the [Standard52]
/// implementation of the [Suited] trait that's declared in the
/// [Standard52] deck and the [Manila] implementation of the
/// [Ranked] trait.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Manila {}

impl Decked<Manila, Standard52> for Manila {
    fn pack(&self) -> Pile<Manila, Standard52> {
        Manila::deck()
    }
}

impl Ranked for Manila {
    fn rank_chars() -> Vec<char> {
        vec![
            '6', '7', '8', '9', 'T', 't', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Standard52::ACE,
            Standard52::KING,
            Standard52::QUEEN,
            Standard52::JACK,
            Standard52::TEN,
            Standard52::NINE,
            Standard52::EIGHT,
            Standard52::SEVEN,
            Standard52::SIX,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__manila__tests {
    use super::*;
    use crate::types::rank::Rank;

    #[test]
    fn deck() {
        let deck = Manila::deck();
        assert_eq!(deck.len(), 36);
        assert_eq!(deck.to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣");
    }

    #[test]
    fn rank_chars() {
        assert_eq!(
            Rank::<Manila>::rank_chars(),
            vec!['6', '7', '8', '9', 'T', 't', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
        );
    }

    #[test]
    fn pile__sort() {
        let deck = Manila::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }
}
