use crate::decks::french::French;
use crate::decks::modern::Modern;
use crate::decks::pinochle::Pinochle;
use crate::decks::skat::Skat;
use crate::decks::tarot::Tarot;
use crate::localization::{FluentName, Named};
use crate::types::card_error::CardError;
use crate::types::traits::Ranked;
use crate::types::utils::Bit;
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

    /// Used to generate the `Card`'s binary signature, aka [Cactus Kev](https://suffe.cool/poker/evaluator.html)
    /// numbers.
    #[must_use]
    pub fn ckc_number(&self) -> u32 {
        self.get_bits() | self.get_shift8() | self.prime
    }

    #[must_use]
    pub fn get_bits(&self) -> u32 {
        1 << (Bit::RANK_FLAG_SHIFT + self.weight)
    }

    #[must_use]
    pub fn get_shift8(&self) -> u32 {
        self.weight << 8
    }

    #[must_use]
    pub fn update_weight(&self, weight: u32) -> Self {
        Self::new_with_weight(self.fluent_name_string().as_str(), weight)
    }
}

impl<RankType> Default for Rank<RankType>
where
    RankType: Ranked,
{
    fn default() -> Self {
        Rank::<RankType> {
            weight: 0,
            prime: 0,
            name: FluentName::default(),
            phantom_data: PhantomData,
        }
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
        match RankType::type_name() {
            Skat::DECK_NAME => match c {
                'D' | 'd' => Rank::new(Skat::DAUS),
                'T' | 't' | '0' => Rank::new(Skat::ZHEN),
                'K' | 'k' => Rank::new(Skat::KÖNIG),
                'O' | 'o' => Rank::new(Skat::OBER),
                'U' | 'u' => Rank::new(Skat::UNTER),
                '9' => Rank::new(Skat::NEUN),
                '8' => Rank::new(Skat::ACHT),
                '7' => Rank::new(Skat::SIEBEN),
                _ => Rank::new(FluentName::BLANK),
            },
            Pinochle::DECK_NAME => match c {
                'A' | 'a' => Rank::new(Pinochle::ACE),
                'T' | 't' => Rank::new(Pinochle::TEN),
                'K' | 'k' => Rank::new(Pinochle::KING),
                'Q' | 'q' => Rank::new(Pinochle::QUEEN),
                'J' | 'j' => Rank::new(Pinochle::JACK),
                '9' => Rank::new(Pinochle::NINE),
                _ => Rank::new(FluentName::BLANK),
            },
            Tarot::DECK_NAME => match c {
                '2' => Rank::new(French::TWO),
                '3' => Rank::new(French::THREE),
                '4' => Rank::new(French::FOUR),
                '5' => Rank::new(French::FIVE),
                '6' => Rank::new(French::SIX),
                '7' => Rank::new(French::SEVEN),
                '8' => Rank::new(French::EIGHT),
                '9' => Rank::new(French::NINE),
                'T' | 't' | '0' => Rank::new(French::TEN),
                'P' | 'p' => Rank::new(Tarot::PAGE),
                'J' | 'j' => Rank::new(Tarot::KNIGHT),
                'Q' | 'q' => Rank::new(French::QUEEN),
                'K' | 'k' => Rank::new(French::KING),
                'A' | 'a' => Rank::new(French::ACE),
                '🤡' => Rank::new(Tarot::FOOL),
                '🧙' => Rank::new(Tarot::MAGICIAN),
                '😇' => Rank::new(Tarot::PRIESTESS),
                '👑' => Rank::new(Tarot::EMPRESS),
                '🤴' => Rank::new(Tarot::EMPEROR),
                '🧎' => Rank::new(Tarot::HIEROPHANT),
                '💏' => Rank::new(Tarot::LOVERS),
                '🏎' => Rank::new(Tarot::CHARIOT),
                '💪' => Rank::new(Tarot::STRENGTH),
                '💡' => Rank::new(Tarot::HERMIT),
                '🍀' => Rank::new(Tarot::FORTUNE),
                '⚖' => Rank::new(Tarot::JUSTICE),
                '🙃' => Rank::new(Tarot::HANGED),
                '💀' => Rank::new(Tarot::DEATH),
                '🚭' => Rank::new(Tarot::TEMPERANCE),
                '😈' => Rank::new(Tarot::DEVIL),
                '🏢' => Rank::new(Tarot::TOWER),
                '⭐' => Rank::new(Tarot::STAR),
                '🌙' => Rank::new(Tarot::MOON),
                '🌞' => Rank::new(Tarot::SUN),
                '🔔' => Rank::new(Tarot::JUDGEMENT),
                '🌍' => Rank::new(Tarot::WORLD),
                _ => Rank::new(FluentName::BLANK),
            },
            _ => match c {
                '2' => Rank::new(French::TWO),
                '3' => Rank::new(French::THREE),
                '4' => Rank::new(French::FOUR),
                '5' => Rank::new(French::FIVE),
                '6' => Rank::new(French::SIX),
                '7' => Rank::new(French::SEVEN),
                '8' => Rank::new(French::EIGHT),
                '9' => Rank::new(French::NINE),
                'T' | 't' | '0' => Rank::new(French::TEN),
                'J' | 'j' => Rank::new(French::JACK),
                'Q' | 'q' => Rank::new(French::QUEEN),
                'K' | 'k' => Rank::new(French::KING),
                'A' | 'a' => Rank::new(French::ACE),
                'B' | 'b' => Rank::new(Modern::BIG),
                'L' | 'l' => Rank::new(Modern::LITTLE),
                _ => Rank::new(FluentName::BLANK),
            },
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

#[cfg(test)]
#[allow(non_snake_case)]
mod types__rank__tests {
    use super::*;
    use crate::card;
    use crate::types::card::Card;
    use crate::types::utils::Bit;
    use ckc_rs::CardNumber;

    #[test]
    fn get_bits() {
        let card = card!("AS");
        let ckc_as = Bit::ckc_bits(CardNumber::ACE_SPADES);

        // println!("{:b}", ckc_as);
        // println!("{:b}", card.rank.get_bits());

        assert_eq!(card.rank.get_bits(), ckc_as);
    }

    #[test]
    fn get_shift8() {
        let card = Card::<French, French>::from_str("3S").unwrap();

        assert_eq!(
            card.rank.get_shift8(),
            Bit::ckc_shift8(CardNumber::TREY_SPADES)
        );
    }

    #[test]
    fn prime() {
        let card = card!("AS");
        let ckc_as = Bit::ckc_prime(CardNumber::ACE_SPADES);

        // println!("{:b}", ckc_as);
        // println!("{:b}", card.rank.prime);

        assert_eq!(card.rank.prime, ckc_as);
    }

    #[test]
    fn ckc_number() {
        let card = card!("AS");
        let ckc_as = Bit::strip_suit_flags(CardNumber::ACE_SPADES);

        assert_eq!(card.rank.ckc_number(), ckc_as);
    }

    #[test]
    fn from_char() {
        let rank = Rank::<French>::from('A');

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<French>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str__invalid() {
        let rank = Rank::<French>::from_str("Z'");

        assert!(rank.is_err());
        if let Err(CardError::InvalidFluentRank(_)) = rank {
            // The error is of type CardError::InvalidFluentRank
            // There has got to be a better way to test this.
        } else {
            panic!("Expected CardError::InvalidFluentRank");
        }
    }
}
