use cardpack::decks::canasta::Canasta;
use cardpack::decks::euchre24::Euchre24;
use cardpack::decks::hand_and_foot::HandAndFoot;
use cardpack::decks::manila::Manila;
use cardpack::decks::modern::Modern;
use cardpack::decks::pinochle::Pinochle;
use cardpack::decks::skat::Skat;
use cardpack::decks::spades::Spades;
use cardpack::decks::standard52::Standard52;
use cardpack::decks::tarot::Tarot;
use cardpack::types::card_error::CardError;
use cardpack::types::traits::Decked;
use clap::Parser;

/// Run all of the decks with 1 for each:
///
/// `cargo run --example cli -- -emjkpsac`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'c', long)]
    canasta: bool,

    #[clap(short = 'e', long)]
    euchre: bool,

    #[clap(short = 'f', long)]
    hand_and_foot: bool,

    #[clap(short = 'm', long)]
    manila: bool,

    #[clap(short = 'j', long)]
    modern: bool,

    #[clap(short = 'p', long)]
    pinochle: bool,

    #[clap(short = 'k', long)]
    skat: bool,

    #[clap(short = 'a', long)]
    spades: bool,

    #[clap(short = 's', long)]
    standard: bool,

    #[clap(short = 't', long)]
    tarot: bool,
}

fn main() -> Result<(), CardError> {
    let args = Args::parse();

    if args.tarot {
        Tarot::demo();
    }

    if args.canasta {
        Canasta::demo();
    }

    if args.euchre {
        Euchre24::demo();
    }

    if args.manila {
        Manila::demo();
    }

    if args.modern {
        Modern::demo();
    }

    if args.spades {
        Spades::demo();
    }

    if args.pinochle {
        Pinochle::demo();
    }

    if args.skat {
        Skat::demo();
    }

    if args.hand_and_foot {
        HandAndFoot::demo();
    }

    if args.standard {
        Standard52::demo();
    }

    Ok(())
}
