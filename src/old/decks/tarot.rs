use crate::old::decks::french::French;
use crate::old::types::card::Card;
use crate::old::types::pile::Pile;
use crate::old::types::rank::Rank;
use crate::old::types::suit::Suit;
use crate::old::types::traits::{Decked, Ranked, Suited};
use crate::types::errors::CardError;
use colored::Color;
use std::collections::HashMap;
use std::str::FromStr;

// #[macro_export] These macros doesn't work yet.
#[allow(unused_macros, clippy::pedantic)]
macro_rules! tarot_card {
    ($card_str:expr) => {
        Card::<Tarot, Tarot>::from_str($card_str)
            .unwrap_or_else(|_| Card::<Tarot, Tarot>::default())
    };
}

// #[macro_export]
#[allow(unused_macros, clippy::pedantic)]
macro_rules! tarot {
    ($card_str:expr) => {
        Pile::<Tarot, Tarot>::from_str($card_str)
    };
}

/// The great thing about trying to get T
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tarot {}

impl Tarot {
    pub const DECK_NAME: &'static str = "Tarot";

    //  Suit Fluent Identifiers
    pub const MAJOR_ARCANA: &'static str = "major-arcana";
    pub const WANDS: &'static str = "wands";
    pub const CUPS: &'static str = "cups";
    pub const SWORDS: &'static str = "swords";
    pub const PENTACLES: &'static str = "pentacles";

    // Suit Symbols
    pub const MAJOR_ARCANA_SYMBOL: char = 'M';
    pub const WANDS_SYMBOL: char = '🪄';
    pub const CUPS_SYMBOL: char = '🏆';
    pub const SWORDS_SYMBOL: char = '⚔';
    pub const PENTACLES_SYMBOL: char = '☆';

    // Rank Fluent Identifiers
    pub const FOOL: &'static str = "fool";
    pub const MAGICIAN: &'static str = "magician";
    pub const PRIESTESS: &'static str = "priestess";
    pub const EMPRESS: &'static str = "empress";
    pub const EMPEROR: &'static str = "emperor";
    pub const HIEROPHANT: &'static str = "hierophant";
    pub const LOVERS: &'static str = "lovers";
    pub const CHARIOT: &'static str = "chariot";
    pub const STRENGTH: &'static str = "strength";
    pub const HERMIT: &'static str = "hermit";
    pub const FORTUNE: &'static str = "fortune";
    pub const JUSTICE: &'static str = "justice";
    pub const HANGED: &'static str = "hanged";
    pub const DEATH: &'static str = "death";
    pub const TEMPERANCE: &'static str = "temperance";
    pub const DEVIL: &'static str = "devil";
    pub const TOWER: &'static str = "tower";
    pub const STAR: &'static str = "star";
    pub const MOON: &'static str = "moon";
    pub const SUN: &'static str = "sun";
    pub const JUDGEMENT: &'static str = "judgement";
    pub const WORLD: &'static str = "world";
    pub const KNIGHT: &'static str = "knight";
    pub const PAGE: &'static str = "page";

    // Tarot Deck Rank Symbols
    pub const FOOL_SYMBOL: char = '🤡';
    pub const MAGICIAN_SYMBOL: char = '🧙';
    pub const PRIESTESS_SYMBOL: char = '😇';
    pub const EMPRESS_SYMBOL: char = '👑';
    pub const EMPEROR_SYMBOL: char = '🤴';
    pub const HIEROPHANT_SYMBOL: char = '🧎';
    pub const LOVERS_SYMBOL: char = '💏';
    pub const CHARIOT_SYMBOL: char = '🏎';
    pub const STRENGTH_SYMBOL: char = '💪';
    pub const HERMIT_SYMBOL: char = '💡';
    pub const FORTUNE_SYMBOL: char = '🍀';
    pub const JUSTICE_SYMBOL: char = '⚖';
    pub const HANGED_SYMBOL: char = '🙃';
    pub const DEATH_SYMBOL: char = '💀';
    pub const TEMPERANCE_SYMBOL: char = '🚭';
    pub const DEVIL_SYMBOL: char = '😈';
    pub const TOWER_SYMBOL: char = '🏢';
    pub const STAR_SYMBOL: char = '⭐';
    pub const MOON_SYMBOL: char = '🌙';
    pub const SUN_SYMBOL: char = '🌞';
    pub const JUDGEMENT_SYMBOL: char = '🔔';
    pub const WORLD_SYMBOL: char = '🌍';
    pub const KNIGHT_SYMBOL: char = '🗡';
    pub const PAGE_SYMBOL: char = '📜';

    fn major_arcana() -> Pile<Tarot, Tarot> {
        let mut pile = Pile::<Tarot, Tarot>::from(Vec::new());

        for rank in Tarot::major_arcana_rank_names() {
            pile.push(Tarot::major_arcana_card_factory(rank));
        }

        pile
    }

    fn major_arcana_card_factory(raw: &str) -> Card<Tarot, Tarot> {
        Card::<Tarot, Tarot>::new(
            Rank::<Tarot>::new(raw),
            Suit::<Tarot>::new(Tarot::MAJOR_ARCANA),
        )
    }

    fn major_arcana_rank_names() -> Vec<&'static str> {
        vec![
            Tarot::FOOL,
            Tarot::MAGICIAN,
            Tarot::PRIESTESS,
            Tarot::EMPRESS,
            Tarot::EMPEROR,
            Tarot::HIEROPHANT,
            Tarot::LOVERS,
            Tarot::CHARIOT,
            Tarot::STRENGTH,
            Tarot::HERMIT,
            Tarot::FORTUNE,
            Tarot::JUSTICE,
            Tarot::HANGED,
            Tarot::DEATH,
            Tarot::TEMPERANCE,
            Tarot::DEVIL,
            Tarot::TOWER,
            Tarot::STAR,
            Tarot::MOON,
            Tarot::SUN,
            Tarot::JUDGEMENT,
            Tarot::WORLD,
        ]
    }

    fn minor_arcana() -> Pile<Tarot, Tarot> {
        let ranks = Rank::<Tarot>::ranks_from_array(&Tarot::rank_names());
        let suits = Suit::<Tarot>::suits();

        let mut pile = Pile::<Tarot, Tarot>::from(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                pile.push(Card::<Tarot, Tarot>::new(rank.clone(), suit.clone()));
            }
        }

        pile
    }

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Tarot, Tarot>, CardError> {
        Pile::<Tarot, Tarot>::from_str(index)
    }
}

impl Decked<Tarot, Tarot> for Tarot {
    fn deck() -> Pile<Tarot, Tarot> {
        let mut major_arcana = Tarot::major_arcana();
        let minor_arcana = Tarot::minor_arcana();

        major_arcana.extend(&minor_arcana);

        major_arcana
    }

    fn blank() -> Card<Tarot, Tarot> {
        Card::<Tarot, Tarot>::default()
    }

    fn guide() -> Option<String> {
        None
    }
}

impl Ranked for Tarot {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'P', 'p', 'J', 'j', 'Q', 'q',
            'K', 'k', 'A', 'a', '🤡', '🧙', '😇', '👑', '🤴', '🧎', '💏', '🏎', '💪', '💡', '🍀',
            '⚖', '🙃', '💀', '🚭', '😈', '🏢', '⭐', '🌙', '🌞', '🔔', '🌍', '🗡', '📜',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            French::KING,
            French::QUEEN,
            Tarot::KNIGHT,
            Tarot::PAGE,
            French::TEN,
            French::NINE,
            French::EIGHT,
            French::SEVEN,
            French::SIX,
            French::FIVE,
            French::FOUR,
            French::THREE,
            French::TWO,
            French::ACE,
        ]
    }

    fn type_name() -> &'static str {
        Tarot::DECK_NAME
    }
}

impl Suited for Tarot {
    fn colors() -> HashMap<char, Color> {
        let mut mappie = HashMap::new();

        mappie.insert('M', Color::Blue);
        mappie.insert('H', Color::Red);
        mappie.insert('D', Color::Red);

        mappie
    }

    fn suit_chars() -> Vec<char> {
        vec![
            Tarot::MAJOR_ARCANA_SYMBOL,
            'M',
            'm',
            Tarot::WANDS_SYMBOL,
            'W',
            'w',
            Tarot::CUPS_SYMBOL,
            'C',
            'c',
            Tarot::SWORDS_SYMBOL,
            'S',
            's',
            Tarot::PENTACLES_SYMBOL,
            'P',
            'p',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![Tarot::WANDS, Tarot::CUPS, Tarot::SWORDS, Tarot::PENTACLES]
    }

    fn type_name() -> &'static str {
        Tarot::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__tarot__tests {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn macro__tarot_card() {
        let card = tarot_card!("🤡M");

        assert_eq!(card.to_string(), "🤡M");
    }

    #[test]
    fn deck() {
        let deck = Tarot::deck();

        assert_eq!(78, deck.len());
    }

    #[test]
    fn display() {
        let deck = Tarot::deck();

        assert_eq!(deck.to_string(), "🤡M 🧙M 😇M 👑M 🤴M 🧎M 💏M 🏎\u{fe0f}M 💪M 🧑\u{200d}🌾M 🍀M ⚖M 🙃M 💀M 🚭M 😈M 🏢M ⭐M 🌙M 🌞M 🔔M 🌍M K🪄 Q🪄 J🪄 P🪄 T🪄 9🪄 8🪄 7🪄 6🪄 5🪄 4🪄 3🪄 2🪄 A🪄 K🏆 Q🏆 J🏆 P🏆 T🏆 9🏆 8🏆 7🏆 6🏆 5🏆 4🏆 3🏆 2🏆 A🏆 K⚔ Q⚔ J⚔ P⚔ T⚔ 9⚔ 8⚔ 7⚔ 6⚔ 5⚔ 4⚔ 3⚔ 2⚔ A⚔ K☆ Q☆ J☆ P☆ T☆ 9☆ 8☆ 7☆ 6☆ 5☆ 4☆ 3☆ 2☆ A☆");
    }

    #[test]
    fn pile__sort() {
        let deck = Tarot::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    // #[rstest]
    // #[ignore]
    // #[case("🤡M")]
    // #[case("🧙M")]
    // #[case("😇M")]
    // #[case("👑M")]
    // #[case("🤴M")]
    // #[case("🧎M")]
    // #[case("💏M")]
    // // #[case("🚗M")]
    // fn card(#[case] s: &str) {
    //     let s = s.trim();
    //     if let Some(c) = s.chars().next() {
    //         let rank = Rank::<Tarot>::from(c);
    //         println!("{:?}", rank);
    //         if let Some(d) = s.chars().last() {
    //             let suit = Suit::<Tarot>::from(d);
    //             println!("{:?}", suit);
    //         }
    //     }
    //     let card = tarot_card!(s);
    //     println!("{:?}", card);
    //     println!();
    // }

    #[test]
    #[ignore]
    fn to_string__from_str() {
        let deck = Tarot::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = Tarot::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
