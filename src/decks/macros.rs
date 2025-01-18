/// These macros make me very happy. They plaster over a lot of headaches
/// from the generics.
///
/// ```
/// use cardpack::prelude::*;
///
/// assert_eq!(card!(AS), Card::<French, French>::from_str("A♠").unwrap());
/// assert_eq!(card!(KS), Card::<French, French>::from_str("K♠").unwrap());
/// assert_eq!(card!(QS), Card::<French, French>::from_str("Q♠").unwrap());
/// assert_eq!(card!(JS), Card::<French, French>::from_str("J♠").unwrap());
/// assert_eq!(card!(TS), Card::<French, French>::from_str("T♠").unwrap());
/// assert_eq!(card!(9S), Card::<French, French>::from_str("9♠").unwrap());
/// assert_eq!(card!(8S), Card::<French, French>::from_str("8♠").unwrap());
/// assert_eq!(card!(7S), Card::<French, French>::from_str("7♠").unwrap());
/// assert_eq!(card!(6S), Card::<French, French>::from_str("6♠").unwrap());
/// assert_eq!(card!(5S), Card::<French, French>::from_str("5♠").unwrap());
/// assert_eq!(card!(4S), Card::<French, French>::from_str("4♠").unwrap());
/// assert_eq!(card!(3S), Card::<French, French>::from_str("3♠").unwrap());
/// assert_eq!(card!(2S), Card::<French, French>::from_str("2♠").unwrap());
/// assert_eq!(card!(AH), Card::<French, French>::from_str("A♥").unwrap());
/// assert_eq!(card!(KH), Card::<French, French>::from_str("K♥").unwrap());
/// assert_eq!(card!(QH), Card::<French, French>::from_str("Q♥").unwrap());
/// assert_eq!(card!(JH), Card::<French, French>::from_str("J♥").unwrap());
/// assert_eq!(card!(TH), Card::<French, French>::from_str("T♥").unwrap());
/// assert_eq!(card!(9H), Card::<French, French>::from_str("9♥").unwrap());
/// assert_eq!(card!(8H), Card::<French, French>::from_str("8♥").unwrap());
/// assert_eq!(card!(7H), Card::<French, French>::from_str("7♥").unwrap());
/// assert_eq!(card!(6H), Card::<French, French>::from_str("6♥").unwrap());
/// assert_eq!(card!(5H), Card::<French, French>::from_str("5♥").unwrap());
/// assert_eq!(card!(4H), Card::<French, French>::from_str("4♥").unwrap());
/// assert_eq!(card!(3H), Card::<French, French>::from_str("3♥").unwrap());
/// assert_eq!(card!(2H), Card::<French, French>::from_str("2♥").unwrap());
/// assert_eq!(card!(AD), Card::<French, French>::from_str("A♦").unwrap());
/// assert_eq!(card!(KD), Card::<French, French>::from_str("K♦").unwrap());
/// assert_eq!(card!(QD), Card::<French, French>::from_str("Q♦").unwrap());
/// assert_eq!(card!(JD), Card::<French, French>::from_str("J♦").unwrap());
/// assert_eq!(card!(TD), Card::<French, French>::from_str("T♦").unwrap());
/// assert_eq!(card!(9D), Card::<French, French>::from_str("9♦").unwrap());
/// assert_eq!(card!(8D), Card::<French, French>::from_str("8♦").unwrap());
/// assert_eq!(card!(7D), Card::<French, French>::from_str("7♦").unwrap());
/// assert_eq!(card!(6D), Card::<French, French>::from_str("6♦").unwrap());
/// assert_eq!(card!(5D), Card::<French, French>::from_str("5♦").unwrap());
/// assert_eq!(card!(4D), Card::<French, French>::from_str("4♦").unwrap());
/// assert_eq!(card!(3D), Card::<French, French>::from_str("3♦").unwrap());
/// assert_eq!(card!(2D), Card::<French, French>::from_str("2♦").unwrap());
/// ```
#[macro_export]
macro_rules! card {
    (AS) => {
        Card {
            suit: French::SPADES,
            rank: French::ACE,
        }
    };
    (KS) => {
        Card {
            suit: French::SPADES,
            rank: French::KING,
        }
    };
    (QS) => {
        Card {
            suit: French::SPADES,
            rank: French::QUEEN,
        }
    };
    (JS) => {
        Card {
            suit: French::SPADES,
            rank: French::JACK,
        }
    };
    (TS) => {
        Card {
            suit: French::SPADES,
            rank: French::TEN,
        }
    };
    (9S) => {
        Card {
            suit: French::SPADES,
            rank: French::NINE,
        }
    };
    (8S) => {
        Card {
            suit: French::SPADES,
            rank: French::EIGHT,
        }
    };
    (7S) => {
        Card {
            suit: French::SPADES,
            rank: French::SEVEN,
        }
    };
    (6S) => {
        Card {
            suit: French::SPADES,
            rank: French::SIX,
        }
    };
    (5S) => {
        Card {
            suit: French::SPADES,
            rank: French::FIVE,
        }
    };
    (4S) => {
        Card {
            suit: French::SPADES,
            rank: French::FOUR,
        }
    };
    (3S) => {
        Card {
            suit: French::SPADES,
            rank: French::TREY,
        }
    };
    (2S) => {
        Card {
            suit: French::SPADES,
            rank: French::DEUCE,
        }
    };
    (AH) => {
        Card {
            suit: French::HEARTS,
            rank: French::ACE,
        }
    };
    (KH) => {
        Card {
            suit: French::HEARTS,
            rank: French::KING,
        }
    };
    (QH) => {
        Card {
            suit: French::HEARTS,
            rank: French::QUEEN,
        }
    };
    (JH) => {
        Card {
            suit: French::HEARTS,
            rank: French::JACK,
        }
    };
    (TH) => {
        Card {
            suit: French::HEARTS,
            rank: French::TEN,
        }
    };
    (9H) => {
        Card {
            suit: French::HEARTS,
            rank: French::NINE,
        }
    };
    (8H) => {
        Card {
            suit: French::HEARTS,
            rank: French::EIGHT,
        }
    };
    (7H) => {
        Card {
            suit: French::HEARTS,
            rank: French::SEVEN,
        }
    };
    (6H) => {
        Card {
            suit: French::HEARTS,
            rank: French::SIX,
        }
    };
    (5H) => {
        Card {
            suit: French::HEARTS,
            rank: French::FIVE,
        }
    };
    (4H) => {
        Card {
            suit: French::HEARTS,
            rank: French::FOUR,
        }
    };
    (3H) => {
        Card {
            suit: French::HEARTS,
            rank: French::TREY,
        }
    };
    (2H) => {
        Card {
            suit: French::HEARTS,
            rank: French::DEUCE,
        }
    };
    (AD) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::ACE,
        }
    };
    (KD) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::KING,
        }
    };
    (QD) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::QUEEN,
        }
    };
    (JD) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::JACK,
        }
    };
    (TD) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::TEN,
        }
    };
    (9D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::NINE,
        }
    };
    (8D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::EIGHT,
        }
    };
    (7D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::SEVEN,
        }
    };
    (6D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::SIX,
        }
    };
    (5D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::FIVE,
        }
    };
    (4D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::FOUR,
        }
    };
    (3D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::TREY,
        }
    };
    (2D) => {
        Card {
            suit: French::DIAMONDS,
            rank: French::DEUCE,
        }
    };
    (AC) => {
        Card {
            suit: French::CLUBS,
            rank: French::ACE,
        }
    };
    (KC) => {
        Card {
            suit: French::CLUBS,
            rank: French::KING,
        }
    };
    (QC) => {
        Card {
            suit: French::CLUBS,
            rank: French::QUEEN,
        }
    };
    (JC) => {
        Card {
            suit: French::CLUBS,
            rank: French::JACK,
        }
    };
    (TC) => {
        Card {
            suit: French::CLUBS,
            rank: French::TEN,
        }
    };
    (9C) => {
        Card {
            suit: French::CLUBS,
            rank: French::NINE,
        }
    };
    (8C) => {
        Card {
            suit: French::CLUBS,
            rank: French::EIGHT,
        }
    };
    (7C) => {
        Card {
            suit: French::CLUBS,
            rank: French::SEVEN,
        }
    };
    (6C) => {
        Card {
            suit: French::CLUBS,
            rank: French::SIX,
        }
    };
    (5C) => {
        Card {
            suit: French::CLUBS,
            rank: French::FIVE,
        }
    };
    (4C) => {
        Card {
            suit: French::CLUBS,
            rank: French::FOUR,
        }
    };
    (3C) => {
        Card {
            suit: French::CLUBS,
            rank: French::TREY,
        }
    };
    (2C) => {
        Card {
            suit: French::CLUBS,
            rank: French::DEUCE,
        }
    };
    (__) => {
        Card::<French, French>::default()
    };
    ($rank:expr, $suit:expr) => {
        Card::<French, French>::new(Rank::<French>::from($rank), Suit::<French>::from($suit))
    };
    ($card_str:expr) => {
        Card::<French, French>::from_str($card_str)
            .unwrap_or_else(|_| Card::<French, French>::default())
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__macros__tests {
    use crate::prelude::*;

    #[test]
    fn card_macro__AS() {
        assert_eq!(card!(AS), Card::<French, French>::from_str("A♠").unwrap());
    }
}
