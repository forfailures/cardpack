use crate::decks::french::French;
use crate::localization::FluentName;
use crate::types::pips::{Rank, Suit, BLANK};
use crate::types::traits::{Decked, Ranked, Suited};
use crate::types::{Card, Pile};
use colored::Color;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modern {}

#[allow(clippy::module_name_repetitions)]
pub type ModernCard = Card<Modern, Modern>;
#[allow(clippy::module_name_repetitions)]
pub type ModernDeck = Pile<Modern, Modern>;

impl Modern {
    pub const BIG_JOKER: Card<Modern, Modern> = Card {
        suit: Modern::JOKER_SUIT,
        rank: Modern::BIG_JOKER_RANK,
    };
    pub const LITTLE_JOKER: Card<Modern, Modern> = Card {
        suit: Modern::JOKER_SUIT,
        rank: Modern::LITTLE_JOKER_RANK,
    };

    // Ranks
    // Fluent Names
    pub const BIG_JOKER_INDEX: char = 'B';
    pub const LITTLE_JOKER_INDEX: char = 'L';
    pub const FLUENT_KEY_BIG: &'static str = "big-joker";
    pub const FLUENT_KEY_LITTLE: &'static str = "little-joker";

    pub const BIG_JOKER_RANK: Rank<Modern> = Rank {
        weight: 14,
        index: Modern::BIG_JOKER_INDEX,
        phantom_data: PhantomData,
    };
    pub const LITTLE_JOKER_RANK: Rank<Modern> = Rank {
        weight: 13,
        index: Modern::LITTLE_JOKER_INDEX,
        phantom_data: PhantomData,
    };
    // OK, part of me hates how lowbrow and hacky this is. Another part of me loves it for that
    // very reason. Well, if we come up with something better, we can add it for phase III, which
    // I hope never happens.
    pub const ACE: Rank<Modern> = Rank {
        weight: French::ACE.weight,
        index: French::ACE_INDEX,
        phantom_data: PhantomData,
    };
    pub const KING: Rank<Modern> = Rank {
        weight: French::KING.weight,
        index: French::KING_INDEX,
        phantom_data: PhantomData,
    };
    pub const QUEEN: Rank<Modern> = Rank {
        weight: French::QUEEN.weight,
        index: French::QUEEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const JACK: Rank<Modern> = Rank {
        weight: French::JACK.weight,
        index: French::JACK_INDEX,
        phantom_data: PhantomData,
    };
    pub const TEN: Rank<Modern> = Rank {
        weight: French::TEN.weight,
        index: French::TEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const NINE: Rank<Modern> = Rank {
        weight: French::NINE.weight,
        index: French::NINE_INDEX,
        phantom_data: PhantomData,
    };
    pub const EIGHT: Rank<Modern> = Rank {
        weight: French::EIGHT.weight,
        index: French::EIGHT_INDEX,
        phantom_data: PhantomData,
    };
    pub const SEVEN: Rank<Modern> = Rank {
        weight: French::SEVEN.weight,
        index: French::SEVEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const SIX: Rank<Modern> = Rank {
        weight: French::SIX.weight,
        index: French::SIX_INDEX,
        phantom_data: PhantomData,
    };
    pub const FIVE: Rank<Modern> = Rank {
        weight: French::FIVE.weight,
        index: French::FIVE_INDEX,
        phantom_data: PhantomData,
    };
    pub const FOUR: Rank<Modern> = Rank {
        weight: French::FOUR.weight,
        index: French::FOUR_INDEX,
        phantom_data: PhantomData,
    };
    pub const TREY: Rank<Modern> = Rank {
        weight: French::TREY.weight,
        index: French::TREY_INDEX,
        phantom_data: PhantomData,
    };
    pub const DEUCE: Rank<Modern> = Rank {
        weight: French::DEUCE.weight,
        index: French::DEUCE_INDEX,
        phantom_data: PhantomData,
    };

    // Suits
    // Fluent Names
    pub const FLUENT_KEY_JOKER: &'static str = "joker";
    pub const JOKERS_INDEX: char = 'J';
    pub const JOKERS_SYMBOL: char = '🃟';

    pub const JOKER_SUIT: Suit<Modern> = Suit {
        weight: 5,
        index: Modern::JOKERS_INDEX,
        phantom_data: PhantomData,
    };
    pub const SPADES: Suit<Modern> = Suit {
        weight: French::SPADES.weight,
        index: French::SPADES_INDEX,
        phantom_data: PhantomData,
    };
    pub const HEARTS: Suit<Modern> = Suit {
        weight: French::HEARTS.weight,
        index: French::HEARTS_INDEX,
        phantom_data: PhantomData,
    };
    pub const DIAMONDS: Suit<Modern> = Suit {
        weight: French::DIAMONDS.weight,
        index: French::DIAMONDS_INDEX,
        phantom_data: PhantomData,
    };
    pub const CLUBS: Suit<Modern> = Suit {
        weight: French::CLUBS.weight,
        index: French::CLUBS_INDEX,
        phantom_data: PhantomData,
    };

    #[must_use]
    pub fn jokers() -> Pile<Modern, Modern> {
        Pile::from(vec![Modern::BIG_JOKER, Modern::LITTLE_JOKER])
    }

    #[must_use]
    pub fn get_rank_fluent_name(c: char) -> FluentName {
        match c {
            Modern::BIG_JOKER_INDEX => FluentName::new(Modern::FLUENT_KEY_BIG),
            Modern::LITTLE_JOKER_INDEX => FluentName::new(Modern::FLUENT_KEY_LITTLE),
            _ => French::get_rank_fluent_name(c),
        }
    }

    #[must_use]
    pub fn get_rank_index(c: char) -> char {
        match c {
            'b' | Modern::BIG_JOKER_INDEX => Modern::BIG_JOKER_INDEX,
            'l' | Modern::LITTLE_JOKER_INDEX => Modern::LITTLE_JOKER_INDEX,
            _ => French::get_rank_index(c),
        }
    }

    #[must_use]
    pub fn get_rank_weight(c: char) -> u32 {
        match c {
            'b' | Modern::BIG_JOKER_INDEX => Modern::BIG_JOKER_RANK.weight,
            'l' | Modern::LITTLE_JOKER_INDEX => Modern::LITTLE_JOKER_RANK.weight,
            _ => French::get_rank_weight(c),
        }
    }

    #[must_use]
    pub fn get_suit_fluent_name(c: char) -> FluentName {
        match c {
            '🃟' | 'j' | Modern::JOKERS_INDEX => FluentName::new(Modern::FLUENT_KEY_JOKER),
            _ => French::get_suit_fluent_name(c),
        }
    }

    #[must_use]
    pub fn get_suit_index(c: char) -> char {
        match c {
            '🃟' | 'j' | Modern::JOKERS_INDEX => Modern::JOKERS_INDEX,
            _ => French::get_suit_index(c),
        }
    }

    #[must_use]
    pub fn get_suit_symbol(c: char) -> char {
        match c {
            'j' | Modern::JOKERS_SYMBOL | Modern::JOKERS_INDEX => Modern::JOKERS_SYMBOL,
            _ => French::get_suit_symbol(c),
        }
    }

    #[must_use]
    pub fn get_suit_weight(c: char) -> u32 {
        match c {
            '🃟' | 'j' | Modern::JOKERS_INDEX => Modern::JOKER_SUIT.weight,
            _ => French::get_suit_weight(c),
        }
    }
}

impl Decked<Modern, Modern> for Modern {
    fn get_rank(index_char: char) -> Rank<Modern> {
        match index_char {
            'b' | Modern::BIG_JOKER_INDEX => Modern::BIG_JOKER_RANK,
            'l' | Modern::LITTLE_JOKER_INDEX => Modern::LITTLE_JOKER_RANK,
            _ => Rank::<Modern>::from(French::get_rank(index_char)),
        }
    }

    fn get_suit(index_char: char) -> Suit<Modern> {
        match index_char {
            '🃟' | 'j' | Modern::JOKERS_INDEX => Modern::JOKER_SUIT,
            _ => Suit::<Modern>::from(French::get_suit(index_char)),
        }
    }
}

impl From<Rank<French>> for Rank<Modern> {
    fn from(rank: Rank<French>) -> Self {
        Rank {
            weight: rank.weight,
            index: rank.index,
            phantom_data: PhantomData,
        }
    }
}

impl From<Suit<French>> for Suit<Modern> {
    fn from(suit: Suit<French>) -> Self {
        Suit {
            weight: suit.weight,
            index: suit.index,
            phantom_data: PhantomData,
        }
    }
}

impl Suited for Modern {
    fn colors() -> HashMap<char, Color> {
        let mut mappie = HashMap::new();

        mappie.insert('J', Color::BrightBlue);
        mappie.insert('H', Color::Red);
        mappie.insert('D', Color::Red);

        mappie
    }

    fn get_suit_fluent_name(c: char) -> FluentName {
        Modern::get_suit_fluent_name(c)
    }

    fn get_suit_index(c: char) -> char {
        Modern::get_suit_index(c)
    }

    fn get_suit_symbol(c: char) -> char {
        Modern::get_suit_symbol(c)
    }

    fn get_suit_weight(c: char) -> u32 {
        Modern::get_rank_weight(c)
    }

    fn suit_indexes() -> Vec<char> {
        vec![
            Modern::JOKERS_INDEX,
            French::SPADES_INDEX,
            French::HEARTS_INDEX,
            French::DIAMONDS_INDEX,
            French::CLUBS_INDEX,
        ]
    }
}

impl Ranked for Modern {
    fn get_rank_fluent_name(c: char) -> FluentName {
        Modern::get_rank_fluent_name(c)
    }

    fn get_rank_index(c: char) -> char {
        Modern::get_rank_index(c)
    }

    fn get_rank_weight(c: char) -> u32 {
        Modern::get_rank_weight(c)
    }

    fn rank_indexes() -> Vec<char> {
        vec![
            Modern::BIG_JOKER_INDEX,
            Modern::LITTLE_JOKER_INDEX,
            French::ACE_INDEX,
            French::KING_INDEX,
            French::QUEEN_INDEX,
            French::JACK_INDEX,
            French::TEN_INDEX,
            French::NINE_INDEX,
            French::EIGHT_INDEX,
            French::SEVEN_INDEX,
            French::SIX_INDEX,
            French::FIVE_INDEX,
            French::FOUR_INDEX,
            French::TREY_INDEX,
            French::DEUCE_INDEX,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__modern__tests {
    use super::*;
    use crate::localization::Named;

    #[test]
    fn new() {
        assert_eq!(
            Card::<Modern, Modern>::new(Modern::BIG_JOKER_RANK, Modern::JOKER_SUIT).index(),
            "BJ"
        );
        assert_eq!(
            ModernCard::new(Modern::LITTLE_JOKER_RANK, Modern::JOKER_SUIT).index(),
            "LJ"
        );
        assert_eq!(
            Card::<Modern, Modern>::new(Modern::ACE, Modern::SPADES).index(),
            "AS"
        );
    }
}
