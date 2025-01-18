use cardpack::old::decks::canasta::Canasta;
use cardpack::old::decks::euchre24::Euchre24;
use cardpack::old::decks::french::French;
use cardpack::old::decks::hand_and_foot::HandAndFoot;
use cardpack::old::decks::modern::Modern;
use cardpack::old::decks::pinochle::Pinochle;
use cardpack::old::decks::short::Short;
use cardpack::old::decks::skat::Skat;
use cardpack::old::decks::spades::Spades;
use cardpack::old::decks::tarot::Tarot;
use cardpack::types::card_error::CardError;
use cardpack::types::traits::Decked;
use clap::Parser;

/// Run all of the decks with 1 for each:
///
/// `cargo run --example cli -- -temjkpsac -v`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'v', long)]
    verbose: bool,

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
        Tarot::demo(args.verbose);
    }

    if args.canasta {
        Canasta::demo(args.verbose);
    }

    if args.euchre {
        Euchre24::demo(args.verbose);
    }

    if args.manila {
        Short::demo(args.verbose);
    }

    if args.modern {
        Modern::demo(args.verbose);
    }

    if args.spades {
        Spades::demo(args.verbose);
    }

    if args.pinochle {
        Pinochle::demo(args.verbose);
    }

    if args.skat {
        Skat::demo(args.verbose);
    }

    if args.hand_and_foot {
        HandAndFoot::demo(args.verbose);
    }

    if args.standard {
        French::demo(args.verbose);
    }

    Ok(())
}
