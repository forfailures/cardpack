[![Build and Test](https://github.com/forfailures/cardpack/actions/workflows/CI.yaml/badge.svg)](https://github.com/forfailures/cardpack/actions/workflows/CI.yaml)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)

WIP Complete rewrite of my [cardpack.rs](https://github.com/ImperialBower/cardpack.rs) 
library, taking advantages of generics to "simplify" the library. 

---
Generic Deck of Cards Library in Rust

The goal of this library is to be able to support the creation of card
decks of various sizes and suites. Possibilities include:

* [French Deck](https://en.wikipedia.org/wiki/French_playing_cards)
  * [Canasta](https://en.wikipedia.org/wiki/Canasta#Cards_and_deal)
  * [Euchre](https://en.wikipedia.org/wiki/Euchre)
  * [Pinochle](https://en.wikipedia.org/wiki/Pinochle#Deck)
  * [Spades](https://en.wikipedia.org/wiki/Spades_(card_game)#General_overview) with [Jokers](https://en.wikipedia.org/wiki/Joker_(playing_card))
  * [Standard 52](https://en.wikipedia.org/wiki/Standard_52-card_deck)
* [Skat](https://en.wikipedia.org/wiki/Skat_(card_game)#Deck)
* [Tarot](https://en.wikipedia.org/wiki/Tarot#Tarot_gaming_decks)

## Responsibilities

* Represent a specific type of card deck.
* Validate that a collection of cards is valid for that type of deck.
* Create a textual representation of a deck that can be serialized and deserialized.
* Shuffle a deck
* Verify that a specific card is playable given a set of discards.
* Determine if two deck types are translatable.

## Example

```rust
use cardpack::prelude::*;

fn main() {
    let pack = French::deck();
    let mut shuffled = pack.shuffle();

    let small_blind = shuffled.draw(2);
    let big_blind = shuffled.draw(2);

    println!("small blind: {}", small_blind.to_string());
    println!("big blind:   {}", big_blind.to_string());

    println!();
    println!("flop : {}", shuffled.draw(3).to_string());
    println!("turn : {}", shuffled.draw(1).to_string());
    println!("river: {}", shuffled.draw(1).to_string());
}
```

## Dependencies

* [Clap](https://crates.io/crates/clap)
* [colored](https://github.com/colored-rs/colored)
* [hashbag](https://github.com/jonhoo/hashbag)
* [log](https://github.com/rust-lang/log)
* [rand](https://crates.io/crates/rand)
  * [The Rust Rand Book](https://rust-random.github.io/book/)
* [thiserror](https://github.com/dtolnay/thiserror)

## Refactoring Goals

* Use generics to represent cards and decks.
* Good error reporting
