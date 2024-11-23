use crate::decks::modern::Modern;
use crate::types::pile::Pile;
use crate::types::traits::Decked;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandAndFoot {}

impl Decked<Modern, Modern> for HandAndFoot {
    fn deck() -> Pile<Modern, Modern> {
        Modern::decks(5).sort()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__hand_and_foot__tests {
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
}
