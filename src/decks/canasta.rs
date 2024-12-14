use crate::decks::modern::Modern;
use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::Decked;

/// [Canasta](https://en.wikipedia.org/wiki/Canasta)deck
///
/// TODO: WIP
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Canasta {}

impl Canasta {
    pub const DECK_NAME: &'static str = "Canasta";

    fn red_threes() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::new(Vec::new());

        let three_hearts = Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::HEARTS),
            100_001,
        );
        let three_diamonds = Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::DIAMONDS),
            100_000,
        );

        pile.push(three_hearts.clone());
        pile.push(three_hearts);
        pile.push(three_diamonds.clone());
        pile.push(three_diamonds);

        pile
    }

    fn twos() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::new(Vec::new());

        let two_spades = Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::SPADES),
            5003,
        );
        let two_hearts = Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::HEARTS),
            5002,
        );
        let two_diamonds = Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::DIAMONDS),
            5001,
        );
        let two_clubs = Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::CLUBS),
            5000,
        );

        pile.push(two_spades.clone());
        pile.push(two_spades);
        pile.push(two_hearts.clone());
        pile.push(two_hearts);
        pile.push(two_diamonds.clone());
        pile.push(two_diamonds);
        pile.push(two_clubs.clone());
        pile.push(two_clubs);

        pile
    }
}

impl Decked<Modern, Modern> for Canasta {
    fn deck() -> Pile<Modern, Modern> {
        let mut deck = Modern::decks(2);

        let twos = Canasta::twos();
        deck.remove_cards(&twos);
        deck.prepend(&twos);

        let red_threes = Canasta::red_threes();
        deck.remove_cards(&red_threes);
        deck.prepend(&red_threes);

        deck.sort_in_place();
        deck
    }

    fn pack(&self) -> Pile<Modern, Modern> {
        Canasta::deck()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__canasta__tests {
    use super::*;

    #[test]
    fn deck() {
        let deck = Canasta::deck();

        let expected = "3♥ 3♥ 3♦ 3♦ B🃟 B🃟 L🃟 L🃟 2♠ 2♠ 2♥ 2♥ 2♦ 2♦ 2♣ 2♣ A♠ A♠ K♠ K♠ Q♠ Q♠ J♠ J♠ T♠ T♠ 9♠ 9♠ 8♠ 8♠ 7♠ 7♠ 6♠ 6♠ 5♠ 5♠ 4♠ 4♠ 3♠ 3♠ A♥ A♥ K♥ K♥ Q♥ Q♥ J♥ J♥ T♥ T♥ 9♥ 9♥ 8♥ 8♥ 7♥ 7♥ 6♥ 6♥ 5♥ 5♥ 4♥ 4♥ A♦ A♦ K♦ K♦ Q♦ Q♦ J♦ J♦ T♦ T♦ 9♦ 9♦ 8♦ 8♦ 7♦ 7♦ 6♦ 6♦ 5♦ 5♦ 4♦ 4♦ A♣ A♣ K♣ K♣ Q♣ Q♣ J♣ J♣ T♣ T♣ 9♣ 9♣ 8♣ 8♣ 7♣ 7♣ 6♣ 6♣ 5♣ 5♣ 4♣ 4♣ 3♣ 3♣";

        assert_eq!(deck.len(), 108);
        assert_eq!(deck.to_string(), expected);
    }

    #[test]
    fn pile__sort() {
        let deck = Canasta::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }
}
