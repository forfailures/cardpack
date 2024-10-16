use crate::decks::standard52::Standard52;
use crate::types::rank::Rank;
use crate::types::traits::{Decked, Ranked};

/// Manila, aka Six Plus aka Short-deck.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Manila {}

impl Decked<Standard52, Manila> for Manila {}

impl Ranked for Manila {
    fn rank_chars() -> Vec<char> {
        vec![
            '6', '7', '8', '9', 'T', 't', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Rank::<Manila>::ACE,
            Rank::<Manila>::KING,
            Rank::<Manila>::QUEEN,
            Rank::<Manila>::JACK,
            Rank::<Manila>::TEN,
            Rank::<Manila>::NINE,
            Rank::<Manila>::EIGHT,
            Rank::<Manila>::SEVEN,
            Rank::<Manila>::SIX,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__manila__tests {
    use super::*;

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
}
