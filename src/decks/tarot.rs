use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked, Suited};

// Tarot Deck Suit Fluent Identifiers
pub const MAJOR_ARCANA: &str = "major-arcana";
pub const WANDS: &str = "wands";
pub const CUPS: &str = "cups";
pub const SWORDS: &str = "swords";
pub const PENTACLES: &str = "pentacles";

// Tarot Deck Ranks:
pub const FOOL: &str = "fool";
pub const MAGICIAN: &str = "magician";
pub const PRIESTESS: &str = "priestess";
pub const EMPRESS: &str = "empress";
pub const EMPEROR: &str = "emperor";
pub const HIEROPHANT: &str = "hierophant";
pub const LOVERS: &str = "lovers";
pub const CHARIOT: &str = "chariot";
pub const STRENGTH: &str = "strength";
pub const HERMIT: &str = "hermit";
pub const FORTUNE: &str = "fortune";
pub const JUSTICE: &str = "justice";
pub const HANGED: &str = "hanged";
pub const DEATH: &str = "death";
pub const TEMPERANCE: &str = "temperance";
pub const DEVIL: &str = "devil";
pub const TOWER: &str = "tower";
pub const STAR: &str = "star";
pub const MOON: &str = "moon";
pub const SUN: &str = "sun";
pub const JUDGEMENT: &str = "judgement";
pub const WORLD: &str = "world";
pub const KNIGHT: &str = "knight";
pub const PAGE: &str = "page";

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

/// The great thing about trying to get T
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tarot {}

impl Tarot {
    fn major_arcana() -> Pile<Tarot, Tarot> {
        let mut pile = Pile::<Tarot, Tarot>::new(Vec::new());

        pile.push(Tarot::major_arcana_card_factory(FOOL));
        pile.push(Tarot::major_arcana_card_factory(MAGICIAN));
        pile.push(Tarot::major_arcana_card_factory(PRIESTESS));
        pile.push(Tarot::major_arcana_card_factory(EMPRESS));
        pile.push(Tarot::major_arcana_card_factory(EMPEROR));
        pile.push(Tarot::major_arcana_card_factory(HIEROPHANT));
        pile.push(Tarot::major_arcana_card_factory(LOVERS));
        pile.push(Tarot::major_arcana_card_factory(CHARIOT));
        pile.push(Tarot::major_arcana_card_factory(STRENGTH));
        pile.push(Tarot::major_arcana_card_factory(HERMIT));
        pile.push(Tarot::major_arcana_card_factory(FORTUNE));
        pile.push(Tarot::major_arcana_card_factory(JUSTICE));
        pile.push(Tarot::major_arcana_card_factory(HANGED));
        pile.push(Tarot::major_arcana_card_factory(DEATH));
        pile.push(Tarot::major_arcana_card_factory(TEMPERANCE));
        pile.push(Tarot::major_arcana_card_factory(DEVIL));
        pile.push(Tarot::major_arcana_card_factory(TOWER));
        pile.push(Tarot::major_arcana_card_factory(STAR));
        pile.push(Tarot::major_arcana_card_factory(MOON));
        pile.push(Tarot::major_arcana_card_factory(SUN));
        pile.push(Tarot::major_arcana_card_factory(JUDGEMENT));
        pile.push(Tarot::major_arcana_card_factory(WORLD));

        pile
    }

    fn major_arcana_card_factory(raw: &str) -> Card<Tarot, Tarot> {
        Card::<Tarot, Tarot>::new(Rank::<Tarot>::new(raw), Suit::<Tarot>::new(MAJOR_ARCANA))
    }

    fn minor_arcana() -> Pile<Tarot, Tarot> {
        let ranks = Rank::<Tarot>::ranks();
        let suits = Suit::<Tarot>::suits();

        let mut pile = Pile::<Tarot, Tarot>::new(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                pile.push(Card::<Tarot, Tarot>::new(rank.clone(), suit.clone()));
            }
        }

        pile
    }
}

impl Decked<Tarot, Tarot> for Tarot {
    fn deck() -> Pile<Tarot, Tarot> {
        let mut major_arcana = Tarot::major_arcana();
        let minor_arcana = Tarot::minor_arcana();

        major_arcana.extend(&minor_arcana);

        major_arcana
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
            Rank::<Tarot>::KING,
            Rank::<Tarot>::QUEEN,
            Rank::<Tarot>::KNIGHT,
            Rank::<Tarot>::PAGE,
            Rank::<Tarot>::TEN,
            Rank::<Tarot>::NINE,
            Rank::<Tarot>::EIGHT,
            Rank::<Tarot>::SEVEN,
            Rank::<Tarot>::SIX,
            Rank::<Tarot>::FIVE,
            Rank::<Tarot>::FOUR,
            Rank::<Tarot>::THREE,
            Rank::<Tarot>::TWO,
            Rank::<Tarot>::ACE,
        ]
    }
}

impl Suited for Tarot {
    fn suit_chars() -> Vec<char> {
        vec![
            'M', 'm', '🪄', 'W', 'w', '🏆', 'C', 'c', '⚔', 'S', 's', '☆', 'P', 'p',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![
            Suit::<Tarot>::WANDS,
            Suit::<Tarot>::CUPS,
            Suit::<Tarot>::SWORDS,
            Suit::<Tarot>::PENTACLES,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__tarot__tests {
    use super::*;

    #[test]
    fn deck() {
        let deck = Tarot::deck();

        println!("{deck}");

        assert_eq!(78, deck.len());
    }
}
