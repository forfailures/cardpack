use crate::localization::FluentName;
use crate::prelude::{Ranked, Suited};
use crate::types::utils::Bit;
use colored::Color;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub const BLANK: char = '_';

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Rank

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank<RankType>
where
    RankType: Ranked,
{
    pub weight: u32,
    pub index: char,
    pub phantom_data: PhantomData<RankType>,
}

impl<RankType> Rank<RankType>
where
    RankType: Ranked,
{
    pub const BLANK: Rank<RankType> = Rank {
        weight: 0,
        index: BLANK,
        phantom_data: PhantomData,
    };

    const PRIMES: [u32; 20] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];

    /// Returns the `FluentName` of the rank. This is used to get localized values stored
    /// in the `localization` module.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(French::DEUCE.get_name(), FluentName::new("two"));
    /// ```
    #[must_use]
    pub fn get_name(&self) -> FluentName {
        RankType::get_rank_fluent_name(self.index)
    }

    /// Returns the xth prime number where x is the weight of the rank.
    ///
    ///The goal of this function is to replace the earlier version of the struct that stored the
    /// prime number as a field. In refactoring this code, I am trying to take a minimalist approach
    /// to the types. The original structs come from when I was very new to Rust.
    ///
    ///```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(French::KING.get_prime(), 37);
    /// ```
    ///
    /// It only goes up to 20:
    ///
    ///```
    /// use std::marker::PhantomData;
    /// use cardpack::prelude::*;
    ///
    /// let heavy_card = French::TREY.update_weight(21);
    ///
    /// assert_eq!(heavy_card.get_prime(), 0);
    /// ```
    /// TODO: Hack
    #[must_use]
    pub fn get_prime(&self) -> u32 {
        if self.weight as usize >= Rank::<RankType>::PRIMES.len() {
            0
        } else {
            Rank::<RankType>::PRIMES[(self.weight) as usize]
        }
    }

    /// Returns true if the `Rank` is blank.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let rank = Rank::<French>::from('X');
    ///
    /// assert!(rank.is_blank());
    /// ```
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.index == BLANK
    }

    /// # ASIDE
    ///
    /// Wasted 10 minutes on a copilot suggestion that had into instead of from
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let expected = vec![
    ///     French::ACE,
    ///     French::KING,
    ///     French::QUEEN,
    ///     French::JACK,
    ///     French::TEN,
    ///     French::NINE,
    ///     French::EIGHT,
    ///     French::SEVEN,
    ///     French::SIX,
    ///     French::FIVE,
    ///     French::FOUR,
    ///     French::TREY,
    ///     French::DEUCE,
    /// ];
    ///
    /// assert_eq!(Rank::<French>::ranks(), expected);
    /// ```
    #[must_use]
    pub fn ranks() -> Vec<Self> {
        RankType::rank_indexes()
            .iter()
            .map(|index| Rank::<RankType>::from(*index))
            .collect()
    }

    /// Returns an index of the `Ranks` joined by the joiner `&str`;
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let ranks = vec![French::ACE, French::DEUCE, French::TREY, French::FOUR, French::FIVE];
    ///
    /// assert_eq!("A-2-3-4-5", Rank::<French>::ranks_index(&ranks, "-"));
    /// ```
    #[must_use]
    pub fn ranks_index(ranks: &[Rank<RankType>], joiner: &str) -> String {
        ranks
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(joiner)
    }

    /// Returns an index of all the `Ranks` in the implementing `Deck` joined by the joiner `&str`;
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(Rank::<French>::ranks_index_all("+"), "A+K+Q+J+T+9+8+7+6+5+4+3+2");
    /// ```
    #[must_use]
    pub fn ranks_index_all(joiner: &str) -> String {
        Rank::<RankType>::ranks_index(&Rank::<RankType>::ranks(), joiner)
    }

    /// Returns a new instance of the `Rank` with the weight updated. This is to enable the ability
    /// to alter how the `Rank` is sorted.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut ranks = Rank::<French>::ranks();
    /// let mut mirror_ranks: Vec<Rank<French>> = Vec::new();
    ///
    /// for (weight, rank) in ranks.iter().enumerate() {
    ///     let updated_rank = rank.update_weight(weight as u32);
    ///     mirror_ranks.push(updated_rank);
    ///     println!("{:?}", updated_rank);
    /// }
    ///
    /// ranks.sort();
    /// mirror_ranks.sort();
    ///
    /// assert_eq!("2 3 4 5 6 7 8 9 T J Q K A", Rank::<French>::ranks_index(&ranks, " "));
    /// assert_eq!("A K Q J T 9 8 7 6 5 4 3 2", Rank::<French>::ranks_index(&mirror_ranks, " "));
    /// ```
    ///
    /// **NOTE:** These base structs sort from the natural order of the weight, just like an integer.
    /// The plan is to update sorting at the `Card` level.
    #[must_use]
    pub fn update_weight(&self, weight: u32) -> Rank<RankType> {
        Rank {
            weight,
            index: self.index,
            phantom_data: PhantomData,
        }
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // CKC Calculations
    /// Used to generate the `Rank` portion of the `Card`'s binary number,
    /// aka [Cactus Kev](https://suffe.cool/poker/evaluator.html) number.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(0b00000000_00000001_00000000_00000010, French::DEUCE.ckc_number());
    /// assert_eq!(0b00000000_00000010_00000001_00000011, French::TREY.ckc_number());
    /// assert_eq!(0b00000000_00000100_00000010_00000101, French::FOUR.ckc_number());
    /// assert_eq!(0b00000000_00001000_00000011_00000111, French::FIVE.ckc_number());
    /// assert_eq!(0b00000000_00010000_00000100_00001011, French::SIX.ckc_number());
    /// assert_eq!(0b00000000_00100000_00000101_00001101, French::SEVEN.ckc_number());
    /// assert_eq!(0b00000000_01000000_00000110_00010001, French::EIGHT.ckc_number());
    /// assert_eq!(0b00000000_10000000_00000111_00010011, French::NINE.ckc_number());
    /// assert_eq!(0b00000001_00000000_00001000_00010111, French::TEN.ckc_number());
    /// assert_eq!(0b00000010_00000000_00001001_00011101, French::JACK.ckc_number());
    /// assert_eq!(0b00000100_00000000_00001010_00011111, French::QUEEN.ckc_number());
    /// assert_eq!(0b00001000_00000000_00001011_00100101, French::KING.ckc_number());
    /// assert_eq!(0b00010000_00000000_00001100_00101001, French::ACE.ckc_number());
    /// ```
    #[must_use]
    pub fn ckc_number(&self) -> u32 {
        self.get_bits() | self.get_shift8() | self.get_prime()
    }

    #[must_use]
    fn get_bits(&self) -> u32 {
        1 << (Bit::RANK_FLAG_SHIFT + self.weight)
    }

    #[must_use]
    pub fn get_shift8(&self) -> u32 {
        self.weight << 8
    }
}

/// ```
/// use std::marker::PhantomData;
/// use cardpack::prelude::*;
///
/// let expected = Rank::<French> {
///     weight: 0,
///     index: BLANK,
///     phantom_data: PhantomData,
/// };
///
/// assert_eq!(Rank::<French>::default(), expected);
/// ```
impl<RankType: Ranked> Default for Rank<RankType> {
    fn default() -> Self {
        Rank::<RankType>::BLANK
    }
}

impl<RankType> Display for Rank<RankType>
where
    RankType: Ranked,
{
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(French::DEUCE_INDEX.to_string(), French::DEUCE.to_string());
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

/// # REFACTOR WIN
///
/// This win ended up being a big W when I tried to wire it into the Card struct.
///
/// ```
/// use cardpack::prelude::*;
///
/// assert_eq!(Rank::<French>::from('A'), French::ACE);
/// assert_eq!(Rank::<French>::from('a'), French::ACE);
/// assert_eq!(Rank::<French>::from('2'), French::DEUCE);
/// ```
///
/// # DOUBLE REFACTOR WIN
///
/// We've moved this down from the Deck generic instance to here, making it all-encompasing.
/// The original version was:
///
/// ```txt
/// impl From<char> for Rank<French> {
///     fn from(c: char) -> Self {
///         match c {
///             French::ACE_INDEX | 'a' => French::ACE,
///             French::KING_INDEX | 'k' => French::KING,
///             French::QUEEN_INDEX | 'q' => French::QUEEN,
///             French::JACK_INDEX | 'j' => French::JACK,
///             French::TEN_INDEX | 't' | '0' => French::TEN,
///             French::NINE_INDEX => French::NINE,
///             French::EIGHT_INDEX => French::EIGHT,
///             French::SEVEN_INDEX => French::SEVEN,
///             French::SIX_INDEX => French::SIX,
///             French::FIVE_INDEX => French::FIVE,
///             French::FOUR_INDEX => French::FOUR,
///             French::TREY_INDEX => French::TREY,
///             French::DEUCE_INDEX => French::DEUCE,
///             _ => Rank::<French>::default(),
///         }
///     }
/// }
/// ```
///
/// # TRIPLE REFACTOR WIN
///
/// We can now remove the `Rank::<RankType>::new() function and just use this instead.`
impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(index: char) -> Self {
        // Wash the index to make sure it's the correct char.
        let index = RankType::get_rank_index(index);
        Rank {
            weight: RankType::get_rank_weight(index),
            index,
            phantom_data: PhantomData,
        }
    }
}

impl<RankType: Ranked> Ranked for Rank<RankType> {
    fn get_rank_fluent_name(c: char) -> FluentName {
        RankType::get_rank_fluent_name(c)
    }

    fn get_rank_index(c: char) -> char {
        RankType::get_rank_index(c)
    }

    fn get_rank_weight(c: char) -> u32 {
        RankType::get_rank_weight(c)
    }

    fn rank_indexes() -> Vec<char> {
        RankType::rank_indexes()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod ranks {
    use super::*;
    use crate::prelude::French;
    use rstest::rstest;

    #[test]
    fn rank__get_prime() {
        assert_eq!(French::DEUCE.get_prime(), 2);
        assert_eq!(French::TREY.get_prime(), 3);
        assert_eq!(French::FOUR.get_prime(), 5);
        assert_eq!(French::FIVE.get_prime(), 7);
        assert_eq!(French::SIX.get_prime(), 11);
        assert_eq!(French::SEVEN.get_prime(), 13);
        assert_eq!(French::EIGHT.get_prime(), 17);
        assert_eq!(French::NINE.get_prime(), 19);
        assert_eq!(French::TEN.get_prime(), 23);
        assert_eq!(French::JACK.get_prime(), 29);
        assert_eq!(French::QUEEN.get_prime(), 31);
        assert_eq!(French::KING.get_prime(), 37);
        assert_eq!(French::ACE.get_prime(), 41);

        assert_eq!(French::TREY.update_weight(21).get_prime(), 0);
    }

    #[test]
    fn rank__is_blank() {
        let rank = Rank::<French>::default();

        assert!(rank.is_blank());
    }

    #[test]
    fn rank__ranks_index() {
        let ranks = vec![French::KING, French::QUEEN, French::JACK, French::TEN];
        assert_eq!(Rank::<French>::ranks_index(&ranks, "_"), "K_Q_J_T");
    }

    #[test]
    fn rank__ranks_index_all() {
        assert_eq!(
            Rank::<French>::ranks_index_all(" "),
            "A K Q J T 9 8 7 6 5 4 3 2"
        );
    }

    /// TODO: Add some rows for other decks when they're added.
    #[rstest]
    #[case('A', French::ACE)]
    #[case('a', French::ACE)]
    #[case('K', French::KING)]
    #[case('k', French::KING)]
    #[case('Q', French::QUEEN)]
    #[case('q', French::QUEEN)]
    #[case('J', French::JACK)]
    #[case('j', French::JACK)]
    #[case('T', French::TEN)]
    #[case('t', French::TEN)]
    #[case('0', French::TEN)]
    #[case('9', French::NINE)]
    #[case('8', French::EIGHT)]
    #[case('7', French::SEVEN)]
    #[case('6', French::SIX)]
    #[case('5', French::FIVE)]
    #[case('4', French::FOUR)]
    #[case('3', French::TREY)]
    #[case('2', French::DEUCE)]
    #[case('1', French::BLANK_RANK)]
    #[case('F', French::BLANK_RANK)]
    fn rank__from__char(#[case] input: char, #[case] expected: Rank<French>) {
        assert_eq!(expected, Rank::<French>::from(input));
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Suit

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Suit<SuitType>
where
    SuitType: Suited,
{
    pub weight: u32,
    pub index: char,
    pub phantom_data: PhantomData<SuitType>,
}

impl<SuitType> Suit<SuitType>
where
    SuitType: Suited,
{
    pub const BLANK: Suit<SuitType> = Suit {
        weight: 0,
        index: BLANK,
        phantom_data: PhantomData,
    };

    /// Returns the Suit portion of the `CKC Number`.
    /// Used to generate the `Card`'s binary signature, aka [Cactus Kev](https://suffe.cool/poker/evaluator.html)
    /// numbers.
    ///
    /// Revised version that inverts the weight for sorting, making Spades be the highest. Has no
    /// effect on the generated card ranks, but does make sorting easier.
    ///
    /// TODO: need a way to add the jokers suit. Right now this assumes standard 52
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(0b00000000_00000000_10000000_00000000, French::SPADES.ckc_number());
    /// assert_eq!(0b00000000_00000000_01000000_00000000, French::HEARTS.ckc_number());
    /// assert_eq!(0b00000000_00000000_00100000_00000000, French::DIAMONDS.ckc_number());
    /// assert_eq!(0b00000000_00000000_00010000_00000000, French::CLUBS.ckc_number());
    ///
    /// ```
    #[must_use]
    pub fn ckc_number(&self) -> u32 {
        match self.weight {
            0 => 0,
            _ => 1 << (Bit::SUIT_FLAG_SHIFT + self.weight),
        }
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert!(Suit::<French>::default().is_blank());
    /// ```
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.index == BLANK
    }

    /// Returns a vector of all the suits in the implementing `Deck`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let expected = vec![French::SPADES, French::HEARTS, French::DIAMONDS, French::CLUBS];
    ///
    /// assert_eq!(Suit::<French>::suits(), expected);
    /// ```
    #[must_use]
    pub fn suits() -> Vec<Self> {
        SuitType::suit_indexes()
            .iter()
            .map(|index| Suit::<SuitType>::from(*index))
            .collect()
    }

    /// Returns the suit;s symbol as set in the `FluentName` struct for the suit.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(French::SPADES.symbol(), '♠');
    /// ```
    ///
    /// TODO: Possible REFACTOR - Add symbol char to struct to avoid need for localization
    /// call. We will save this for after REF2 is complete.
    #[must_use]
    pub fn symbol(&self) -> char {
        SuitType::get_suit_symbol(self.index)
    }
}

impl<SuitType: Suited> Default for Suit<SuitType> {
    fn default() -> Self {
        Suit::<SuitType>::BLANK
    }
}

impl<SuitType> Display for Suit<SuitType>
where
    SuitType: Suited,
{
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(French::DIAMONDS.to_string(), "♦");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Suit::<SuitType>::get_suit_symbol(self.index))
    }
}

/// Instantiates a new Suit struct from the passed in index.
///
/// ```
/// use cardpack::prelude::*;
///
/// assert_eq!(Suit::<French>::from('♠'), French::SPADES);
/// assert_eq!(Suit::<French>::from('S'), French::SPADES);
/// assert_eq!(Suit::<French>::from('s'), French::SPADES);
/// assert_eq!(Suit::<French>::from('♤'), French::SPADES);
/// ```
impl<SuiteType: Suited> From<char> for Suit<SuiteType> {
    fn from(index: char) -> Self {
        let index = Suit::<SuiteType>::get_suit_index(index);
        Suit {
            weight: Suit::<SuiteType>::get_suit_weight(index),
            index,
            phantom_data: PhantomData,
        }
    }
}

impl<SuiteType: Suited> Suited for Suit<SuiteType> {
    fn colors() -> HashMap<char, Color> {
        SuiteType::colors()
    }

    fn get_suit_fluent_name(c: char) -> FluentName {
        SuiteType::get_suit_fluent_name(c)
    }

    fn get_suit_index(c: char) -> char {
        SuiteType::get_suit_index(c)
    }

    fn get_suit_symbol(c: char) -> char {
        SuiteType::get_suit_symbol(c)
    }

    fn get_suit_weight(c: char) -> u32 {
        SuiteType::get_suit_weight(c)
    }

    fn suit_indexes() -> Vec<char> {
        SuiteType::suit_indexes()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suits {
    use super::*;
    use crate::prelude::French;
    use rstest::rstest;

    /// TODO: Add some rows for other decks when they're added.
    #[rstest]
    #[case('♠', French::SPADES)]
    #[case('♤', French::SPADES)]
    #[case('S', French::SPADES)]
    #[case('s', French::SPADES)]
    #[case('♥', French::HEARTS)]
    #[case('♡', French::HEARTS)]
    #[case('H', French::HEARTS)]
    #[case('h', French::HEARTS)]
    #[case('♦', French::DIAMONDS)]
    #[case('♢', French::DIAMONDS)]
    #[case('D', French::DIAMONDS)]
    #[case('d', French::DIAMONDS)]
    #[case('♣', French::CLUBS)]
    #[case('♧', French::CLUBS)]
    #[case('C', French::CLUBS)]
    #[case('c', French::CLUBS)]
    #[case('🃟', French::BLANK_SUIT)]
    #[case('T', French::BLANK_SUIT)]
    #[case('t', French::BLANK_SUIT)]
    #[case(' ', French::BLANK_SUIT)]
    #[case('F', French::BLANK_SUIT)]
    fn suit__from__char(#[case] input: char, #[case] expected: Suit<French>) {
        assert_eq!(expected, Suit::<French>::from(input));
    }

    #[test]
    fn suit__symbol() {
        assert_eq!(Suit::<French>::from('♠').symbol(), '♠');
        assert_eq!(Suit::<French>::from('S').symbol(), '♠');
        assert_eq!(Suit::<French>::from('s').symbol(), '♠');
        assert_eq!(Suit::<French>::from('♤').symbol(), '♠');
        assert_eq!(Suit::<French>::from(' ').symbol(), '_');
    }

    #[test]
    fn suit__suits() {
        let expected = vec![
            French::SPADES,
            French::HEARTS,
            French::DIAMONDS,
            French::CLUBS,
        ];

        assert_eq!(Suit::<French>::suits(), expected);
    }
}
