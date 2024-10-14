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
pub const FOOL_SYMBOL: &str = "🤡";
pub const MAGICIAN_SYMBOL: &str = "🪄";
pub const PRIESTESS_SYMBOL: &str = "😇";
pub const EMPRESS_SYMBOL: &str = "👑";
pub const EMPEROR_SYMBOL: &str = "🤴";
pub const HIEROPHANT_SYMBOL: &str = "🧎";
pub const LOVERS_SYMBOL: &str = "💏";
pub const CHARIOT_SYMBOL: &str = "🏎️";
pub const STRENGTH_SYMBOL: &str = "💪";
pub const HERMIT_SYMBOL: &str = "🧙";
pub const FORTUNE_SYMBOL: &str = "🍀";
pub const JUSTICE_SYMBOL: &str = "⚖️";
pub const HANGED_SYMBOL: &str = "🙃";
pub const DEATH_SYMBOL: &str = "💀";
pub const TEMPERANCE_SYMBOL: &str = "🍷";
pub const DEVIL_SYMBOL: &str = "😈";
pub const TOWER_SYMBOL: &str = "🏢";
pub const STAR_SYMBOL: &str = "⭐";
pub const MOON_SYMBOL: &str = "🌙";
pub const SUN_SYMBOL: &str = "☀️";
pub const JUDGEMENT_SYMBOL: &str = "🔔";
pub const WORLD_SYMBOL: &str = "🌍";
pub const KNIGHT_SYMBOL: &str = "🗡️";
pub const PAGE_SYMBOL: &str = "📜";

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
