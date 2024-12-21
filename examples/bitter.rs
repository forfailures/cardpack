use cardpack::decks::standard52::Standard52;
use cardpack::s52card;
use cardpack::types::card::Card;
use cardpack::types::utils::Bit;
use ckc_rs::CardNumber;
use std::str::FromStr;

fn main() {
    let card = s52card!("AS");
    let ckc_as = Bit::strip_suit_flags(CardNumber::ACE_SPADES);

    println!("{}", Bit::string_guided(ckc_as));
    println!("{}", Bit::string(CardNumber::ACE_SPADES));
    println!("{}", Bit::string(card.rank.ckc_number()));
}
