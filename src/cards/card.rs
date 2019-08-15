use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
    Joker,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub value: u32,
    pub suit: Suit,
    pub selected: bool,
}

impl Card {
    pub fn new(val: u32, s: Suit, sel: bool) -> Card {
        Card {
            value: val,
            suit: s,
            selected: sel,
        }
    }

    pub fn display(self) {
        println! {"{}", self.value};
    }

    pub fn search_greater_cards(cards: &[Card], greater_than: &[Card]) -> Option<Vec<usize>> {
        if greater_than.len() == 1 {
            for (i, item) in cards.iter().enumerate() {
                if item.value > greater_than[0].value {
                    return Some(vec![i]);
                }
            }
            None
        } else {
            None
        }
    }

    pub fn compare(c1: Card, c2: Card) -> i32 {
        if c1.value > c2.value {
            1
        } else {
            0
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Suit::Heart => f.write_str("Heart"),
            Suit::Spade => f.write_str("Spade"),
            Suit::Club => f.write_str("Club"),
            Suit::Diamond => f.write_str("Diamond"),
            Suit::Joker => f.write_str("Joker"),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit && self.selected == other.selected
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Card {}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Card {{ value: {}, suit: {}, selected: {} }}\n",
            self.value, self.suit, self.selected
        )
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Card {{ value: {}, suit: {}, selected: {} }}\n",
            self.value, self.suit, self.selected
        )
    }
}
