use cardpack::old::decks::skat::Skat;
use cardpack::old::types::traits::Decked;
use cardpack::old_prelude::French;

fn main() {
    let deck = Skat::deck();

    for card in deck.v() {
        println!(
            "{} \n{}",
            card.index,
            Skat::string_guided(card.get_ckc_number())
        );
    }

    let deck = French::deck();

    for card in deck.v() {
        println!(
            "{} \n{}",
            card.index,
            Skat::string_guided(card.get_ckc_number())
        );
    }
}
