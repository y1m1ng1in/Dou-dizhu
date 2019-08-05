use super::super::cards::airplane::Airplane;
use super::super::cards::bomb::*;
use super::super::cards::card::Card;
use super::super::cards::pair::Pair;
use super::super::cards::pairchain::PairChain;
use super::super::cards::solochain::SoloChain;
use super::super::cards::trio::Trio;
use super::super::cards::utils::*;

#[derive(Debug)]
pub struct Strategy {
    bombs: Vec<Card>,
    chains: Vec<Card>,
    trios: Vec<Card>,
    pairs: Vec<Card>,
    solos: Vec<Card>,
}

pub struct ComputerPlayer {
    owned_cards: Vec<Card>,
    strategy: Strategy,
}

pub fn split_from_indice(cards: &mut Vec<Card>, indices: &[usize]) -> Vec<Card> {
    let mut shifted: usize = 0;
    let mut result = Vec::new();
    let mut index: usize;

    for i in indices {
        index = *i - shifted;
        result.push(cards.remove(index));
        shifted += 1;
    }

    result
}

impl Strategy {
    pub fn new() -> Strategy {
        Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: Vec::new(),
        }
    }

    pub fn construct(self: &mut Self, cards: &mut Vec<Card>) {
        self.bombs = Strategy::search_bombs_trios_pairs(cards, 1u32);
        self.chains = Strategy::search_chains(cards);
        self.trios = Strategy::search_bombs_trios_pairs(cards, 2u32);
        self.pairs = Strategy::search_bombs_trios_pairs(cards, 3u32);
        self.solos = cards.to_vec();
    }

    fn search_chains(cards: &mut Vec<Card>) -> Vec<Card> {
        let mut indices;
        let mut has_more = true;
        let mut chains: Vec<Card> = Vec::new();

        while has_more {
            indices = Strategy::search_longest_chain(cards);
            match indices {
                Some(i) => chains.extend(split_from_indice(cards, &i)),
                None => has_more = false,
            }
        }

        chains
    }

    fn search_longest_chain(cards: &[Card]) -> Option<Vec<usize>> {
        let airplane = Airplane::search_longest_cards(cards).unwrap_or(Vec::new());
        let pairchain = PairChain::search_longest_cards(cards).unwrap_or(Vec::new());
        let solochain = SoloChain::search_longest_cards(cards).unwrap_or(Vec::new());

        let airplane_len = airplane.len();
        let pairchain_len = pairchain.len();
        let solochain_len = solochain.len();

        if airplane_len != 0 || pairchain_len != 0 || solochain_len != 0 {
            if airplane_len > pairchain_len && airplane_len > solochain_len {
                Some(airplane)
            } else if pairchain_len > airplane_len && pairchain_len > solochain_len {
                Some(pairchain)
            } else {
                Some(solochain)
            }
        } else {
            None
        }
    }

    fn search_bombs_trios_pairs(cards: &mut Vec<Card>, pattern: u32) -> Vec<Card> {
        let mut has_more = true;
        let mut result: Vec<Card> = Vec::new();
        let mut item: Vec<Card>;

        while has_more {
            item = match pattern {
                1 => Bomb::split_from_cards(cards),
                2 => Trio::split_from_cards(cards),
                3 => Pair::split_from_cards(cards),
                _ => Vec::new(),
            };
            if item.is_empty() {
                has_more = false;
            } else {
                result.extend(item);
            }
        }

        result
    }

    // pattern obtained by last turn handed cards
    pub fn hand_in_greater_from_strategy(
        self: &mut Self,
        greater_than: &[Card],
        pattern: Pattern,
    ) -> Vec<Card> {
        unimplemented!("hand in greater cards from strategy.");
    }

    pub fn hand_in_greater_by_merged(
        self: &mut Self,
        greater_than: &[Card],
        pattern: Pattern,
    ) -> Vec<Card> {
        unimplemented!("merge strategy and then find greater cards.");
    }
}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        self.bombs == other.bombs
            && self.chains == other.chains
            && self.trios == other.trios
            && self.pairs == other.pairs
            && self.solos == other.solos
    }
}

impl ComputerPlayer {
    pub fn new(cards: Vec<Card>) -> ComputerPlayer {
        ComputerPlayer {
            owned_cards: cards,
            strategy: Strategy::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use super::super::super::cards::card::Card;
    use super::super::super::cards::card::Suit;
    use super::Strategy;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }
    /*
    #[test]
    fn strategy_construct_test1() {
        let mut cards = generate(vec![3, 3, 3, 4, 4, 5, 6, 7, 9, 9, 9]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            airplanes: Vec::new(),
            pairchains: Vec::new(),
            solochains: vec![generate(vec![3, 4, 5, 6, 7])],
            trios: vec![generate(vec![9, 9, 9])],
            pairs: vec![generate(vec![3, 3])],
            solos: generate(vec![4]),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }

    #[test]
    fn strategy_construct_test2() {
        let mut cards = generate(vec![3, 3, 3, 4, 4, 5, 5, 6, 7, 9, 12, 12, 12, 12]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: vec![generate(vec![12, 12, 12, 12])],
            airplanes: Vec::new(),
            pairchains: vec![generate(vec![3, 3, 4, 4, 5, 5])],
            solochains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![3, 6, 7, 9]),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }

    #[test]
    fn strategy_construct_test3() {
        let mut cards = generate(vec![3, 3, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            airplanes: Vec::new(),
            pairchains: Vec::new(),
            solochains: vec![generate(vec![3, 4, 5, 6, 7, 8, 9, 10, 11])],
            trios: Vec::new(),
            pairs: vec![generate(vec![3, 3]), generate(vec![4, 4])],
            solos: Vec::new(),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }

    #[test]
    fn strategy_construct_test4() {
        let mut cards = generate(vec![3, 3, 4, 6, 6, 7, 9, 10, 11, 12, 13, 13]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            airplanes: Vec::new(),
            pairchains: Vec::new(),
            solochains: vec![generate(vec![9, 10, 11, 12, 13])],
            trios: Vec::new(),
            pairs: vec![generate(vec![3, 3]), generate(vec![6, 6])],
            solos: generate(vec![4, 7, 13]),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }
    */
}
