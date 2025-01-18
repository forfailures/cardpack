use cardpack::old_prelude::*;

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
