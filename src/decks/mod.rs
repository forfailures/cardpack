pub mod canasta;
pub mod euchre24;
pub mod french;
pub mod hand_and_foot;
pub mod modern;
pub mod pinochle;
pub mod short;
pub mod skat;
pub mod spades;
pub mod tarot;

pub use crate::decks::canasta::Canasta;
pub use crate::decks::euchre24::Euchre24;
pub use crate::decks::french::French;
pub use crate::decks::french::FrenchCard;
pub use crate::decks::french::FrenchDeck;
pub use crate::decks::french::Standard52;
pub use crate::decks::hand_and_foot::HandAndFoot;
pub use crate::decks::modern::Modern;
pub use crate::decks::modern::ModernCard;
pub use crate::decks::modern::ModernDeck;
pub use crate::decks::pinochle::Pinochle;
pub use crate::decks::short::Short;
pub use crate::decks::short::ShortCard;
pub use crate::decks::short::ShortDeck;
pub use crate::decks::skat::Skat;
pub use crate::decks::tarot::Tarot;
pub use crate::localization::FluentName;
pub use crate::localization::Named;
