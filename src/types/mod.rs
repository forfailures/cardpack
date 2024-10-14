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
    ///     "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
    /// ); // Holy mefer sheit! It frackin' worked.
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

    /// This function was `Pile::pile_on()` in the v.0 cardpack library.
    #[must_use]
    fn decks(n: usize) -> Pile<RankType, SuitType> {
        let mut pile = Pile::<RankType, SuitType>::new(Vec::new());
        for _ in 0..n {
            pile.extend(&Self::deck());
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
