use super::card::Card;

pub struct SoloChain {}

pub struct SoloChainSearch<'a> {
    cards: &'a [Card],
}

impl<'a> IntoIterator for SoloChainSearch<'a> {
    type Item = Vec<usize>;
    type IntoIter = SoloChainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SoloChainIterator {
            cards: self.cards,
            index: 0,
            previous: 0,
            start_val: 0,
        }
    }
}

pub struct SoloChainIterator<'a> {
    cards: &'a [Card],
    index: usize,
    previous: u32,
    start_val: u32,
}

impl<'a> Iterator for SoloChainIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        let mut result: Vec<usize> = Vec::new();
        let mut start_at = self.index;
        let mut start_at_moved = false;

        if self.index < self.cards.len() {
            while self.index < self.cards.len() {
                let current = self.cards[self.index].value;

                if self.start_val + 1 == current && !start_at_moved {
                    start_at = self.index;
                    start_at_moved = true;
                }

                if current == self.previous {
                    self.index += 1;
                } else if current == self.previous + 1 {
                    result.push(self.index);
                    self.previous = current;
                    self.index += 1;
                } else if result.len() >= 5 {
                    self.index = start_at;
                    self.start_val = current;
                    return Some(result);
                } else {
                    result = Vec::new();
                    result.push(self.index);
                    start_at_moved = false;
                    self.start_val = current;
                    self.previous = current;
                    self.index += 1;
                }
            }
            if result.len() >= 5 {
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

impl SoloChain {
    // cards' value has already sorted in ascending
    pub fn is_solo_chain(cards: &[Card]) -> bool {
        let mut prev: u32 = 0;

        cards.iter().fold(true, |ret, curr| {
            if prev == 0 {
                prev = curr.value;
                true
            } else if !ret {
                false
            } else if prev + 1 == curr.value {
                prev = curr.value;
                true
            } else {
                false
            }
        }) && cards.len() >= 5
    }

    pub fn search_greater_cards(cards: &[Card], greater_than: &[Card]) -> Option<Vec<usize>> {
        if !greater_than.is_empty() {
            let indices = SoloChainSearch { cards }.into_iter().find(|x| {
                cards[x[0]].value > greater_than[0].value && x.len() >= greater_than.len()
            });

            match indices {
                Some(i) => Some(i[..greater_than.len()].to_vec()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn search_longest_cards(cards: &[Card]) -> Option<Vec<usize>> {
        let iter = SoloChainSearch { cards };
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

        if result.len() < 5 {
            None
        } else {
            Some(result)
        }
    }

    pub fn compare(c1: &[Card], c2: &[Card]) -> i32 {
        if SoloChain::is_solo_chain(c1) && SoloChain::is_solo_chain(c2) && c1.len() == c2.len() {
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
    use super::SoloChain;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }

    #[test]
    fn is_solo_chain_test() {
        let c1 = generate(vec![3, 4, 5, 6, 7]);
        let c2 = generate(vec![]);
        let c3 = generate(vec![3, 4, 5, 6]);
        let c4 = generate(vec![3, 4, 5, 6, 7, 8, 9]);
        let c5 = generate(vec![3, 4, 5, 8, 9]);
        let c6 = generate(vec![3, 4, 5, 6, 7, 9, 10, 11, 12]);
        let c7 = generate(vec![3, 4, 5, 6, 6, 7, 7, 8, 9]);

        assert_eq!(SoloChain::is_solo_chain(&c1), true);
        assert_eq!(SoloChain::is_solo_chain(&c2), false);
        assert_eq!(SoloChain::is_solo_chain(&c3), false);
        assert_eq!(SoloChain::is_solo_chain(&c4), true);
        assert_eq!(SoloChain::is_solo_chain(&c5), false);
        assert_eq!(SoloChain::is_solo_chain(&c6), false);
        assert_eq!(SoloChain::is_solo_chain(&c7), false);
    }

    #[test]
    fn search_greater_test1() {
        let cards = generate(vec![3, 3, 3, 4, 4, 5, 6, 7, 8, 9, 9, 9]);
        let handed_in = generate(vec![4, 5, 6, 7, 8]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![5, 6, 7, 8, 9], result);
    }

    #[test]
    fn search_greater_test2() {
        let cards = generate(vec![3, 3, 3, 4, 4, 6, 7, 8, 9, 9, 9, 10]);
        let handed_in = generate(vec![4, 5, 6, 7, 8]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![5, 6, 7, 8, 11], result);
    }

    #[test]
    fn search_greater_test3() {
        let cards = generate(vec![3, 3, 3, 4, 4, 6, 7, 8, 9, 9, 9, 11]);
        let handed_in = generate(vec![4, 5, 6, 7, 8]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test4() {
        let cards = generate(vec![3, 4, 5, 6, 8, 10, 11, 12, 13, 14]);
        let handed_in = generate(vec![10, 11, 12, 13, 14]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test5() {
        let cards = generate(vec![3, 4, 5, 6, 8, 10, 11, 12, 13, 14]);
        let handed_in = generate(vec![6, 7, 8, 9, 10]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![5, 6, 7, 8, 9], result);
    }

    #[test]
    fn search_greater_test6() {
        let cards = generate(vec![]);
        let handed_in = generate(vec![5, 6, 7, 8, 9, 10]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test7() {
        let cards = generate(vec![
            3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9,
        ]);
        let handed_in = generate(vec![4, 5, 6, 7, 8]);
        let result = SoloChain::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![4, 8, 11, 15, 18], result);
    }

    #[test]
    fn search_longest_test1() {
        let cards = generate(vec![3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14]);
        let result = SoloChain::search_longest_cards(&cards).unwrap();
        assert_eq!(vec![5, 6, 7, 8, 9, 10], result);
    }

    #[test]
    fn search_longest_test2() {
        let cards = generate(vec![3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14]);
        let result = SoloChain::search_longest_cards(&cards).unwrap();
        assert_eq!(vec![4, 5, 6, 7, 8, 9, 10], result);
    }

    #[test]
    fn search_longest_test3() {
        let cards = generate(vec![3, 4, 5, 6, 9, 10, 11, 12, 14, 14]);
        let result = SoloChain::search_longest_cards(&cards);
        assert_eq!(None, result);
    }
}
