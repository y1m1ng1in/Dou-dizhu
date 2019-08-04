use super::card::Card;
use super::pair::Pair;

pub struct PairChain<'a> {
    chain: Vec<Pair<'a>>,
}

pub struct PairChainSearch<'a> {
    cards: &'a [Card],
}

impl<'a> IntoIterator for PairChainSearch<'a> {
    type Item = Vec<usize>;
    type IntoIter = PairChainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PairChainIterator {
            cards: self.cards,
            index: 0,
            previous: 0,
            start_val: 0,
        }
    }
}

pub struct PairChainIterator<'a> {
    cards: &'a [Card],
    index: usize,
    previous: u32,
    start_val: u32,
}

impl<'a> Iterator for PairChainIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        let mut result: Vec<usize> = Vec::new();
        let mut start_at = self.index;
        let mut start_at_moved = false;

        if self.index + 1 < self.cards.len() {
            while self.index + 1 < self.cards.len() {
                let current = self.cards[self.index].value;

                if self.start_val + 1 == current && !start_at_moved {
                    start_at = self.index;
                    start_at_moved = true;
                }

                if current == self.previous {
                    self.index += 1;
                } else if current == self.previous + 1
                    && Pair::is_pair(&self.cards[self.index..self.index + 2])
                {
                    result.push(self.index);
                    result.push(self.index + 1);
                    self.previous = current;
                    self.index += 2;
                } else {
                    if result.len() >= 6 {
                        self.index = start_at;
                        self.start_val = current;
                        return Some(result);
                    } else {
                        result = Vec::new();
                        if Pair::is_pair(&self.cards[self.index..self.index + 2]) {
                            result.push(self.index);
                            result.push(self.index + 1);
                            start_at_moved = false;
                            self.start_val = current;
                            self.previous = current;
                            self.index += 2;
                        } else {
                            self.index += 1;
                        }
                    }
                }
            }
            if result.len() >= 6 {
                self.start_val = self.cards[start_at].value;
                self.index = start_at;
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }
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
    pub fn is_pair_chain(cards: &[Card]) -> bool {
        let mut previous: u32 = 0;

        if cards.len() >= 6 && cards.len() % 2 == 0 {
            for i in (0..cards.len()).step_by(2) {
                if Pair::is_pair(&cards[i..i+2]) {
                    if previous == 0 {
                        previous = cards[i].value;
                    } else {
                        if cards[i].value != previous + 1 {
                            return false;
                        } else {
                            previous = cards[i].value;
                        }
                    }
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn search_greater_cards(cards: &[Card], greater_than: &[Card]) -> Option<Vec<usize>> {
        let indices = PairChainSearch { cards: cards }
            .into_iter()
            .find(|x| cards[x[0]].value > greater_than[0].value && x.len() >= greater_than.len());

        match indices {
            Some(i) => Some(i[..greater_than.len()].to_vec()),
            _ => None,
        }
    }

    pub fn search_longest_cards(cards: &[Card]) -> Option<Vec<usize>> {
        let iter = PairChainSearch { cards: cards };
        let result =
            iter.into_iter().fold(
                Vec::new(),
                |max, curr| {
                    if curr.len() > max.len() {
                        curr
                    } else {
                        max
                    }
                },
            );

        if result.len() >= 6 {
            Some(result)
        } else {
            None
        }
    }

    pub fn compare(c1: &Vec<Card>, c2: &Vec<Card>) -> i32 {
        if PairChain::is_pair_chain(c1) && PairChain::is_pair_chain(c2) && c1.len() == c2.len() {
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use super::super::card::Card;
    use super::super::card::Suit;
    use super::PairChain;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }

    #[test]
    fn is_pair_chain_test() {
        let c1 = generate(vec![3, 3, 4, 4, 5, 5]);
        let c2 = generate(vec![]);
        let c3 = generate(vec![3, 3, 5, 5, 6, 6]);
        let c4 = generate(vec![3, 3, 4, 4, 5, 5, 6, 6, 7, 7]);
        let c5 = generate(vec![3, 3, 4, 5, 5, 6, 6]);
        let c6 = generate(vec![3, 3, 3, 4, 4, 5, 5, 6, 6, 6]);

        assert_eq!(PairChain::is_pair_chain(&c1), true);
        assert_eq!(PairChain::is_pair_chain(&c2), false);
        assert_eq!(PairChain::is_pair_chain(&c3), false);
        assert_eq!(PairChain::is_pair_chain(&c4), true);
        assert_eq!(PairChain::is_pair_chain(&c5), false);
        assert_eq!(PairChain::is_pair_chain(&c6), false);
    }

    #[test]
    fn search_greater_test1() {
        let cards = generate(vec![3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8]);
        let handed_in = generate(vec![3, 3, 4, 4, 5, 5]);
        let result = PairChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![2, 3, 4, 5, 6, 7], result);
    }

    #[test]
    fn search_greater_test2() {
        let cards = generate(vec![3, 3, 3, 4, 4, 5, 5, 6, 6, 6, 7, 7]);
        let handed_in = generate(vec![3, 3, 4, 4, 5, 5, 6, 6]);
        let result = PairChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 10, 11], result);
    }

    #[test]
    fn search_greater_test3() {
        let cards = generate(vec![3, 3, 4, 4, 5, 6, 6, 7, 7, 8, 8]);
        let handed_in = generate(vec![5, 5, 6, 6, 7, 7]);
        let result = PairChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![5, 6, 7, 8, 9, 10], result);
    }

    #[test]
    fn search_greater_test4() {
        let cards = generate(vec![3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 9, 9, 10, 10, 11]);
        let handed_in = generate(vec![8, 8, 9, 9, 10, 10]);
        let result = PairChain::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test5() {
        let cards = generate(vec![4, 4, 5, 6, 6]);
        let handed_in = generate(vec![3, 3, 4, 4, 5, 5]);
        let result = PairChain::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test6() {
        let cards = generate(vec![
            3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9,
        ]);
        let handed_in = generate(vec![4, 4, 5, 5, 6, 6]);
        let result = PairChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![4, 5, 8, 9, 11, 12], result);
    }
}
