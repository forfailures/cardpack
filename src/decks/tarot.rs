use crate::types::pile::Pile;
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

impl Decked<Tarot, Tarot> for Tarot {
    fn deck() -> Pile<Tarot, Tarot> {
        todo!()
    }
}

impl Ranked for Tarot {
    fn rank_chars() -> Vec<char> {
        todo!()
    }

    fn rank_names() -> Vec<&'static str> {
        todo!()
    }
}

impl Suited for Tarot {
    fn suit_chars() -> Vec<char> {
        todo!()
    }

    fn suit_names() -> Vec<&'static str> {
        todo!()
    }
}
