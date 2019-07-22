pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
    Joker,
}

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