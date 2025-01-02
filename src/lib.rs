#![warn(clippy::pedantic)]

//! Library to represent various decks of playing cards.
//!
//! ```rust
//! use std::collections::HashMap;
//! use colored::Color;
//! use cardpack::prelude::{Card, Decked, Rank, Ranked, French, Suit, Suited};
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
