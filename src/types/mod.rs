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
    /// This trait makes me very happy. It feels like it has an elegance that I really love.
    ///
    /// NOTE: We are going to need to override it for decks that have two tiers of suits, such
    /// as tarot decks and ones with jokers.
    ///
    /// ```rust
    /// use cardpack::decks::standard52::Standard52;
    /// use cardpack::types::Decked;
    /// use cardpack::types::pile::Pile;
    ///
    /// assert_eq!(
    ///     Standard52::deck().to_string(),
    ///     "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C"
    /// ); // Holy mefer sheit! It did not frackin' work.
    /// ```
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
