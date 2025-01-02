#![warn(clippy::pedantic)]

//! Library to represent various decks of playing cards.
//!
//! ```rust
//! use std::collections::HashMap;
//! use colored::Color;
//! use cardpack::prelude::{Card, Decked, Ranked, Standard52, Suited};
//!
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
//!     fn rank_names() -> Vec<&'static str> {
//!         vec![
//!             Standard52::ACE,
//!             Standard52::KING,
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
//!     fn suit_names() -> Vec<&'static str> {
//!         vec![
//!             Standard52::SPADES,
//!             Standard52::HEARTS,
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
//! ```
pub mod decks;
pub mod localization;
pub mod prelude;
pub mod types;
