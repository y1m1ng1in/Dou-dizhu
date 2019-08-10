use super::card::Card;
use super::pair::PairSearch;
use super::trio::TrioSearch;

pub struct Airplane {}

impl Airplane {
    // generate a vector of cards that in form of airplane
    pub fn reorder(cards: &[Card]) -> (Vec<Card>, usize) {
        let (start, trio_num) = Airplane::find_size(cards);
        let mut result: Vec<Card> = Vec::new();

        if trio_num != 0 {
            for i in start..start + 3 * trio_num {
                result.push(cards[i].clone());
            }
            for i in 0..start {
                result.push(cards[i].clone());
            }
            for i in start + 3 * trio_num..cards.len() {
                result.push(cards[i].clone());
            }
        }

        (result, trio_num)
    }

    // cards' value has already sorted in ascending
    pub fn is_airplane(cards: &[Card]) -> bool {
        let trio_num = Airplane::find_size(cards).1;

        (trio_num * 3 == cards.len()
            || trio_num * 3 + trio_num == cards.len()
            || trio_num * 3 + trio_num * 2 == cards.len())
            && trio_num >= 2
    }

    fn find_size(cards: &[Card]) -> (usize, usize) {
        let mut previous = 0;
        let mut max_size: usize = 0;
        let mut max_size_trio_start = 0;

        for indices in TrioSearch(cards) {
            let current = cards[indices[0]].value;

            if previous + 1 == current {
                previous = current;
                max_size += 1;
            } else {
                max_size = 1;
                max_size_trio_start = indices[0];
                previous = current;
            }
        }

        (max_size_trio_start, max_size)
    }

    fn kicker_type(length: usize, size: usize) -> i32 {
        if length == 3 * size {
            0
        } else if length == size + 3 * size {
            1
        } else if length == 2 * size + 3 * size {
            2
        } else {
            -1
        }
    }

    pub fn search_greater_cards(cards: &[Card], greater_than: &[Card]) -> Option<Vec<usize>> {
        let mut previous = 0;
        let mut current_size = 0;
        let mut result = Vec::new();
        let (trio_start, size) = Airplane::find_size(greater_than);

        if size < 2 || trio_start >= cards.len() {
            return None;
        }

        let val = greater_than[trio_start].value;
        let kicker = Airplane::kicker_type(greater_than.len(), size);

        for indices in TrioSearch(cards) {
            let current = cards[indices[0]].value;

            if current > val {
                if previous + 1 == current {
                    previous = current;
                    result.extend(indices);
                    current_size += 1;
                    if current_size == size {
                        break;
                    }
                } else {
                    result = Vec::new();
                    result.extend(indices);
                    current_size = 1;
                    previous = current;
                }
            }
        }

        if current_size == size {
            match kicker {
                0 => Some(result),
                1 => Airplane::search_kickers(cards, &mut result, size, false),
                2 => Airplane::search_kickers(cards, &mut result, size, true),
                _ => None,
            }
        } else {
            None
        }
    }

    fn search_kickers(
        cards: &[Card],
        trio_indices: &mut Vec<usize>,
        size: usize,
        is_pair_kicker: bool,
    ) -> Option<Vec<usize>> {
        let mut kickers: Vec<usize> = Vec::new();
        let mut current_size = 0;

        if !is_pair_kicker {
            for i in 0..cards.len() {
                if !trio_indices.contains(&i) {
                    kickers.push(i);
                    current_size += 1;
                }
                if current_size == size {
                    break;
                }
            }
        } else {
            for indices in PairSearch(cards) {
                if !trio_indices.contains(&indices[0]) && !trio_indices.contains(&indices[1]) {
                    kickers.push(indices[0]);
                    kickers.push(indices[1]);
                    current_size += 1;
                }
                if current_size == size {
                    break;
                }
            }
        }

        if current_size == size {
            trio_indices.extend(kickers);
            Some(trio_indices.to_vec())
        } else {
            None
        }
    }

    pub fn search_longest_cards(cards: &[Card]) -> Option<Vec<usize>> {
        let mut largest = Vec::new();
        let mut temp = Vec::new();
        let mut previous: u32 = 0;

        for indices in TrioSearch(cards) {
            let current = cards[indices[0]].value;

            if previous + 1 == current {
                previous = current;
                temp.extend(indices);
            } else {
                if temp.len() > largest.len() {
                    largest = Vec::new();
                    largest.append(&mut temp);
                } else {
                    temp = Vec::new();
                }
                temp.extend(indices);
                previous = current;
            }
        }

        if temp.len() > largest.len() {
            largest = Vec::new();
            largest.append(&mut temp);
        }

        if largest.len() >= 6 {
            Airplane::search_longest_kickers(cards, &mut largest)
        } else {
            None
        }
    }

    fn search_longest_kickers(cards: &[Card], trio_indices: &mut Vec<usize>) -> Option<Vec<usize>> {
        let size: usize = trio_indices.len() / 3;
        let mut append_solos = trio_indices.clone();
        let mut append_pairs = trio_indices.clone();

        let p = Airplane::search_kickers(cards, &mut append_pairs, size, true);
        if p.is_none() {
            let s = Airplane::search_kickers(cards, &mut append_solos, size, false);
            if s.is_none() {
                if size > 2 {
                    let mut pr = trio_indices.clone();
                    for _ in 0..3 {
                        pr.pop();
                    }
                    let p_reduced = Airplane::search_kickers(cards, &mut pr, size - 1, true);
                    if p_reduced.is_some() {
                        p_reduced
                    } else {
                        Some(trio_indices.to_vec())
                    }
                } else {
                    Some(trio_indices.to_vec())
                }
            } else {
                s
            }
        } else {
            p
        }
    }

    pub fn compare(c1: &[Card], c2: &[Card]) -> i32 {
        if Airplane::is_airplane(c1) && Airplane::is_airplane(c2) {
            let reformed_c1 = Airplane::reorder(c1);
            let reformed_c2 = Airplane::reorder(c2);

            if reformed_c1.1 == reformed_c2.1 && reformed_c1.0.len() == reformed_c2.0.len() {
                if reformed_c1.0[0].value > reformed_c2.0[0].value {
                    1
                } else {
                    0
                }
            } else {
                -1
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
    use super::Airplane;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }

    #[test]
    fn find_size_test1() {
        let handed_in = generate(vec![3, 3, 3, 4, 4, 4]);
        let result = Airplane::find_size(&handed_in);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn find_size_test2() {
        let handed_in = generate(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 9, 10, 11]);
        let result = Airplane::find_size(&handed_in);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 3);
    }

    #[test]
    fn is_airplane_test() {
        let c1 = generate(vec![3, 3, 3, 4, 4, 4]);
        let c2 = generate(vec![3, 3, 3, 4, 4]);
        let c3 = generate(vec![3, 3, 3, 4, 4, 4, 5, 6]);
        let c4 = generate(vec![3, 3, 3, 3, 4, 4, 4, 4]);
        let c5 = generate(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 7, 8]);
        let c6 = generate(vec![3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 7, 8]);

        assert_eq!(true, Airplane::is_airplane(&c1));
        assert_eq!(false, Airplane::is_airplane(&c2));
        assert_eq!(true, Airplane::is_airplane(&c3));
        assert_eq!(true, Airplane::is_airplane(&c4));
        assert_eq!(true, Airplane::is_airplane(&c5));
        assert_eq!(true, Airplane::is_airplane(&c6));
    }

    #[test]
    fn search_greater_test1() {
        let cards = generate(vec![4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 9]);
        let handed_in = generate(vec![3, 3, 3, 4, 4, 4, 5, 5, 6, 6]);
        let result = Airplane::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }

    #[test]
    fn search_greater_test2() {
        let cards = generate(vec![5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 9, 10]);
        let handed_in = generate(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 9, 10, 11]);
        let result = Airplane::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], result);
    }

    #[test]
    fn search_greater_test3() {
        let cards = generate(vec![3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 8]);
        let handed_in = generate(vec![3, 3, 3, 4, 4, 4, 5, 6]);
        let result = Airplane::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test4() {
        let cards = generate(vec![]);
        let handed_in = generate(vec![4, 4, 4, 5, 5, 5, 6, 6, 7, 7]);
        let result = Airplane::search_greater_cards(&cards, &handed_in);
        assert_eq!(None, result);
    }

    #[test]
    fn search_greater_test5() {
        let cards = generate(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7]);
        let handed_in = generate(vec![4, 4, 4, 5, 5, 5]);
        let result = Airplane::search_greater_cards(&cards, &handed_in).unwrap();
        assert_eq!(vec![6, 7, 8, 10, 11, 12], result);
    }
}
