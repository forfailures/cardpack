use crate::localization::FluentName;
use crate::localization::Named;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;

pub trait Decked<RankType: Ranked + Clone + Ord + Default, SuitType: Suited + Clone + Ord + Default> {
    /// This trait makes me very happy. It feels like it has an elegance that I really love.
    ///
    /// NOTE: We are going to need to override it for decks that have two tiers of suits, such
    /// as tarot decks and ones with jokers.
    ///
    /// ```rust
    /// use cardpack::decks::standard52::Standard52;
    /// use cardpack::types::traits::Decked;
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

    fn demo(verbose: bool) {
        let deck = Self::deck();
        let shuffled = deck.shuffle_default();
        let name = Self::name();

        println!();
        println!("{name} Deck:          {deck}");
        println!("{name} Deck Index:    {}", deck.index());
        println!("{name} Deck Shuffled: {shuffled}");

        if verbose {
            println!();
            println!("Long in English and German:");

            for card in deck.cards() {
                let anzugname = card.suit.name.long(&FluentName::DEUTSCH);
                let suitname = card.suit.name.long(&FluentName::US_ENGLISH);
                let rangname = card.rank.name.long(&FluentName::DEUTSCH);
                let rankname = card.rank.name.long(&FluentName::US_ENGLISH);
                println!("  {rankname} of {suitname} ");
                println!("  {rangname} von {anzugname} ");
            }
        }
    }

    fn is_complete(&self) -> bool {
        self == Self::deck()
    }

    #[must_use]
    fn name() -> String {
        let full_name = std::any::type_name::<Self>();
        full_name.split("::").last().unwrap().to_string()
    }

    fn pack(&self) -> Pile<RankType, SuitType>;
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

pub trait Shufflable<RankType: Ranked + Ord + Clone, SuitType: Suited + Ord + Clone> {
    fn shuffle<F>(&mut self, _shuffle_fn: F)
    where
        F: FnMut(&mut Vec<Card<RankType, SuitType>>),
    {
        todo!()
    }
}
