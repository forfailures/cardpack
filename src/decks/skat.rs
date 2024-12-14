use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked, Suited};
use colored::Color;
use std::collections::HashMap;

/// Skat is a German, trick based card game for three players.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Skat {}

impl Skat {
    // Skat Deck Ranks:
    pub const DAUS: &'static str = "daus";
    pub const OBER: &'static str = "ober";
    pub const UNTER: &'static str = "unter";

    // Skat Suit Fluent Identifiers
    pub const EICHEL: &'static str = "eichel"; // Acorns
    pub const LAUB: &'static str = "laub"; // Leaves
    pub const HERZ: &'static str = "herz"; // Hearts
    pub const SHELLEN: &'static str = "schellen"; // Bells
}

impl Decked<Skat, Skat> for Skat {
    #[must_use]
    fn deck() -> Pile<Skat, Skat> {
        let ranks = Rank::<Skat>::ranks_from_array(&Skat::rank_names());
        let suits = Suit::<Skat>::suits();

        let mut pile = Pile::<Skat, Skat>::new(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                pile.push(Card::<Skat, Skat>::new(rank.clone(), suit.clone()));
            }
        }

        pile
    }

    fn pack(&self) -> Pile<Skat, Skat> {
        Skat::deck()
    }
}

impl Ranked for Skat {
    fn rank_chars() -> Vec<char> {
        vec![
            '7', '8', '9', 'T', 't', '0', 'U', 'u', 'O', 'o', 'K', 'k', 'D', 'd',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Skat::DAUS,
            Standard52::TEN,
            Standard52::KING,
            Skat::OBER,
            Skat::UNTER,
            Standard52::NINE,
            Standard52::EIGHT,
            Standard52::SEVEN,
        ]
    }
}

impl Suited for Skat {
    fn colors() -> HashMap<char, Color> {
        let mut mappie = HashMap::new();

        mappie.insert('L', Color::Green);
        mappie.insert('H', Color::Red);
        mappie.insert('S', Color::BrightBlue);

        mappie
    }

    fn suit_chars() -> Vec<char> {
        vec![
            '♧', '♣', 'E', 'e', '♤', '♠', 'L', 'l', '♡', '♥', 'H', 'h', '♢', '♦', 'S', 's',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![Skat::EICHEL, Skat::LAUB, Skat::HERZ, Skat::SHELLEN]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__skat__tests {
    use super::*;
    use crate::types::rank::Rank;

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<Skat>::new_with_weight(Skat::DAUS, 20);

        assert_eq!(rank.weight, 20);
    }

    #[test]
    fn rank__update_weight() {
        let rank = Rank::<Skat>::new(Skat::DAUS);
        let updated_rank = rank.update_weight(21);

        assert_eq!(updated_rank.weight, 21);
    }

    #[test]
    fn pile__sort() {
        let deck = Skat::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        // D♣ K♣ O♣ U♣ T♣ 9♣ 8♣ 7♣ D♠ K♠ O♠ U♠ T♠ 9♠ 8♠ 7♠ D♥ K♥ O♥ U♥ T♥ 9♥ 8♥ 7♥ D♦ K♦ O♦ U♦ T♦ 9♦ 8♦ 7♦
        assert_eq!(deck.to_string(), shuffled.to_string());
    }
}
