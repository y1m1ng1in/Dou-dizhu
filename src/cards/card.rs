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

    pub fn display(&self) {
        println! {"{}", self.value};
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Card) -> Option<usize> {
        let val = greater_than.value;

        for i in 0..cards.len() {
            if cards[i].value > val {
                return Some(i);
            }
        }

        None
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

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Card {{ value: {}, suit: {}, selected: {} }}\n",
            self.value, self.suit, self.selected
        )
    }
}
