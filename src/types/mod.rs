use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;

pub mod card;
pub mod card_error;
pub mod pile;
pub mod rank;
pub mod suit;

pub trait Decked<SuitType: Suited + Clone + Ord, RankType: Ranked + Clone + Ord> {
    #[must_use]
    fn deck() -> Pile<RankType, SuitType> {
        let ranks = Rank::<RankType>::ranks();
        let suits = Suit::<SuitType>::suits();

        let mut pile = Pile::<RankType, SuitType>::new(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                pile.push(Card::<RankType, SuitType>::new(rank.clone(), suit.clone()));
            }
        }

        pile
    }
}

pub trait Ranked {
    fn rank_chars() -> Vec<char>;

    fn rank_names() -> Vec<&'static str>;

    #[must_use]
    fn is_valid_rank_char(c: &char) -> bool {
        Self::rank_chars().contains(c)
    }
}

pub trait Suited {
    #[must_use]
    fn is_valid_suit_char(c: &char) -> bool {
        Self::suit_chars().contains(c)
    }

    fn suit_chars() -> Vec<char>;

    fn suit_names() -> Vec<&'static str>;
}
