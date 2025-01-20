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
            _ => French::get_rank(index_char),
        }
    }

    fn get_suit(index_char: char) -> Suit<Modern> {
        todo!()
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
