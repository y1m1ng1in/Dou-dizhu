use super::card::Card;

pub struct Bomb {}

impl Bomb {
    pub fn is_bomb(cards: &[Card]) -> bool {
        cards.len() == 4
            && cards[0].value == cards[1].value
            && cards[1].value == cards[2].value
            && cards[2].value == cards[3].value
    }

    pub fn search_greater_cards(cards: &[Card], greater_than: &[Card]) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let mut result: Vec<usize> = Vec::new();

        while i + 3 < cards.len() {
            if Bomb::is_bomb(&cards[i..i + 4]) {
                if cards[i].value > greater_than[0].value {
                    result = (i..i + 4).collect();
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

    pub fn split_from_cards(cards: &mut Vec<Card>) -> Vec<Card> {
        let mut i: usize = 0;
        let mut result = Vec::new();

        while i + 3 < cards.len() {
            if Bomb::is_bomb(&cards[i..i + 4]) {
                for _ in 0..4 {
                    result.push(cards.remove(i));
                }
                break;
            } else {
                i += 1;
            }
        }

        result
    }

    pub fn compare(c1: &[Card], c2: &[Card]) -> i32 {
        if Bomb::is_bomb(c1) && Bomb::is_bomb(c2) {
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
    use super::Bomb;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }

    #[test]
    fn is_bomb_test() {
        let c1 = generate(vec![4, 4, 4, 4]);
        let c2 = generate(vec![4, 4, 4]);
        let c3 = generate(vec![5, 5, 5, 5, 5, 5]);
        let c4 = generate(vec![3, 3, 3, 4]);
        let c5 = generate(vec![]);

        assert_eq!(Bomb::is_bomb(&c1), true);
        assert_eq!(Bomb::is_bomb(&c2), false);
        assert_eq!(Bomb::is_bomb(&c3), false);
        assert_eq!(Bomb::is_bomb(&c4), false);
        assert_eq!(Bomb::is_bomb(&c5), false);
    }

    #[test]
    fn search_greater_test1() {
        let cards = generate(vec![3, 3, 3, 4, 4, 4, 4, 7, 8, 9, 9, 9]);
        let handed_in = generate(vec![3, 3, 3, 3]);
        let result = Bomb::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![3, 4, 5, 6], result);
    }

    #[test]
    fn search_greater_test2() {
        let cards = generate(vec![3, 3, 3, 4, 4, 6, 7, 8, 9, 9, 9, 9]);
        let handed_in = generate(vec![4, 4, 4, 4]);
        let result = Bomb::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![8, 9, 10, 11], result);
    }

    #[test]
    fn search_greater_test3() {
        let cards = generate(vec![3, 3, 3, 4, 4, 6, 7, 8, 9, 9, 9, 11]);
        let handed_in = generate(vec![6, 6, 6, 6]);
        let result = Bomb::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test4() {
        let cards = generate(vec![3, 4, 5]);
        let handed_in = generate(vec![6, 6, 6, 6]);
        let result = Bomb::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test5() {
        let cards = generate(vec![]);
        let handed_in = generate(vec![6, 6, 6, 6]);
        let result = Bomb::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }
}
