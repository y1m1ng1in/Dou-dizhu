use super::card::Card;
use super::pair::Pair;
use super::trio::Trio;
use super::card::Suit;

pub struct Airplane<'a> {
    chain: Vec<&'a Card>,
    kicker: Vec<&'a Card>,
    size: usize,
}

impl<'a> Airplane<'a> {
    pub fn new(cards: &Vec<&'a Card>) -> Airplane<'a> {
        let airplane_size = Airplane::find_size(cards);
        let start = airplane_size.0;
        let num = airplane_size.1;
        let mut trios: Vec<&Card> = Vec::new();
        let mut kickers: Vec<&Card> = Vec::new();

        for i in start..start + 3 * num {
            trios.push(cards[i]);
        }

        for i in 0..start {
            kickers.push(cards[i]);
        }
        for i in start + 3 * num..cards.len() {
            kickers.push(cards[i]);
        }

        Airplane {
            chain: trios,
            kicker: kickers,
            size: num,
        }
    }

    // cards' value has already sorted in ascending
    pub fn is_airplane(cards: &Vec<&Card>) -> bool {
        let airplane_size = Airplane::find_size(cards);

        if airplane_size.1 == 0 {
            return false;
        } else {
            return true;
        }
    }

    fn find_size(cards: &Vec<&'a Card>) -> (usize, usize) {
        let mut i: usize = 0;
        let mut previous = 0;
        let mut first_trio = true;
        let mut trio_start = 0;
        let mut max_size = 0;
        let mut max_size_trio_start = 0;
        let mut trio_nums = 0;

        if cards.len() < 6 {
            return (0, 0);
        }

        while i + 2 < cards.len() {
            if Trio::is_trio(&vec![cards[i], cards[i + 1], cards[i + 2]]) {
                if first_trio {
                    trio_start = i;
                    previous = cards[i].value;
                    first_trio = false;
                    trio_nums += 1;
                } else {
                    if cards[i].value - previous == 1 {
                        trio_nums += 1;
                        previous = cards[i].value;
                    } else {
                        if trio_nums > max_size {
                            max_size = trio_nums;
                            max_size_trio_start = trio_start;
                        }
                        trio_start = i;
                        previous = cards[i].value;
                        trio_nums = 1;
                    }
                }
                i += 3;
            } else {
                i += 1;
                if trio_nums > max_size {
                    max_size = trio_nums;
                    max_size_trio_start = trio_start;
                    trio_nums = 0;
                    first_trio = true;
                }
            }
        }

        if trio_nums > max_size {
            max_size = trio_nums;
            max_size_trio_start = trio_start;
        }

        if max_size == 0 {
            (0, 0)
        } else {
            (max_size_trio_start, max_size)
        }
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Airplane) -> Option<Vec<usize>> {
        let pair_kicker: bool;
        let val = greater_than.chain[0].value;
        let mut i: usize = 0;

        if greater_than.kicker.len() == greater_than.size * 2 {
            pair_kicker = true;
        } else {
            pair_kicker = false;
        }

        while i < cards.len() {
            if cards[i].value <= val {
                i += 1;
            } else {
                break;
            }
        }

        Airplane::search_from_pos(cards, greater_than.size, i, pair_kicker)
    }

    fn search_from_pos(
        cards: &Vec<Card>,
        size: usize,
        start_from: usize,
        pair_kicker: bool,
    ) -> Option<Vec<usize>> {
        let mut i: usize = start_from;
        let mut previous = 0;
        let mut current_size = 0;
        let mut result = Vec::new(); 

        if start_from >= cards.len() || start_from + 3 * size >= cards.len() {
            return None;
        }

        while i + 2 < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]])
                && cards[i].value == previous + 1 
            {
                previous = cards[i].value;
                current_size += 1;

                result.push(i);
                result.push(i + 1);
                result.push(i + 2);

                if current_size == size {
                    break;
                } else {
                    i += 3;
                }
            } else {
                result = Vec::new();
                while i + 2 < cards.len() {
                    if !Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]]) {
                        i += 1;
                    } else {
                        previous = cards[i].value;
                        current_size = 1;

                        result.push(i);
                        result.push(i + 1);
                        result.push(i + 2);

                        i += 3;
                        break;
                    }
                }
            }
        }

        if current_size != size {
            return None;
        } 
            
        Airplane::search_kickers(cards, &mut result, size, pair_kicker)
    }

    fn search_kickers(cards: &Vec<Card>, trio_indices: &mut Vec<usize>, size: usize, pair_kicker: bool) -> Option<Vec<usize>> {
        let mut result = Vec::new();
        let mut i: usize = 0;
        let mut current_size = 0;

        if !pair_kicker {
            while i < trio_indices.len() {
                if !trio_indices.contains(&i) {
                    result.push(i);
                    current_size += 1;
                    i += 1;
                } else {
                    i += 1;
                }
                if current_size == size {
                    break;
                }
            }
        } else {
            while i + 1 < trio_indices.len() {
                if Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
                    if !trio_indices.contains(&i) && !trio_indices.contains(&(i + 1)) {
                        result.push(i);
                        result.push(i + 1);
                        current_size += 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                    if current_size == size {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        }

        if current_size == size {
            for j in 0..result.len() {
                trio_indices.push(result[j]);
            }
            Some(trio_indices.to_vec())
        } else {
            None
        }
    }
}
