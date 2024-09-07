#![warn(clippy::pedantic)]

use std::error::Error;

mod fluent;

#[derive(Debug)]
pub enum CardError {
    InvalidCard(String),
    InvalidCardCount,
    InvalidIndex(String),
    NotEnoughCards,
    TooManyCards,
}

// https://github.com/dtolnay/thiserror
impl std::fmt::Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for CardError {}