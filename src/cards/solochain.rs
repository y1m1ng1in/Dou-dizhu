use super::card::Card;

pub type Solo<'a> = &'a Card;

pub struct SoloChain<'a> {
    chain: Vec<Solo<'a>>,
}

impl<'a> SoloChain<'a> {
    pub fn new(cards: Vec<&Card>) -> SoloChain {
        SoloChain { chain: cards }
    }

    // cards' value has already sorted in ascending
    pub fn is_solo_chain(cards: &Vec<Card>) -> bool {
        let mut previous: u32;

        if cards.len() < 5 {
            return false;
        }
        previous = cards[0].value;
        for card in &cards[1..] {
            if card.value - previous != 1 {
                return false;
            } else {
                previous = card.value;
            }
        }

        true
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &SoloChain) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.chain[0].value;
        let size = greater_than.chain.len();

        while i < cards.len() {
            if cards[i].value <= val {
                i += 1;
            } else {
                break;
            }
        }

        SoloChain::search_from_pos(cards, size, i)
    }

    fn search_from_pos(cards: &Vec<Card>, size: usize, start_from: usize) -> Option<Vec<usize>> {
        let mut i = start_from;
        let mut previous = 0; // the card value can never be 0
        let mut current_size = 0;
        let mut result = Vec::new();

        if start_from >= cards.len() || size + start_from > cards.len() {
            return None;
        }

        while i < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if cards[i].value == previous + 1 {
                result.push(i);
                previous = cards[i].value;
                i += 1;
                current_size += 1;
                if current_size == size {
                    return Some(result);
                }
            } else {
                result = Vec::new();
                result.push(i);
                previous = cards[i].value;
                i += 1;
                current_size = 1;
            }
        }

        if current_size == size {
            Some(result)
        } else {
            None
        }
    }

    pub fn search_longest_cards(cards: &Vec<Card>) -> Option<Vec<usize>> {
        let mut largest = Vec::new();
        let mut current = Vec::new();
        let mut i: usize = 0;
        let mut previous: u32 = 0;

        while i < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if cards[i].value == previous + 1 {
                current.push(i);
                previous = cards[i].value;
            } else {
                if largest.len() < current.len() {
                    largest = Vec::new();
                    largest.append(&mut current);
                } else {
                    current = Vec::new();
                }
                current.push(i);
                previous = cards[i].value;
                i += 1;
            }
        }

        if current.len() > largest.len() && current.len() >= 5 {
            Some(current)
        } else if largest.len() >= 5 {
            Some(largest)
        } else {
            None
        }
    }

    pub fn compare(c1: &Vec<Card>, c2: &Vec<Card>) -> i32 {
        if SoloChain::is_solo_chain(c1) && SoloChain::is_solo_chain(c2) {
            if c1[0].value > c2[0].value {
                1
            } else {
                0
            }
        } else {
            -1
        }
    }
}
