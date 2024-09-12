use thiserror::Error;

#[derive(Error, Debug)]
pub enum CardError {
    #[error("Invalid Card: `{0}`")]
    InvalidCard(String),
    #[error("Invalid Card Count: `{0}`")]
    InvalidCardCount(usize),
    #[error("Invalid Index: `{0}`")]
    InvalidIndex(String),
    #[error("Not enough cards: `{0}` missing")]
    NotEnoughCards(usize),
    #[error("Too many cards: `{0}` extra")]
    TooManyCards(usize),
}
