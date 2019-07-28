use super::card::Card;
use super::card::Suit;

pub struct Bomb<'a> {
    card1: &'a Card,
    card2: &'a Card,
    card3: &'a Card,
    card4: &'a Card,
}

pub struct Rocket<'a> {
    joker1: &'a Card,
    joker2: &'a Card,
}

impl<'a> Bomb<'a> {
    pub fn new(cards: &Vec<&'a Card>) -> Bomb<'a> {
        Bomb {
            card1: cards[0],
            card2: cards[1],
            card3: cards[2],
            card4: cards[3],
        }
    }

    pub fn is_bomb(cards: &Vec<&'a Card>) -> bool {
        let val;

        if cards.len() != 4 {
            return false;
        }

        val = cards[0].value;
        for card in &cards[1..] {
            if card.value != val {
                return false;
            }
        }

        true
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Bomb) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.card1.value;
        let mut result = Vec::new();

        while i + 3 < cards.len() {
            if Bomb::is_bomb(&vec![
                &cards[i],
                &cards[i + 1],
                &cards[i + 2],
                &cards[i + 3],
            ]) {
                if cards[i].value > val {
                    result.push(i);
                    result.push(i + 1);
                    result.push(i + 2);
                    result.push(i + 3);
                    break;
                } else {
                    i += 4;
                }
            } else {
                i += 1;
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn split_from_cards(cards: &mut Vec<Card>) -> (Vec<Card>, Vec<Card>) {
        let mut i: usize = 0;
        let mut result = Vec::new();

        while i + 3 < cards.len() {
            if Bomb::is_bomb(&vec![
                &cards[i],
                &cards[i + 1],
                &cards[i + 2],
                &cards[i + 3],
            ]) {
                for _ in 0..4 {
                    result.push(cards.remove(i));
                }
                break;
            } else {
                i += 1;
            }
        }

        (result, cards.to_vec())
    }
}

impl<'a> Rocket<'a> {
    pub fn is_rocket(cards: &Vec<&Card>) -> bool {
        if cards.len() != 2 {
            return false;
        }

        let type1 = match cards[0].suit {
            Suit::Joker => true,
            _ => false,
        };

        let type2 = match cards[1].suit {
            Suit::Joker => true,
            _ => false,
        };

        if type1 && type2 {
            true
        } else {
            false
        }
    }
}
