# Refactoring Notes

## Refactor One: To Generics

I feel like this has been a qualified success. The big problem is that it reveals the underlying 
issues with the structs themselves. They're too heavy. There's no reason they can't be light. 

## Refactor Two: Simplification

GOAL: I want my structs to be copyable. Heavy stuff should come through function calls, not stored automatically. 

The big lesson I've been learning from doing this refactoring, and more importantly in adding all of the 
doctests, os that they structs are too clunky. They are a pain in the ass to instantiate. 

# Jan 14th 2025

The IDE is so slow with the dupes in code and having added so many doc tests. I need to get to a point where I can
trim it all down.

# Jan 15th 2025

TODO: Possible REFACTOR - Add symbol char to struct to avoid need for localization
call. We will save this for after REF2 is complete.

## Removing ::new()

I've decided to remove the Rank and Suit ::new() instantiation function and just use ::new(char). 
This feels more idiomatic to me. 

So glad I can eliminate the need for this:

```rust
impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(c: char) -> Self {
        if !RankType::is_valid_rank_char(&c) {
            return Rank::<RankType> {
                weight: 0,
                index: '_',
                prime: 0,
                name: FluentName::default(),
                phantom_data: PhantomData,
            };
        }
        match RankType::type_name() {
            Skat::DECK_NAME => match c {
                'D' | 'd' => Rank::new(Skat::DAUS),
                'T' | 't' | '0' => Rank::new(Skat::ZHEN),
                'K' | 'k' => Rank::new(Skat::KÖNIG),
                'O' | 'o' => Rank::new(Skat::OBER),
                'U' | 'u' => Rank::new(Skat::UNTER),
                '9' => Rank::new(Skat::NEUN),
                '8' => Rank::new(Skat::ACHT),
                '7' => Rank::new(Skat::SIEBEN),
                _ => Rank::new(FluentName::BLANK),
            },
            Pinochle::DECK_NAME => match c {
                'A' | 'a' => Rank::new(Pinochle::ACE),
                'T' | 't' => Rank::new(Pinochle::TEN),
                'K' | 'k' => Rank::new(Pinochle::KING),
                'Q' | 'q' => Rank::new(Pinochle::QUEEN),
                'J' | 'j' => Rank::new(Pinochle::JACK),
                '9' => Rank::new(Pinochle::NINE),
                _ => Rank::new(FluentName::BLANK),
            },
            Tarot::DECK_NAME => match c {
                '2' => Rank::new(French::TWO),
                '3' => Rank::new(French::THREE),
                '4' => Rank::new(French::FOUR),
                '5' => Rank::new(French::FIVE),
                '6' => Rank::new(French::SIX),
                '7' => Rank::new(French::SEVEN),
                '8' => Rank::new(French::EIGHT),
                '9' => Rank::new(French::NINE),
                'T' | 't' | '0' => Rank::new(French::TEN),
                'P' | 'p' => Rank::new(Tarot::PAGE),
                'J' | 'j' => Rank::new(Tarot::KNIGHT),
                'Q' | 'q' => Rank::new(French::QUEEN),
                'K' | 'k' => Rank::new(French::KING),
                'A' | 'a' => Rank::new(French::ACE),
                '🤡' => Rank::new(Tarot::FOOL),
                '🧙' => Rank::new(Tarot::MAGICIAN),
                '😇' => Rank::new(Tarot::PRIESTESS),
                '👑' => Rank::new(Tarot::EMPRESS),
                '🤴' => Rank::new(Tarot::EMPEROR),
                '🧎' => Rank::new(Tarot::HIEROPHANT),
                '💏' => Rank::new(Tarot::LOVERS),
                '🏎' => Rank::new(Tarot::CHARIOT),
                '💪' => Rank::new(Tarot::STRENGTH),
                '💡' => Rank::new(Tarot::HERMIT),
                '🍀' => Rank::new(Tarot::FORTUNE),
                '⚖' => Rank::new(Tarot::JUSTICE),
                '🙃' => Rank::new(Tarot::HANGED),
                '💀' => Rank::new(Tarot::DEATH),
                '🚭' => Rank::new(Tarot::TEMPERANCE),
                '😈' => Rank::new(Tarot::DEVIL),
                '🏢' => Rank::new(Tarot::TOWER),
                '⭐' => Rank::new(Tarot::STAR),
                '🌙' => Rank::new(Tarot::MOON),
                '🌞' => Rank::new(Tarot::SUN),
                '🔔' => Rank::new(Tarot::JUDGEMENT),
                '🌍' => Rank::new(Tarot::WORLD),
                _ => Rank::new(FluentName::BLANK),
            },
            _ => match c {
                '2' => Rank::new(French::TWO),
                '3' => Rank::new(French::THREE),
                '4' => Rank::new(French::FOUR),
                '5' => Rank::new(French::FIVE),
                '6' => Rank::new(French::SIX),
                '7' => Rank::new(French::SEVEN),
                '8' => Rank::new(French::EIGHT),
                '9' => Rank::new(French::NINE),
                'T' | 't' | '0' => Rank::new(French::TEN),
                'J' | 'j' => Rank::new(French::JACK),
                'Q' | 'q' => Rank::new(French::QUEEN),
                'K' | 'k' => Rank::new(French::KING),
                'A' | 'a' => Rank::new(French::ACE),
                'B' | 'b' => Rank::new(Modern::BIG),
                'L' | 'l' => Rank::new(Modern::LITTLE),
                _ => Rank::new(FluentName::BLANK),
            },
        }
    }
}
```

REFERENCE:

- Timothy Cain - [Code Refactoring](https://youtu.be/SuMElKtydDQ?si=c4XR3CghGDrdJvi-)
- [Nobody understands playing cards — not even the experts](https://www.youtube.com/watch?v=kY6DmPs4klU)

# Jan 18th 2025

Finished core ref2. Now focusing on macros and decks. 