use super::card::Card;

pub struct Trio<'a> {
    card1: &'a Card,
    card2: &'a Card,
    card3: &'a Card,
}

impl<'a> Trio<'a> {
    pub fn new(c1: &'a Card, c2: &'a Card, c3: &'a Card) -> Trio<'a> {
        Trio {
            card1: c1,
            card2: c2,
            card3: c3,
        }
    }

    pub fn is_trio(cards: &Vec<&Card>) -> bool {
        if cards.len() != 3 {
            return false;
        }
        if cards[0].value == cards[1].value && cards[1].value == cards[2].value {
            true
        } else {
            false
        }
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Trio) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.card1.value;

        while i + 2 < cards.len() {
            if Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]]) {
                if cards[i].value > val {
                    return Some(vec![i, i + 1, i + 2]);
                }
                i += 3;
            } else {
                i += 1;
            }
        }

        None
    }

    pub fn split_from_cards(cards: &mut Vec<Card>) -> Vec<Card> {
        let mut i: usize = 0;
        let mut result = Vec::new();

        while i + 2 < cards.len() {
            if Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]]) {
                for _ in 0..3 {
                    result.push(cards.remove(i));
                }
                break;
            } else {
                i += 1;
            }
        }

        result
    }
}
