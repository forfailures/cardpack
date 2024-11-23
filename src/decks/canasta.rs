use crate::decks::modern::Modern;
use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked};

/// [Canasta](https://en.wikipedia.org/wiki/Canasta)deck
///
/// TODO: WIP
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Canasta {}

impl Canasta {
    fn red_threes() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::new(Vec::new());

        let three_hearts = Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::HEARTS),
            100_001
        );
        let three_diamonds = Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::DIAMONDS),
            100_000
        );

        pile.push(three_hearts.clone());
        pile.push(three_hearts);
        pile.push(three_diamonds.clone());
        pile.push(three_diamonds);

        pile
    }
}

impl Decked<Modern, Modern> for Canasta {
    fn deck() -> Pile<Modern, Modern> {
        let mut deck = Modern::decks(2);

        let red_threes = Canasta::red_threes();

        deck.remove_cards(&red_threes);

        deck.prepend(&red_threes);


        deck.sort_in_place();
        deck
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
