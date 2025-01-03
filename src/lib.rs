#![warn(clippy::pedantic)]

//! Library to represent various decks of playing cards. The library is designed to support the
//! following features:
//!
//! - Custom [`Rank`](types::rank::Rank) and [`Suit`](types::suit::Suit) types.
//! - Ability to sort a [`Pile`](types::pile::Pile) of [`Cards`](types::card::Card)  in various ways.
//! - Localization of card names using [fluent-templates](https://github.com/XAMPPRocky/fluent-templates).
//!
//! Currently, the library supports the following decks:
//!
//! ## Standard 52 Card French Deck
//!
//!
//!
//! ```rust
//! use cardpack::prelude::*;
//!
//! let mut french_deck = French::deck();
//!
//! assert_eq!(
//!     french_deck.to_string(),
//!     "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
//! );
//! assert!(french_deck.contains(&card!(AS)));
//!
//! let shuffled = french_deck.shuffle_default();
//! let parsed = cards!(shuffled.to_string().as_str()).unwrap();
//!
//! // Verify that the cards, in any order, are the same:
//! assert!(french_deck.same(&parsed));
//!
//! // When sorted, they should be exactly the same:
//! assert_eq!(parsed.sort(), french_deck);
//!
//! let royal_flush = french_deck.draw(5);
//! assert_eq!(royal_flush.to_string(), "A♠ K♠ Q♠ J♠ T♠");
//! assert_eq!(royal_flush.index(), "AS KS QS JS TS");
//!
//! // The original deck should now have five cards less:
//! assert_eq!(french_deck.len(), 47);
//!
//! // Cards can provide a longer description in English and German:
//! assert_eq!(card!(AS).long(&FluentName::US_ENGLISH), "Ace Spades");
//! assert_eq!(card!(QH).long(&FluentName::DEUTSCH), "Dame Herzen");
//! ```
//!
//! ## Modern Deck
//!
//! A Modern deck is a French deck with two jokers.
//!
//! ```rust
//! use cardpack::prelude::*;
//!
//! let mut modern_deck = Modern::deck();
//!
//! ```
//!
//! ```rust
//! use std::collections::HashMap;
//! use colored::Color;
//! use cardpack::prelude::*;
//!
//! #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
//! pub struct Tiny {}
//!
//! impl Tiny {
//!    pub const DECK_NAME: &'static str = "Tiny";
//! }
//!
//! impl Ranked for Tiny {
//!     fn rank_chars() -> Vec<char> {
//!         vec!['A', 'a', 'K', 'k']
//!     }
//!
//!     // Since the rank names are the same as the French deck, we can simply just use those:
//!     fn rank_names() -> Vec<&'static str> {
//!         vec![
//!             French::ACE,
//!             French::KING,
//!         ]
//!     }
//!
//!
//!     fn type_name() -> &'static str {
//!         Tiny::DECK_NAME
//!     }
//! }
//!
//! impl Suited for Tiny {
//!     fn colors() -> HashMap<char, Color> {
//!         let mut mappie = HashMap::new();
//!         mappie.insert('H', Color::Red);
//!         mappie
//!     }
//!
//!     fn suit_chars() -> Vec<char> {
//!         vec!['♤', '♠', 'S', 's', '♡', '♥', 'H', 'h',]
//!     }
//!
//!     // And the suit names are the same as the French deck as well:
//!     fn suit_names() -> Vec<&'static str> {
//!         vec![
//!             French::SPADES,
//!             French::HEARTS,
//!         ]
//!     }
//!
//!     fn type_name() -> &'static str {
//!         Tiny::DECK_NAME
//!     }
//! }
//!
//! impl Decked<Tiny, Tiny> for Tiny {
//!     fn blank() -> Card<Tiny, Tiny> {
//!         Card::<Tiny, Tiny>::default()
//!     }
//!
//!     fn guide() -> Option<String> {
//!         todo!()
//!     }
//! }
//!
//! macro_rules! tiny {
//!     (AS) => {
//!         Card::<Tiny, Tiny>::new(Rank::<Tiny>::new(French::ACE), Suit::<Tiny>::new(French::SPADES))
//!     };
//!     (KS) => {
//!         Card::<Tiny, Tiny>::new(Rank::<Tiny>::new(French::KING), Suit::<Tiny>::new(French::SPADES))
//!     };
//!     (AH) => {
//!         Card::<Tiny, Tiny>::new(Rank::<Tiny>::new(French::ACE), Suit::<Tiny>::new(French::HEARTS))
//!     };
//!     (KH) => {
//!         Card::<Tiny, Tiny>::new(Rank::<Tiny>::new(French::KING), Suit::<Tiny>::new(French::HEARTS))
//!     };
//! }
//!
//! let mut deck = Tiny::deck();
//!
//! assert_eq!(deck.to_string(), "A♠ K♠ A♥ K♥");
//!
//! // Deal from the top of the deck:
//! assert_eq!(deck.draw_first().unwrap(), tiny!(AS));
//!
//! // Deal from the bottom of the deck:
//! assert_eq!(deck.draw_last().unwrap(), tiny!(KH));
//!
//! // Should be two cards remaining:
//! assert_eq!(deck.len(), 2);
//! assert_eq!(deck.index(), "KS AH");
//!
//! // Draw the top card and make sure it's got the right Cactus Kev Card Number for the
//! // King of Spades:
//! assert_eq!(deck.draw_first().unwrap().get_ckc_number(), 0b00001000_00000000_10001011_00100101);
//!
//! // Draw the last card:
//! assert_eq!(deck.draw_first().unwrap(), tiny!(AH));
//!
//! // And now the deck is empty:
//! assert!(deck.draw_first().is_none());
//! ```
pub mod decks;
pub mod localization;
pub mod prelude;
pub mod types;
