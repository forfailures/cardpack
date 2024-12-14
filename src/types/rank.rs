use crate::decks::modern::Modern;
use crate::decks::skat::Skat;
use crate::decks::standard52::Standard52;
use crate::localization::{FluentName, Named};
use crate::types::card_error::CardError;
use crate::types::traits::Ranked;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank<RankType>
where
    RankType: Ranked,
{
    pub weight: u32,
    pub prime: u32,
    pub name: FluentName,
    pub phantom_data: PhantomData<RankType>,
}

impl<RankType> Rank<RankType>
where
    RankType: Ranked,
{
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
    pub fn new_with_weight(name_str: &str, weight: u32) -> Rank<RankType> {
        let name = FluentName::new(name_str);

        Rank::<RankType> {
            weight,
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

    #[must_use]
    pub fn ranks_from_array(names: &[&'static str]) -> Vec<Self> {
        let mut v: Vec<Self> = Vec::new();

        #[allow(clippy::cast_possible_truncation)]
        for (i, &elem) in names.iter().enumerate() {
            let weight = (names.len() - 1) - i;
            v.push(Rank::new_with_weight(elem, weight as u32));
        }
        v
    }

    pub fn ranks_index(ranks: &[Rank<RankType>], joiner: &str) -> String {
        ranks
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(joiner)
    }

    /// Hackie utility function to create a quick way to validate the returned ranks.
    #[must_use]
    pub fn ranks_index_all(joiner: &str) -> String {
        Rank::<RankType>::ranks_index(&Rank::<RankType>::ranks(), joiner)
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

    fn type_name() -> &'static str {
        RankType::type_name()
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
            '2' => Rank::new(Standard52::TWO),
            '3' => Rank::new(Standard52::THREE),
            '4' => Rank::new(Standard52::FOUR),
            '5' => Rank::new(Standard52::FIVE),
            '6' => Rank::new(Standard52::SIX),
            '7' => Rank::new(Standard52::SEVEN),
            '8' => Rank::new(Standard52::EIGHT),
            '9' => Rank::new(Standard52::NINE),
            'T' | 't' | '0' => Rank::new(Standard52::TEN),
            'J' | 'j' => Rank::new(Standard52::JACK),
            'Q' | 'q' => Rank::new(Standard52::QUEEN),
            'K' | 'k' => Rank::new(Standard52::KING),
            'A' | 'a' => Rank::new(Standard52::ACE),
            'B' | 'b' => Rank::new(Modern::BIG),
            'L' | 'l' => Rank::new(Modern::LITTLE),
            'D' | 'd' => Rank::new(Skat::DAUS),
            'O' | 'o' => Rank::new(Skat::OBER),
            'U' | 'u' => Rank::new(Skat::UNTER),
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
