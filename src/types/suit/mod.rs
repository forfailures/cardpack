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
