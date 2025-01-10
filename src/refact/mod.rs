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

    const PRIMES: [u32; 20] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];

    #[must_use]
    pub fn get_name(&self) -> FluentName {
        RankType::name(self.index)
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::DEUCE.get_prime(), 2);
    /// assert_eq!(French::TREY.get_prime(), 3);
    /// assert_eq!(French::FOUR.get_prime(), 5);
    /// assert_eq!(French::FIVE.get_prime(), 7);
    /// assert_eq!(French::SIX.get_prime(), 11);
    /// assert_eq!(French::SEVEN.get_prime(), 13);
    /// assert_eq!(French::EIGHT.get_prime(), 17);
    /// assert_eq!(French::NINE.get_prime(), 19);
    /// assert_eq!(French::TEN.get_prime(), 23);
    /// assert_eq!(French::JACK.get_prime(), 29);
    /// assert_eq!(French::QUEEN.get_prime(), 31);
    /// assert_eq!(French::KING.get_prime(), 37);
    /// assert_eq!(French::ACE.get_prime(), 41);
    /// ```
    ///
    /// It only goes up to 20:
    ///
    /// ```
    /// use std::marker::PhantomData;
    /// use cardpack::refactored::*;
    ///
    /// let heavy_card = French::TREY.update_weight(21);
    ///
    /// assert_eq!(heavy_card.get_prime(), 0);
    /// ```
    /// TODO: Hack
    #[must_use]
    pub fn get_prime(&self) -> u32 {
        if self.weight as usize >= Rank::<RankType>::PRIMES.len() {
            0
        } else {
            Rank::<RankType>::PRIMES[(self.weight) as usize]
        }
    }

    #[must_use]
    pub fn update_weight(&self, weight: u32) -> Rank<RankType> {
        Rank {
            weight,
            index: self.index,
            phantom_data: PhantomData,
        }
    }
}

impl<RankType> Display for Rank<RankType>
where
    RankType: Ranked,
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::DEUCE_INDEX.to_string(), French::DEUCE.to_string());
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
        index: French::DEUCE_INDEX,
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
    pub const DEUCE_INDEX: char = '2';

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
    pub const FLUENT_KEY_TREY: &'static str = "three";
    pub const FLUENT_KEY_DEUCE: &'static str = "two";
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
            French::TREY_INDEX => FluentName::new(French::FLUENT_KEY_TREY),
            French::DEUCE_INDEX => FluentName::new(French::FLUENT_KEY_DEUCE),
            _ => FluentName::new(FluentName::BLANK),
        }
    }
}
