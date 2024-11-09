use crate::decks::standard52::Standard52;
use crate::types::traits::{Decked, Ranked};

/// [Canasta](https://en.wikipedia.org/wiki/Canasta)deck
///
/// TODO: WIP
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Canasta {}

impl Decked<Standard52, Canasta> for Canasta {}

impl Ranked for Canasta {
    fn rank_chars() -> Vec<char> {
        Standard52::rank_chars()
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
            Standard52::FIVE,
            Standard52::FOUR,
            Standard52::THREE,
            Standard52::TWO,
            Standard52::ACE,
            Standard52::KING,
            Standard52::QUEEN,
            Standard52::JACK,
            Standard52::TEN,
            Standard52::NINE,
            Standard52::EIGHT,
            Standard52::SEVEN,
            Standard52::SIX,
            Standard52::FIVE,
            Standard52::FOUR,
            Standard52::THREE,
            Standard52::TWO,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__canasta__tests {
    use super::*;
    use crate::types::rank::Rank;

    #[test]
    #[ignore]
    fn deck() {
        let deck = Canasta::deck();

        assert_eq!(
            deck.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
        assert_eq!(deck.len(), 108);
    }

    #[test]
    fn rank_chars() {
        assert_eq!(
            Rank::<Canasta>::rank_chars(),
            vec![
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K',
                'k', 'A', 'a'
            ]
        );
    }
}
