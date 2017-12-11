use std::cmp::Ordering;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::collections::BTreeMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(cards: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<Hand> = Vec::new();
    cards.iter().fold(&mut hands, |acc, hand| {
        acc.push(Hand::new(hand));
        acc
    });

    if hands.iter().any(|hand| hand.hand_type.is_none()) {
        None
    } else {

        hands.sort_by(|a, b| b.cmp(a));
        let winner = hands[0].hand_type.clone();
        hands.retain(|a| a.hand_type == winner);
        let winner_vec: Vec<String> = hands.iter().map(|hand| hand.hand_str.clone()).collect();
        Some(
            cards
                .iter()
                .filter_map(|s| if winner_vec.contains(&s.to_string()) {
                    Some(*s)
                } else {
                    None
                })
                .collect::<Vec<&'a str>>(),
        )
    }

}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl From<char> for Suit {
    fn from(suit: char) -> Self {
        match suit {
            'D' => Suit::Diamond,
            'C' => Suit::Club,
            'H' => Suit::Heart,
            _ => Suit::Spade,
        }
    }
}

#[derive(Eq, Clone, Debug)]
pub struct Card {
    value: u8,
    suit: Suit,
    ignore_suit: bool,
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.ignore_suit {
            self.value.hash(state);
        } else {
            self.value.hash(state);
            self.suit.hash(state);
        }
    }
}

impl From<String> for Card {
    fn from(card: String) -> Self {
        let mut card_mut = card;
        let suit = Suit::from(card_mut.pop().unwrap());

        let value_str: &str = &card_mut;
        let value;
        match value_str {
            "A" => {
                value = 14u8;
            }
            "K" => {
                value = 13u8;
            }
            "Q" => {
                value = 12u8;
            }
            "J" => {
                value = 11u8;
            }
            _ => {
                value = u8::from_str_radix(value_str, 10).unwrap();
            }
        }

        Card {
            value: value,
            suit: suit,
            ignore_suit: true,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.ignore_suit {
            self.value == other.value
        } else {
            self.value == other.value && self.suit == self.suit
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(ord) => {
                match ord {
                    Ordering::Equal => {
                        if !self.ignore_suit {
                            self.suit.partial_cmp(&other.suit)
                        } else {
                            Some(ord)
                        }
                    }
                    _ => Some(ord),
                }
            }
            None => None,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => Ordering::Less,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum PokerHands {
    HighCard(Card, Card, Card, Card, Card),
    OnePairs(Card, Card, Card, Card),
    TwoPairs(Card, Card, Card),
    Three(Card, Card, Card),
    Straight(Card),
    Flush(Card, Card, Card, Card, Card),
    FullHouse(Card, Card),
    Four(Card, Card),
    StraightFlush(Card),
}

#[derive(Eq, Debug)]
struct Hand {
    hand_str: String,
    hand_type: Option<PokerHands>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type {
            Some(ref hand_type) => {
                match other.hand_type {
                    Some(ref other_hand_type) => hand_type.partial_cmp(other_hand_type),
                    None => None,
                }
            }
            None => None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => Ordering::Less,
        }
    }
}

impl Hand {
    pub fn new(cards: &str) -> Hand {
        let mut card_map: BTreeMap<Card, u8> = BTreeMap::new();
        cards.split_whitespace().for_each(|card| {
            let entry = card_map.entry(Card::from(card.to_string())).or_insert(0);
            *entry += 1;
        });

        let mut poker_hand = None;
        match card_map.len() {
            5 => {
                let keys = card_map.keys().collect::<Vec<&Card>>();
                let is_flush = Hand::is_flush(&keys);
                let is_straight = Hand::is_straight(&keys);

                if is_flush && is_straight {
                    poker_hand = Some(PokerHands::StraightFlush(keys[4].clone()));
                } else if is_flush {
                    if keys[4].value == 14 {
                        poker_hand = Some(PokerHands::Straight(keys[3].clone()));
                    } else {
                        poker_hand = Some(PokerHands::Straight(keys[4].clone()));
                    }
                } else if is_straight {
                    poker_hand = Some(PokerHands::Flush(
                        keys[4].clone(),
                        keys[3].clone(),
                        keys[2].clone(),
                        keys[1].clone(),
                        keys[0].clone(),
                    ));
                } else {
                    poker_hand = Some(PokerHands::HighCard(
                        keys[4].clone(),
                        keys[3].clone(),
                        keys[2].clone(),
                        keys[1].clone(),
                        keys[0].clone(),
                    ));
                }
            }
            4 => {
                let mut keys: Vec<Card> = Vec::new();
                for (key, val) in card_map.iter() {
                    if *val == 2u8 {
                        keys.insert(0, key.clone());
                    } else {
                        keys.push(key.clone());
                    }
                }
                poker_hand = Some(PokerHands::OnePairs(
                    keys[0].clone(),
                    keys[3].clone(),
                    keys[2].clone(),
                    keys[1].clone(),
                ));
            }
            3 => {
                let mut keys: Vec<Card> = Vec::new();
                let max_value = *card_map.values().max().unwrap();
                for (key, val) in card_map.iter() {
                    if *val == 2u8 {
                        keys.insert(0, key.clone());
                    } else {
                        keys.push(key.clone());
                    }
                }

                if max_value == 3u8 {
                    poker_hand = Some(PokerHands::Three(
                        keys[0].clone(),
                        keys[2].clone(),
                        keys[1].clone(),
                    ));
                } else {
                    poker_hand = Some(PokerHands::TwoPairs(
                        keys[0].clone(),
                        keys[1].clone(),
                        keys[2].clone(),
                    ));
                }
            }
            2 => {
                let mut keys: Vec<Card> = Vec::new();
                let max_value = *card_map.values().max().unwrap();
                for (key, val) in card_map.iter() {
                    if *val == 2u8 {
                        keys.insert(0, key.clone());
                    } else {
                        keys.push(key.clone());
                    }
                }
                if max_value == 4u8 {
                    poker_hand = Some(PokerHands::Four(keys[0].clone(), keys[1].clone()));
                } else {
                    poker_hand = Some(PokerHands::FullHouse(keys[1].clone(), keys[0].clone()));
                }
            }
            _ => {}
        }

        Hand {
            hand_str: cards.to_string(),
            hand_type: poker_hand,
        }
    }

    fn is_straight(cards: &Vec<&Card>) -> bool {
        cards.iter().all(|card| card.suit == cards[0].suit)
    }

    fn is_flush(cards: &Vec<&Card>) -> bool {
        if cards.len() != 5 {
            false
        } else if cards[0].value + 4 == cards[4].value {
            true
        } else if cards[0].value == 2 && cards[3].value == 5 && cards[4].value == 14 {
            // A 2 3 4 5
            true
        } else {
            false
        }
    }
}
