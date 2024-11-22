use crate::decks::modern::Modern;
use crate::types::pile::Pile;
use crate::types::traits::Decked;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandAndFoot {}

impl Decked<Modern, Modern> for HandAndFoot {
    fn deck() -> Pile<Modern, Modern> {
        Modern::decks(5)
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
}
