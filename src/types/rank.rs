use crate::localization::{FluentName, Named};
use crate::types::card_error::CardError;
use crate::types::traits::Ranked;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank<RankType>
where
    RankType: Ranked,
{
    pub weight: u32,
    pub prime: u32,
    pub name: FluentName,
    phantom_data: PhantomData<RankType>,
}

impl<RankType> Rank<RankType>
where
    RankType: Ranked,
{
    pub const ACE: &str = "ace";
    pub const KING: &str = "king";
    pub const QUEEN: &str = "queen";
    pub const JACK: &str = "jack";
    pub const TEN: &str = "ten";
    pub const NINE: &str = "nine";
    pub const EIGHT: &str = "eight";
    pub const SEVEN: &str = "seven";
    pub const SIX: &str = "six";
    pub const FIVE: &str = "five";
    pub const FOUR: &str = "four";
    pub const THREE: &str = "three";
    pub const TWO: &str = "two";

    // Jokers
    pub const BIG: &str = "big";
    pub const LITTLE: &str = "little";

    #[must_use]
    pub fn new(name_str: &str) -> Rank<RankType> {
        let name = FluentName::new(name_str);

        Rank::<RankType> {
            weight: name.weight(),
            prime: name.prime(),
            name,
            phantom_data: PhantomData,
        }
    }

    #[must_use]
    pub fn ranks() -> Vec<Self> {
        RankType::rank_names()
            .iter()
            .map(|name| Self::new(name))
            .collect()
    }

    /// Hackie utility function to create a quick way to validate the returned ranks.
    #[must_use]
    pub fn ranks_index(joiner: &str) -> String {
        let ranks = Rank::<RankType>::ranks();
        ranks
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(joiner)
    }

    #[must_use]
    pub fn update_weight(&self, weight: u32) -> Self {
        Self::new_with_weight(self.fluent_name_string().as_str(), weight)
    }
}

impl<RankType> Display for Rank<RankType>
where
    RankType: Ranked,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.index_default())
    }
}

impl<RankType> Named<'_> for Rank<RankType>
where
    RankType: Ranked,
{
    fn new_with_weight(name_str: &str, weight: u32) -> Rank<RankType> {
        let name = FluentName::new(name_str);

        Rank::<RankType> {
            weight,
            prime: name.prime(),
            name,
            phantom_data: PhantomData,
        }
    }

    fn fluent_name(&self) -> &FluentName {
        &self.name
    }

    fn fluent_name_string(&self) -> &String {
        self.name.fluent_name_string()
    }

    fn is_blank(&self) -> bool {
        self.name.is_blank()
    }
}

impl<RankType: Ranked> Ranked for Rank<RankType> {
    fn rank_chars() -> Vec<char> {
        RankType::rank_chars()
    }

    fn rank_names() -> Vec<&'static str> {
        RankType::rank_names()
    }
}

impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(c: char) -> Self {
        if !RankType::is_valid_rank_char(&c) {
            return Rank::<RankType> {
                weight: 0,
                prime: 0,
                name: FluentName::default(),
                phantom_data: PhantomData,
            };
        }
        match c {
            '2' => Rank::new(Rank::<RankType>::TWO),
            '3' => Rank::new(Rank::<RankType>::THREE),
            '4' => Rank::new(Rank::<RankType>::FOUR),
            '5' => Rank::new(Rank::<RankType>::FIVE),
            '6' => Rank::new(Rank::<RankType>::SIX),
            '7' => Rank::new(Rank::<RankType>::SEVEN),
            '8' => Rank::new(Rank::<RankType>::EIGHT),
            '9' => Rank::new(Rank::<RankType>::NINE),
            'T' | 't' | '0' => Rank::new(Rank::<RankType>::TEN),
            'J' | 'j' => Rank::new(Rank::<RankType>::JACK),
            'Q' | 'q' => Rank::new(Rank::<RankType>::QUEEN),
            'K' | 'k' => Rank::new(Rank::<RankType>::KING),
            'A' | 'a' => Rank::new(Rank::<RankType>::ACE),
            'B' | 'b' => Rank::new(Rank::<RankType>::BIG),
            'L' | 'l' => Rank::new(Rank::<RankType>::LITTLE),
            _ => Rank::new(FluentName::BLANK),
        }
    }
}

impl<RankType: Ranked> FromStr for Rank<RankType> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            let rank = Rank::<RankType>::from(first_char);
            if rank.name.is_blank() {
                Err(CardError::InvalidFluentRank(s.to_string()))
            } else {
                Ok(rank)
            }
        } else {
            Err(CardError::InvalidFluentRank(s.to_string()))
        }
    }
}
