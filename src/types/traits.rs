use crate::localization::FluentName;
use crate::localization::Named;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Decked<
    RankType: Ranked + Clone + Hash + Ord + Default,
    SuitType: Suited + Clone + Hash + Ord + Default,
>
{
    /// This trait makes me very happy. It feels like it has an elegance that I really love.
    ///
    /// NOTE: We are going to need to override it for decks that have two tiers of suits, such
    /// as tarot decks and ones with jokers.
    ///
    /// ```rust
    /// use cardpack::decks::french::French;
    /// use cardpack::types::traits::Decked;
    /// use cardpack::types::pile::Pile;
    ///
    /// assert_eq!(
    ///     French::deck().to_string(),
    ///     "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
    /// ); // Holy mefer sheit! It frackin' worked.
    /// ```
    #[must_use]
    fn deck() -> Pile<RankType, SuitType> {
        let ranks = Rank::<RankType>::ranks();
        let suits = Suit::<SuitType>::suits();

        let mut pile = Pile::<RankType, SuitType>::from(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                pile.push(Card::<RankType, SuitType>::new(
                    &rank.clone(),
                    &suit.clone(),
                ));
            }
        }

        pile
    }

    fn blank() -> Card<RankType, SuitType>;

    /// This function was [`Pile::pile_on()`](https://github.com/ImperialBower/cardpack.rs/blob/67f2c79fc3f4c038dffda154e6db08608e529a50/src/cards/pile.rs#L39)
    /// in the v.0.5 cardpack library.
    #[must_use]
    fn decks(n: usize) -> Pile<RankType, SuitType> {
        Pile::<RankType, SuitType>::pile_up(n, <Self as Decked<RankType, SuitType>>::deck)
    }

    fn demo(verbose: bool) {
        let deck = Self::deck();
        let shuffled = deck.shuffle();
        let name = Self::name();

        println!();
        println!("{name} Deck:          {}", deck.to_color_symbol_string());
        println!("{name} Deck Index:    {}", deck.index());
        println!(
            "{name} Deck Shuffled: {}",
            shuffled.to_color_symbol_string()
        );

        if verbose {
            println!();
            println!("Long in English and German:");

            for card in deck {
                let anzugname = card.suit.name.long(&FluentName::DEUTSCH);
                let suitname = card.suit.name.long(&FluentName::US_ENGLISH);
                let rangname = card.rank.name.long(&FluentName::DEUTSCH);
                let rankname = card.rank.name.long(&FluentName::US_ENGLISH);
                println!("  {rankname} of {suitname} ");
                println!("  {rangname} von {anzugname} ");
            }
        }
    }

    fn guide() -> Option<String>;

    #[must_use]
    fn name() -> String {
        let full_name = std::any::type_name::<Self>();
        full_name.split("::").last().unwrap().to_string()
    }
}

pub trait Ranked {
    #[must_use]
    fn is_valid_rank_char(c: &char) -> bool {
        Self::rank_chars().contains(c)
    }

    /// A vector of the allowable chars for the `Rank` of a card. This allows for things like supporting
    /// upper and lower case characters for a rank, for added flexibility on the input.
    fn rank_chars() -> Vec<char>;

    /// [`FluentName`] vector used by the [`Rank`] struct's `ranks()` function, which in turn is
    /// called by the [`Decked`] trait's `deck()`.
    fn rank_names() -> Vec<&'static str>;

    /// Used for match statements in the `Rank` struct.
    ///
    /// This feels very hacky. It would be great if we could push this down to the deck implementations.
    fn type_name() -> &'static str;
}

pub trait Suited {
    fn colors() -> HashMap<char, colored::Color>;

    #[must_use]
    fn is_valid_suit_char(c: &char) -> bool {
        Self::suit_chars().contains(c)
    }

    /// A vector of the allowable chars for the `Suit` of a card. This allows for things like supporting
    /// upper and lower case characters for a rank, for added flexibility on the input.
    fn suit_chars() -> Vec<char>;

    fn suit_names() -> Vec<&'static str>;

    /// Used for match statements in the `Suit` struct.
    ///
    /// This feels very hacky. It would be great if we could push this down to the deck implementations.
    fn type_name() -> &'static str;
}

pub trait Shufflable<RankType: Ranked + Ord + Clone, SuitType: Suited + Ord + Clone> {
    fn shuffle<F>(&mut self, _shuffle_fn: F)
    where
        F: FnMut(&mut Vec<Card<RankType, SuitType>>),
    {
        todo!()
    }
}
