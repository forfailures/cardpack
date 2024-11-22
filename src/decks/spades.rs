use crate::decks::modern::Modern;
use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::Decked;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Spades {}

impl Decked<Modern, Modern> for Spades {
    fn deck() -> Pile<Modern, Modern> {
        let mut deck = Modern::deck();

        let two_clubs = Card::new(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::CLUBS),
        );
        let two_diamonds = Card::new(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::DIAMONDS),
        );

        deck.remove_card(&two_clubs).unwrap();
        deck.remove_card(&two_diamonds).unwrap();

        deck
    }
}
