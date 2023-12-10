use std::{collections::HashMap, env, fs, ops::AddAssign, str::FromStr};
use Card::*;
use HandType::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input_string = fs::read_to_string(file_path).unwrap();

    let mut hands: Vec<Hand> = input_string
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Hand>().unwrap())
        .collect();

    hands.sort_by_key(|hand| hand.cards[4]);
    hands.sort_by_key(|hand| hand.cards[3]);
    hands.sort_by_key(|hand| hand.cards[2]);
    hands.sort_by_key(|hand| hand.cards[1]);
    hands.sort_by_key(|hand| hand.cards[0]);
    hands.sort_by_key(|hand| hand.hand_type);

    dbg!(&hands);

    let part1_solution: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * usize::try_from(hand.bid).unwrap())
        .sum();

    dbg!(part1_solution);
}

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Card {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(ONE),
            "2" => Ok(TWO),
            "3" => Ok(THREE),
            "4" => Ok(FOUR),
            "5" => Ok(FIVE),
            "6" => Ok(SIX),
            "7" => Ok(SEVEN),
            "8" => Ok(EIGHT),
            "9" => Ok(NINE),
            "T" => Ok(TEN),
            "J" => Ok(JACK),
            "Q" => Ok(QUEEN),
            "K" => Ok(KING),
            "A" => Ok(ACE),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Card> = s[0..5]
            .chars()
            .map(|c| char::to_string(&c).parse().unwrap())
            .collect();
        let bid = s[6..].parse::<u32>().unwrap();
        Ok(Hand {
            cards: cards.clone(),
            bid,
            hand_type: Hand::compute_hand_type(cards),
        })
    }
}

impl Hand {
    fn compute_hand_type(cards: Vec<Card>) -> HandType {
        let mut cards_count: HashMap<Card, u32> = HashMap::new();
        for card in cards {
            cards_count
                .entry(card)
                .and_modify(|count| count.add_assign(1))
                .or_insert(1);
        }

        let mut counts: Vec<&u32> = cards_count.values().collect();
        counts.sort();
        counts.reverse();
        if *counts[0] == 5 {
            FiveOfAKind
        } else if *counts[0] == 4 {
            FourOfAKind
        } else if *counts[0] == 3 {
            if *counts[1] == 2 {
                FullHouse
            } else {
                ThreeOfAKind
            }
        } else if *counts[0] == 2 {
            if *counts[1] == 2 {
                TwoPairs
            } else {
                Pair
            }
        } else {
            HighCard
        }
    }
}
