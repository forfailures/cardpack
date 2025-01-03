#![warn(clippy::pedantic)]

//! Library to represent various decks of playing cards. The library is designed to support the
//! following features:
//!
//! - Custom [`Rank`](types::rank::Rank) and [`Suit`](types::suit::Suit) types.
//! - Ability to sort a [`Pile`](types::pile::Pile) of [`Cards`](types::card::Card)  in various ways.
//! - Localization of card names using [fluent-templates](https://github.com/XAMPPRocky/fluent-templates).
//!
//! ## Overview
//!
//! The structure of the library is the following: A [`Pile`](types::pile::Pile) is a collection of Cards
//! that have a [`Rank`](types::rank::Rank) that implements the [`Ranked`](types::traits::Ranked) and
//! a Suit that implements the [`Suited`](types::traits::Suited) trait.
//!
//! The library supports the following decks:
//!
//! ## Standard 52 Card French Deck
//!
//! The Standard 52 Card [`French`](decks::french::French) deck is the most common deck of playing cards.
//! It is made up of a [`Pile`](types::pile::Pile) of 52 [`Cards`](types::card::Card) with 13 ranks in each of the four suits.
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
//! An important thing to remember about the decks is that the cards have their weight inside them
//! to facilitate sorting. If you wanted a deck for Razz poker, where the lowest hand wins, you
//! would need to create a separate deck file with the card's `Rank` weights adjusted.
//!
//! ## Modern Deck
//!
//! A [`Modern`](decks::modern::Modern) deck is a French deck with two jokers.
//!
//! ```rust
//! use cardpack::prelude::*;
//!
//! let mut modern_deck = Modern::deck();
//!
//! assert_eq!(modern_deck.len(), 54);
//!
//! // For a joker card's index string, `B` stands for Big Joker and `L` for Little Joker,
//! // with `🃟` being the symbol character for the trump suit.
//! assert_eq!(modern_deck.draw(3).to_string(), "B🃟 L🃟 A♠");
//! ```
//!
//! Other decks include:
//!
//! - [`Canasta`](decks::canasta::Canasta) - 2 Modern decks with the red 3s made trumps.
//! - [`Euchre`](decks::euchre24::Euchre24) - A 24 card version of a Euchre deck.
//! - [`HandAndFoot`](decks::hand_and_foot::HandAndFoot) - 5 Modern decks.
//! - [`Manila`](decks::manila::Manila) - A 36 card deck with ranks 6 through Ace.
//! - [`Pinochle`](decks::pinochle::Pinochle) - A 48 card deck with two copies of the 9 through Ace ranks.
//! - [`Skat`](decks::skat::Skat) - A 32 card German card game with different suits and ranks.
//! - [`Spades`](decks::spades::Spades) - A Modern deck with the 2 of Clubs and 2 of Diamonds removed.
//! - [`Tarot`](decks::tarot::Tarot) - A 78 card deck with 22 Major Arcana and 56 Minor Arcana cards.
//!
//! ## Custom Deck example:
//!
//! Here's a very simple example where we create a tiny deck with only the ace and kink ranks,
//! and only the spades and hearts suits. Just for fun, we'll include macro for one `Tiny` card.
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
