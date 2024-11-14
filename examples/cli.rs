use cardpack::decks::euchre24::Euchre24;
use cardpack::decks::manila::Manila;
use cardpack::decks::modern::Modern;
use cardpack::decks::standard52::Standard52;
use cardpack::decks::tarot::Tarot;
use cardpack::types::card_error::CardError;
use cardpack::types::traits::Decked;
use clap::Parser;

/// Run all of the decks with 1 for each:
///
/// `cargo run --example cli -- -etsm -d 1`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'd', long, default_value = "1")]
    decks: usize,

    #[clap(short = 'e', long)]
    euchre: bool,

    #[clap(short = 'm', long)]
    manila: bool,

    #[clap(short = 'j', long)]
    modern: bool,

    #[clap(short = 's', long)]
    standard: bool,

    #[clap(short = 't', long)]
    tarot: bool,
}

fn main() -> Result<(), CardError> {
    let args = Args::parse();

    let decks = args.decks;

    if args.tarot {
        let deck = Tarot::decks(decks);
        let shuffled = deck.shuffle_default();
        println!();
        println!("Tarot Deck:          {deck}");
        println!("Tarot Deck Shuffled: {shuffled}");
    }

    if args.euchre {
        let deck = Euchre24::decks(decks);
        let shuffled = deck.shuffle_default();
        println!();
        println!("Euchre Deck:          {deck}");
        println!("Euchre Deck Shuffled: {shuffled}");
    }

    if args.manila {
        let deck = Manila::decks(decks);
        let shuffled = deck.shuffle_default();
        println!();
        println!("Manila Deck:          {deck}");
        println!("Manila Deck Shuffled: {shuffled}");
    }

    if args.modern {
        let deck = Modern::decks(decks);
        let shuffled = deck.shuffle_default();
        println!();
        println!("Modern Deck:          {deck}");
        println!("Modern Deck Shuffled: {shuffled}");
    }

    if args.standard {
        let deck = Standard52::decks(decks);
        let shuffled = deck.shuffle_default();
        println!();
        println!("Standard Deck:          {deck}");
        println!("Standard Deck Shuffled: {shuffled}");
    }

    Ok(())
}
