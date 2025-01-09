use crate::decks::FluentName;
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
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct French {}

impl French {
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
    pub const THREE_INDEX: char = '3';
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
            French::THREE_INDEX => FluentName::new(French::FLUENT_KEY_THREE),
            French::TWO_INDEX => FluentName::new(French::FLUENT_KEY_TWO),
            _ => FluentName::new(FluentName::BLANK),
        }
    }
}
