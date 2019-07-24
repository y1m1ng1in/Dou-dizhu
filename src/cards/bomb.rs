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
