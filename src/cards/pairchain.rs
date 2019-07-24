use super::card::Card;
use super::pair::Pair;

pub struct PairChain<'a> {
    chain: Vec<Pair<'a>>,
}

impl<'a> PairChain<'a> {
    // assume cards pass the pattern matching
    pub fn new(cards: Vec<&Card>) -> PairChain {
        let mut result = Vec::new();

        for i in (0..cards.len()).step_by(2) {
            result.push(Pair::new(cards[i], cards[i + 1]));
        }

        PairChain { chain: result }
    }

    // cards' value has already sorted in ascending
    pub fn is_pair_chain(cards: &Vec<&Card>) -> bool {
        let mut previous: u32;

        if cards.len() < 6 || cards.len() % 2 != 0 {
            return false;
        }

        if Pair::is_pair(&vec![cards[0], cards[1]]) {
            previous = cards[0].value;
        } else {
            return false;
        }

        for i in (2..cards.len()).step_by(2) {
            if Pair::is_pair(&vec![cards[i], cards[i + 1]]) {
                if cards[i].value - previous != 1 {
                    return false;
                } else {
                    previous = cards[i].value;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &PairChain) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.chain[0].value();
        let size = greater_than.chain.len();

        while i < cards.len() {
            if cards[i].value <= val {
                i += 1;
            } else {
                break;
            }
        }

        PairChain::search_from_pos(cards, size, i)
    }

    fn search_from_pos(cards: &Vec<Card>, size: usize, start_from: usize) -> Option<Vec<usize>> {
        let mut i = start_from;
        let mut previous: u32 = 0;
        let mut current_size = 0; // the card value can never be 0
        let mut result = Vec::new();

        if start_from >= cards.len() || size * 2 + start_from > cards.len() {
            return None;
        }

        while i + 1 < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if Pair::is_pair(&vec![&cards[i], &cards[i + 1]])
                && cards[i].value == previous + 1
            {
                previous = cards[i].value;
                current_size += 1;
                result.push(i);
                result.push(i + 1);
                i += 2;
                if current_size == size {
                    return Some(result);
                }
            } else {
                result = Vec::new();
                while i + 1 < cards.len() {
                    if !Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
                        i += 1;
                    } else {
                        previous = cards[i].value;
                        current_size = 1; // change += to =
                        result.push(i);
                        result.push(i + 1);
                        i += 2;
                        break;
                    }
                }
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

        while i + 1 < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if Pair::is_pair(&vec![&cards[i], &cards[i + 1]])
                && cards[i].value == previous + 1
            {
                previous = cards[i].value;
                current.push(i);
                current.push(i + 1);
                i += 2;
            } else {
                if current.len() > largest.len() {
                    largest = Vec::new();
                    largest.append(&mut current);
                } else {
                    current = Vec::new();
                }
                while i + 1 < cards.len() {
                    if !Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
                        i += 1;
                    } else {
                        previous = cards[i].value;
                        current.push(i);
                        current.push(i + 1);
                        i += 2;
                        break;
                    }
                }
            }
        }

        if current.len() > largest.len() && current.len() >= 6 {
            Some(current)
        } else if largest.len() >= 6 {
            Some(largest)
        } else {
            None
        }
    }
}
