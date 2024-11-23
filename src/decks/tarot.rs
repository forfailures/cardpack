use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked, Suited};

/// The great thing about trying to get T
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tarot {}

impl Tarot {
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
        let mut pile = Pile::<Tarot, Tarot>::new(Vec::new());

        pile.push(Tarot::major_arcana_card_factory(Tarot::FOOL));
        pile.push(Tarot::major_arcana_card_factory(Tarot::MAGICIAN));
        pile.push(Tarot::major_arcana_card_factory(Tarot::PRIESTESS));
        pile.push(Tarot::major_arcana_card_factory(Tarot::EMPRESS));
        pile.push(Tarot::major_arcana_card_factory(Tarot::EMPEROR));
        pile.push(Tarot::major_arcana_card_factory(Tarot::HIEROPHANT));
        pile.push(Tarot::major_arcana_card_factory(Tarot::LOVERS));
        pile.push(Tarot::major_arcana_card_factory(Tarot::CHARIOT));
        pile.push(Tarot::major_arcana_card_factory(Tarot::STRENGTH));
        pile.push(Tarot::major_arcana_card_factory(Tarot::HERMIT));
        pile.push(Tarot::major_arcana_card_factory(Tarot::FORTUNE));
        pile.push(Tarot::major_arcana_card_factory(Tarot::JUSTICE));
        pile.push(Tarot::major_arcana_card_factory(Tarot::HANGED));
        pile.push(Tarot::major_arcana_card_factory(Tarot::DEATH));
        pile.push(Tarot::major_arcana_card_factory(Tarot::TEMPERANCE));
        pile.push(Tarot::major_arcana_card_factory(Tarot::DEVIL));
        pile.push(Tarot::major_arcana_card_factory(Tarot::TOWER));
        pile.push(Tarot::major_arcana_card_factory(Tarot::STAR));
        pile.push(Tarot::major_arcana_card_factory(Tarot::MOON));
        pile.push(Tarot::major_arcana_card_factory(Tarot::SUN));
        pile.push(Tarot::major_arcana_card_factory(Tarot::JUDGEMENT));
        pile.push(Tarot::major_arcana_card_factory(Tarot::WORLD));

        pile
    }

    fn major_arcana_card_factory(raw: &str) -> Card<Tarot, Tarot> {
        Card::<Tarot, Tarot>::new(
            Rank::<Tarot>::new(raw),
            Suit::<Tarot>::new(Tarot::MAJOR_ARCANA),
        )
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
            Standard52::KING,
            Standard52::QUEEN,
            Tarot::KNIGHT,
            Tarot::PAGE,
            Standard52::TEN,
            Standard52::NINE,
            Standard52::EIGHT,
            Standard52::SEVEN,
            Standard52::SIX,
            Standard52::FIVE,
            Standard52::FOUR,
            Standard52::THREE,
            Standard52::TWO,
            Standard52::ACE,
        ]
    }
}

impl Suited for Tarot {
    fn suit_chars() -> Vec<char> {
        vec![
            Tarot::MAJOR_ARCANA_SYMBOL,
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
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__tarot__tests {
    use super::*;

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
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }
}
