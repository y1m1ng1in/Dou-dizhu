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

    pub fn is_pair(cards: &Vec<&Card>) -> bool {
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
            if Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
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
}
