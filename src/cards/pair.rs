use super::card::Card;

pub struct Pair<'a> {
    card1: &'a Card,
    card2: &'a Card,
}

pub struct PairSearch<'a>(&'a Vec<Card>);

impl<'a> IntoIterator for PairSearch<'a> {
    type Item = Vec<usize>;
    type IntoIter = PairIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PairIterator {
            cards: self,
            index: 0,
        }
    }
}

pub struct PairIterator<'a> {
    cards: PairSearch<'a>,
    index: usize,
}

impl<'a> Iterator for PairIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        let result: Vec<usize>;

        if self.index + 1 < self.cards.0.len() {
            while self.index + 1 < self.cards.0.len() {
                if !Pair::is_pair(&vec![
                    self.cards.0[self.index],
                    self.cards.0[self.index + 1],
                ]) {
                    self.index += 1;
                } else {
                    result = vec![self.index, self.index + 1];
                    self.index += 2;
                    return Some(result);
                }
            }
            None
        } else {
            None
        }
    }
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

    pub fn search_greater_cards(
        cards: &Vec<Card>,
        greater_than: &Vec<Card>,
    ) -> Option<Vec<usize>> {
        let val = greater_than[0].value;
        let iter_cards = PairSearch(cards);

        iter_cards.into_iter().find(|x| cards[x[0]].value > val)
    }

    pub fn split_from_cards(cards: &mut Vec<Card>) -> Vec<Card> {
        let mut result = Vec::new();
        let indices = PairSearch(cards).into_iter().nth(0);

        match indices {
            Some(x) => {
                for _ in 0..2 {
                    result.push(cards.remove(x[0]));    
                }
            },
            _ => (),
        }
        
        result
    }

    // XXX remove pair pattern validation?
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use super::super::card::Card;
    use super::super::card::Suit;
    use super::Pair;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }

    #[test]
    fn search_greater_test1() {
        let mut cards = generate(vec![3, 3, 3, 4, 4, 5, 6, 7, 9, 9, 9]);
        let handed_in = &cards[3..5].to_vec();
        let result = Pair::search_greater_cards(&cards, handed_in).unwrap();
        assert_eq!(vec![8, 9], result);
    }

    #[test]
    fn search_greater_test2() {
        let mut cards = generate(vec![3, 3, 3, 4, 4, 5, 6, 7, 9, 9, 9]);
        let handed_in = &cards[8..10].to_vec();
        let result = Pair::search_greater_cards(&cards, handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test3() {
        let mut cards = generate(vec![]);
        let handed_in = generate(vec![3, 3]);
        let result = Pair::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test4() {
        let mut cards = generate(vec![3, 4, 5, 6, 7, 7, 8, 9]);
        let handed_in = generate(vec![4, 4]);
        let result = Pair::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![4, 5], result);
    }
}
