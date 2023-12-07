use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: u8,
}

impl Card {
    fn new(kind: char) -> Card {
        if kind.is_numeric() {
            return Card {
                value: kind as u8 - 48,
            };
        }

        return Card {
            value: match kind {
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("{}", format!("unknown card: {}", kind)),
            },
        };
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn hand_type_strength(&self) -> usize {
        let mut frequencies = self
            .cards
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            });

        let j_frequency = frequencies.get(&Card { value: 1 });
        let mut increment_to_higher_frequency = 0;
        if j_frequency.is_some() && frequencies.len() > 1 {
            let j_frequency = j_frequency.unwrap();

            increment_to_higher_frequency = *j_frequency;

            frequencies.remove(&Card { value: 1 });
        }

        let mut frequencies: Vec<_> = frequencies.values().collect();

        frequencies.sort();

        let higher_frequency = *frequencies[frequencies.len() - 1];
        let new_higher_frequency = higher_frequency + increment_to_higher_frequency;

        let last_pos = frequencies.len() - 1;
        frequencies[last_pos] = &new_higher_frequency;

        if frequencies.len() == 1 {
            return 6; // Five of a kind
        } else if frequencies.len() == 2 {
            if frequencies[0] == &1 && frequencies[1] == &4 {
                return 5; // Four of a kind
            } else {
                return 4; // Full house
            }
        } else if frequencies.len() == 3 {
            if *frequencies.last().unwrap() == &3 {
                return 3; // Three of a kind
            } else {
                return 2; // Two pair
            }
        } else if frequencies.len() == 4 {
            return 1; // One pair;
        } else {
            return 0; // High card
        }
    }

    fn compare_by_cards(&self, other: &Hand) -> Ordering {
        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            }
        }

        Ordering::Equal
    }
}

fn main() {
    let mut input = parse_input(include_str!("../input"));

    input.sort_by(|a, b| {
        let a_strength = a.hand_type_strength();
        let b_strength = b.hand_type_strength();
        if a_strength == b_strength {
            return a.compare_by_cards(b);
        } else {
            return a_strength.cmp(&b_strength);
        }
    });

    let result: usize = input.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();

    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let (cards, bid) = line.split_once(" ").unwrap();

        hands.push(Hand {
            cards: cards.chars().map(|c| Card::new(c)).collect(),
            bid: bid.parse().unwrap(),
        })
    }

    hands
}
