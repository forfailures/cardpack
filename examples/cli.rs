use cardpack::decks::euchre24::Euchre24;
use cardpack::decks::manila::Manila;
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

    #[clap(short = 's', long)]
    standard: bool,

    #[clap(short = 't', long)]
    tarot: bool,
}

fn main() -> Result<(), CardError> {
    let args = Args::parse();

    let decks = args.decks;

    if args.tarot {
        let tarot_deck = Tarot::decks(decks);
        println!();
        println!("Tarot Deck: {tarot_deck}");
    }

    if args.euchre {
        let euchre_deck = Euchre24::decks(decks);
        println!();
        println!("Euchre Deck: {euchre_deck}");
    }

    if args.manila {
        let manila_deck = Manila::decks(decks);
        println!();
        println!("Manila Deck: {manila_deck}");
    }

    if args.standard {
        let standard_deck = Standard52::decks(decks);
        println!();
        println!("Standard Deck: {standard_deck}");
    }

    Ok(())
}
