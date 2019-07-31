use super::card::Card;

pub struct Pair<'a> {
    card1: &'a Card,
    card2: &'a Card,
}

impl<'a> Pair<'a> {
    pub fn new(c1: &'a Card, c2: &'a Card) -> Pair<'a> {
        Pair {
            card1: c1,
            card2: c2,
        }
    }

    pub fn is_pair(cards: &Vec<Card>) -> bool {
        if cards.len() != 2 {
            return false;
        }
        if cards[0].value != cards[1].value {
            false
        } else {
            true
        }
    }

    pub fn value(&self) -> u32 {
        return self.card1.value;
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Pair) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.card1.value;

        while i + 1 < cards.len() {
            if Pair::is_pair(&vec![cards[i], cards[i + 1]]) {
                if cards[i].value > val {
                    return Some(vec![i, i + 1]);
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        None
    }

    pub fn split_from_cards(cards: &mut Vec<Card>) -> Vec<Card> {
        let mut i: usize = 0;
        let mut result = Vec::new();

        while i + 1 < cards.len() {
            if Pair::is_pair(&vec![cards[i], cards[i + 1]]) {
                for _ in 0..2 {
                    result.push(cards.remove(i));
                }
                break;
            } else {
                i += 1;
            }
        }

        result
    }

    pub fn compare(c1: &Vec<Card>, c2: &Vec<Card>) -> i32 {
        if Pair::is_pair(c1) && Pair::is_pair(c2) {
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
