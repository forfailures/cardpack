use cardpack::decks::standard52::Standard52;
use cardpack::types::card::Card;
use cardpack::types::pile::Pile;
use cardpack::types::traits::Decked;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum BridgeDirection {
    N,
    E,
    S,
    W,
    Unknown,
}

impl BridgeDirection {
    #[must_use]
    pub fn to(c: char) -> BridgeDirection {
        match c {
            'S' | 's' => BridgeDirection::S,
            'N' | 'n' => BridgeDirection::N,
            'E' | 'e' => BridgeDirection::E,
            'W' | 'w' => BridgeDirection::W,
            _ => BridgeDirection::Unknown,
        }
    }

    fn next(&self) -> BridgeDirection {
        match self {
            BridgeDirection::S => BridgeDirection::W,
            BridgeDirection::W => BridgeDirection::N,
            BridgeDirection::N => BridgeDirection::E,
            BridgeDirection::E => BridgeDirection::S,
            BridgeDirection::Unknown => BridgeDirection::Unknown,
        }
    }
}

/// `BridgeBoard` is a French Deck Pack that sorts and validates the hands dealt as a part
/// of a Bridge hand.
#[derive(Clone, Debug, Hash, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct BridgeBoard {
    pack: Pile<Standard52, Standard52>,
    pub south: Pile<Standard52, Standard52>,
    pub west: Pile<Standard52, Standard52>,
    pub north: Pile<Standard52, Standard52>,
    pub east: Pile<Standard52, Standard52>,
}

impl BridgeBoard {
    /// Parses a Portable Bridge Notation deal string and converts it into a
    /// `BridgeBoard` struct.
    ///
    /// # Panics
    ///
    /// Will panic if an invalid PBN deal string is passed in.
    #[must_use]
    pub fn from_pbn_deal(deal: &str) -> Self {
        let (direction, pbn) = BridgeBoard::split_on_direction(deal);

        let mut dir_iter = pbn.split_whitespace();

        let mut board = BridgeBoard::default();
        board.fold_in(&direction, board.to_pile(dir_iter.next().unwrap()));
        board.fold_in(&direction.next(), board.to_pile(dir_iter.next().unwrap()));
        board.fold_in(
            &direction.next().next(),
            board.to_pile(dir_iter.next().unwrap()),
        );
        board.fold_in(
            &direction.next().next().next(),
            board.to_pile(dir_iter.next().unwrap()),
        );

        board
    }

    fn fold_in(&mut self, direction: &BridgeDirection, hand: Pile<Standard52, Standard52>) {
        match direction {
            BridgeDirection::S => self.south = hand.sort(),
            BridgeDirection::W => self.west = hand.sort(),
            BridgeDirection::N => self.north = hand.sort(),
            BridgeDirection::E => self.east = hand.sort(),
            BridgeDirection::Unknown => self.east = hand,
        }
    }

    fn splice_suit_in(s: &str, suit: char) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();

        for c in s.chars() {
            v.push(format!("{c}{suit}"));
        }
        v
    }

    fn split_on_direction(deal: &str) -> (BridgeDirection, &str) {
        let direction = BridgeDirection::to(deal.chars().next().unwrap());
        let remainder = &deal[2..];

        (direction, remainder)
    }

    fn to_pile(&self, s: &str) -> Pile<Standard52, Standard52> {
        let rawsuits: Vec<&str> = s.split('.').collect();

        let mut v: Vec<String> = Vec::new();
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.first().unwrap(),
            'S',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.get(1).unwrap(),
            'H',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.get(2).unwrap(),
            'D',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.get(3).unwrap(),
            'C',
        ));

        let coll: Vec<Card<Standard52, Standard52>> = v
            .iter()
            .map(|s| self.pack.cards().card_by_index(s.as_str()).unwrap().clone())
            .collect();

        Pile::from(coll)
    }
}

impl Default for BridgeBoard {
    fn default() -> Self {
        BridgeBoard {
            pack: Standard52::deck(),
            south: Pile::default(),
            west: Pile::default(),
            north: Pile::default(),
            east: Pile::default(),
        }
    }
}

fn main() {}
