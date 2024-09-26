use crate::localization::{FluentName, Named};
use crate::types::Suited;
use std::marker::PhantomData;

pub struct Suit<SuitType>
where
    SuitType: Suited,
{
    pub weight: u32,
    pub name: FluentName,
    phantom_data: PhantomData<SuitType>,
}

impl<SuitType> Suit<SuitType>
where
    SuitType: Suited,
{
    pub const SPADES: &str = "spades";
    pub const HEARTS: &str = "hearts";
    pub const DIAMONDS: &str = "diamonds";
    pub const CLUBS: &str = "clubs";

    #[must_use]
    pub fn new(name_str: &str) -> Suit<SuitType> {
        let name = FluentName::new(name_str);

        Suit::<SuitType> {
            weight: name.weight(),
            name,
            phantom_data: PhantomData,
        }
    }

    #[must_use]
    pub fn new_with_weight(name_str: &str, weight: u32) -> Suit<SuitType> {
        let name = FluentName::new(name_str);

        Suit::<SuitType> {
            weight,
            name,
            phantom_data: PhantomData,
        }
    }

    #[must_use]
    pub fn binary_signature(&self) -> u32 {
        2u32.pow(self.weight)
    }

    #[must_use]
    pub fn binary_signature_revised(&self) -> u32 {
        2u32.pow(15 - self.weight)
    }
}

impl<SuitType> Named<'_> for Suit<SuitType>
where
    SuitType: Suited,
{
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

impl<SuiteType: Suited> Suited for Suit<SuiteType> {
    fn suit_chars() -> Vec<char> {
        SuiteType::suit_chars()
    }

    fn suit_names() -> Vec<&'static str> {
        SuiteType::suit_names()
    }
}

impl<SuitType: Suited> From<char> for Suit<SuitType> {
    fn from(c: char) -> Self {
        if !SuitType::is_valid_char(&c) {
            return Suit::<SuitType> {
                weight: 0,
                name: FluentName::default(),
                phantom_data: PhantomData,
            };
        }
        match c {
            'S' | 's' | '♤' | '♠' => Suit::<SuitType>::new(Suit::<SuitType>::SPADES),
            'H' | 'h' | '♡' | '♥' => Suit::<SuitType>::new(Suit::<SuitType>::HEARTS),
            'D' => Suit::<SuitType>::new(Suit::<SuitType>::DIAMONDS),
            'C' => Suit::<SuitType>::new(Suit::<SuitType>::CLUBS),
            _ => Suit::new(FluentName::BLANK),
        }
    }
}
