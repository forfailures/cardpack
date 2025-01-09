use crate::decks::FluentName;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub trait Ranked {
    fn name(index: char) -> FluentName;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank<RankType>
where
    RankType: Ranked,
{
    pub weight: u32,
    pub index: char,
    pub phantom_data: PhantomData<RankType>,
}

impl<RankType> Rank<RankType>
where
    RankType: Ranked,
{
    pub const BLANK: char = '_';

    #[must_use]
    pub fn get_name(&self) -> FluentName {
        RankType::name(self.index)
    }
}

impl<RankType> Display for Rank<RankType>
where
    RankType: Ranked,
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::ACE.get_name().fluent_name_string(), French::FLUENT_KEY_ACE);
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct French {}

impl French {
    pub const ACE: Rank<French> = Rank {
        weight: 12,
        index: French::ACE_INDEX,
        phantom_data: PhantomData,
    };
    pub const KING: Rank<French> = Rank {
        weight: 11,
        index: French::KING_INDEX,
        phantom_data: PhantomData,
    };
    pub const QUEEN: Rank<French> = Rank {
        weight: 10,
        index: French::QUEEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const JACK: Rank<French> = Rank {
        weight: 9,
        index: French::JACK_INDEX,
        phantom_data: PhantomData,
    };
    pub const TEN: Rank<French> = Rank {
        weight: 8,
        index: French::TEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const NINE: Rank<French> = Rank {
        weight: 7,
        index: French::NINE_INDEX,
        phantom_data: PhantomData,
    };
    pub const EIGHT: Rank<French> = Rank {
        weight: 6,
        index: French::EIGHT_INDEX,
        phantom_data: PhantomData,
    };
    pub const SEVEN: Rank<French> = Rank {
        weight: 5,
        index: French::SEVEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const SIX: Rank<French> = Rank {
        weight: 4,
        index: French::SIX_INDEX,
        phantom_data: PhantomData,
    };
    pub const FIVE: Rank<French> = Rank {
        weight: 3,
        index: French::FIVE_INDEX,
        phantom_data: PhantomData,
    };
    pub const FOUR: Rank<French> = Rank {
        weight: 2,
        index: French::FOUR_INDEX,
        phantom_data: PhantomData,
    };
    pub const TREY: Rank<French> = Rank {
        weight: 1,
        index: French::TREY_INDEX,
        phantom_data: PhantomData,
    };
    pub const DEUCE: Rank<French> = Rank {
        weight: 0,
        index: French::TWO_INDEX,
        phantom_data: PhantomData,
    };

    pub const ACE_INDEX: char = 'A';
    pub const KING_INDEX: char = 'K';
    pub const QUEEN_INDEX: char = 'Q';
    pub const JACK_INDEX: char = 'J';
    pub const TEN_INDEX: char = 'T';
    pub const NINE_INDEX: char = '9';
    pub const EIGHT_INDEX: char = '8';
    pub const SEVEN_INDEX: char = '7';
    pub const SIX_INDEX: char = '6';
    pub const FIVE_INDEX: char = '5';
    pub const FOUR_INDEX: char = '4';
    pub const TREY_INDEX: char = '3';
    pub const TWO_INDEX: char = '2';

    pub const FLUENT_KEY_ACE: &'static str = "ace";
    pub const FLUENT_KEY_KING: &'static str = "king";
    pub const FLUENT_KEY_QUEEN: &'static str = "queen";
    pub const FLUENT_KEY_JACK: &'static str = "jack";
    pub const FLUENT_KEY_TEN: &'static str = "ten";
    pub const FLUENT_KEY_NINE: &'static str = "nine";
    pub const FLUENT_KEY_EIGHT: &'static str = "eight";
    pub const FLUENT_KEY_SEVEN: &'static str = "seven";
    pub const FLUENT_KEY_SIX: &'static str = "six";
    pub const FLUENT_KEY_FIVE: &'static str = "five";
    pub const FLUENT_KEY_FOUR: &'static str = "four";
    pub const FLUENT_KEY_THREE: &'static str = "three";
    pub const FLUENT_KEY_TWO: &'static str = "two";
}

impl Ranked for French {
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::ACE.get_name().fluent_name_string(), French::FLUENT_KEY_ACE);
    /// ```
    fn name(index: char) -> FluentName {
        match index {
            French::ACE_INDEX => FluentName::new(French::FLUENT_KEY_ACE),
            French::KING_INDEX => FluentName::new(French::FLUENT_KEY_KING),
            French::QUEEN_INDEX => FluentName::new(French::FLUENT_KEY_QUEEN),
            French::JACK_INDEX => FluentName::new(French::FLUENT_KEY_JACK),
            French::TEN_INDEX => FluentName::new(French::FLUENT_KEY_TEN),
            French::NINE_INDEX => FluentName::new(French::FLUENT_KEY_NINE),
            French::EIGHT_INDEX => FluentName::new(French::FLUENT_KEY_EIGHT),
            French::SEVEN_INDEX => FluentName::new(French::FLUENT_KEY_SEVEN),
            French::SIX_INDEX => FluentName::new(French::FLUENT_KEY_SIX),
            French::FIVE_INDEX => FluentName::new(French::FLUENT_KEY_FIVE),
            French::FOUR_INDEX => FluentName::new(French::FLUENT_KEY_FOUR),
            French::TREY_INDEX => FluentName::new(French::FLUENT_KEY_THREE),
            French::TWO_INDEX => FluentName::new(French::FLUENT_KEY_TWO),
            _ => FluentName::new(FluentName::BLANK),
        }
    }
}
