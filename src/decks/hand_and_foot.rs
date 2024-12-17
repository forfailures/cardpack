use crate::decks::modern::Modern;
use crate::types::pile::Pile;
use crate::types::traits::Decked;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandAndFoot {}

impl HandAndFoot {
    pub const DECK_NAME: &'static str = "Hand and Foot";
}

impl Decked<Modern, Modern> for HandAndFoot {
    fn deck() -> Pile<Modern, Modern> {
        Modern::decks(5).sort()
    }

    fn pack(&self) -> Pile<Modern, Modern> {
        HandAndFoot::deck()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__hand_and_foot__tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn deck() {
        let deck = HandAndFoot::deck();

        assert_eq!(deck.len(), 270);
    }

    #[test]
    fn pile__sort() {
        let deck = HandAndFoot::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = HandAndFoot::deck();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = Pile::<Modern, Modern>::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
