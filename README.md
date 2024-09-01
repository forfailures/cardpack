[![Build and Test](https://github.com/forfailures/cardpack/actions/workflows/CI.yaml/badge.svg)](https://github.com/forfailures/cardpack/actions/workflows/CI.yaml)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)

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