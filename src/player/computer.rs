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

    pub fn reconstruct(self: &mut Self) {
        let mut all_cards = Vec::new();

        all_cards.append(&mut self.bombs);
        all_cards.append(&mut self.chains);
        all_cards.append(&mut self.trios);
        all_cards.append(&mut self.pairs);
        all_cards.append(&mut self.solos);

        all_cards.sort_unstable();

        self.construct(&mut all_cards);
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

        chains.sort_unstable();

        chains
    }

    fn search_longest_chain(cards: &[Card]) -> Option<Vec<usize>> {
        let mut airplane = Airplane::search_longest_cards(cards).unwrap_or(Vec::new());
        let pairchain = PairChain::search_longest_cards(cards).unwrap_or(Vec::new());
        let solochain = SoloChain::search_longest_cards(cards).unwrap_or(Vec::new());

        let airplane_len = airplane.len();
        let pairchain_len = pairchain.len();
        let solochain_len = solochain.len();

        if airplane_len != 0 || pairchain_len != 0 || solochain_len != 0 {
            if airplane_len > pairchain_len && airplane_len > solochain_len {
                airplane.sort_unstable(); 
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
        let indices;

        match pattern {
            Pattern::Bomb => {
                indices =
                    Bomb::search_greater_cards(&self.bombs, greater_than).unwrap_or(Vec::new());
                split_from_indice(&mut self.bombs, &indices)
            }
            Pattern::Airplane => {
                indices = Airplane::search_greater_cards(&self.chains, greater_than)
                    .unwrap_or(Vec::new());
                split_from_indice(&mut self.chains, &indices)
            }
            Pattern::SoloChain => {
                indices = SoloChain::search_greater_cards(&self.chains, greater_than)
                    .unwrap_or(Vec::new());
                split_from_indice(&mut self.chains, &indices)
            }
            Pattern::PairChain => {
                indices = PairChain::search_greater_cards(&self.chains, greater_than)
                    .unwrap_or(Vec::new());
                split_from_indice(&mut self.chains, &indices)
            }
            Pattern::Trio => {
                indices =
                    Trio::search_greater_cards(&self.trios, greater_than).unwrap_or(Vec::new());
                split_from_indice(&mut self.trios, &indices)
            }
            Pattern::Pair => {
                indices =
                    Pair::search_greater_cards(&self.pairs, greater_than).unwrap_or(Vec::new());
                split_from_indice(&mut self.pairs, &indices)
            }
            Pattern::Solo => {
                indices =
                    Card::search_greater_cards(&self.solos, greater_than).unwrap_or(Vec::new());
                split_from_indice(&mut self.solos, &indices)
            }
            Pattern::Invalid => Vec::new(),
        }
    }

    pub fn hand_in_greater_by_merged(
        self: &mut Self,
        greater_than: &[Card],
        pattern: Pattern,
    ) -> Vec<Card> {
        let mut all_cards = Vec::new();
        let indices;
        let removed;

        all_cards.append(&mut self.bombs);
        all_cards.append(&mut self.chains);
        all_cards.append(&mut self.trios);
        all_cards.append(&mut self.pairs);
        all_cards.append(&mut self.solos);

        all_cards.sort_unstable();

        match pattern {
            Pattern::Bomb => {
                indices =
                    Bomb::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::Airplane => {
                indices =
                    Airplane::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::SoloChain => {
                indices =
                    SoloChain::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::PairChain => {
                indices =
                    PairChain::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::Trio => {
                indices =
                    Trio::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::Pair => {
                indices =
                    Pair::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::Solo => {
                indices =
                    Card::search_greater_cards(&all_cards, greater_than).unwrap_or(Vec::new());
                removed = split_from_indice(&mut all_cards, &indices);
            }
            Pattern::Invalid => {
                removed = Vec::new();
            }
        };

        self.construct(&mut all_cards);

        removed
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
    use super::super::super::cards::utils::*;
    use super::Strategy;

    fn generate(values: Vec<u32>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();

        for i in values {
            result.push(Card::new(i, Suit::Club, false));
        }

        result
    }

    #[test]
    fn strategy_construct_test1() {
        let mut cards = generate(vec![3, 3, 3, 4, 4, 5, 6, 7, 9, 9, 9]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![3, 4, 5, 6, 7]),
            trios: generate(vec![9, 9, 9]),
            pairs: generate(vec![3, 3]),
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
            bombs: generate(vec![12, 12, 12, 12]),
            chains: generate(vec![3, 3, 4, 4, 5, 5]),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![3, 6, 7, 9]),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }

    #[test]
    fn strategy_construct_test3() {
        let mut cards = generate(vec![3, 3, 5, 6, 7, 7, 7, 8, 8, 8, 10, 10]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![3, 3, 7, 7, 7, 8, 8, 8, 10, 10]),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![5, 6]),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }

    #[test]
    fn strategy_construct_test4() {
        let mut cards = generate(vec![3, 3, 4, 4, 5, 5, 7, 9, 10, 11, 12, 13, 13]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![3, 3, 4, 4, 5, 5, 9, 10, 11, 12, 13]),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![7, 13]),
        };
        s.construct(&mut cards);
        assert_eq!(s, x);
    }

    #[test]
    fn hand_in_from_strategy_test1() {
        let mut x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![3, 3, 4, 4, 5, 5, 9, 10, 11, 12, 13]),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![7, 13]),
        };
        let r = x.hand_in_greater_from_strategy(&generate(vec![5, 6, 7, 8, 9]), Pattern::SoloChain);
        let y = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![3, 3, 4, 4, 5, 5]),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![7, 13]),
        };
        assert_eq!(x, y);
        assert_eq!(r, generate(vec![9, 10, 11, 12, 13]));
    }

    #[test]
    fn hand_in_from_strategy_test2() {
        let mut x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![4, 4, 5, 5, 6, 6]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 7, 7, 8, 8, 10, 10, 12, 12]),
            solos: generate(vec![7, 13]),
        };
        let r = x.hand_in_greater_from_strategy(&generate(vec![3, 3]), Pattern::Pair);
        let y = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![4, 4, 5, 5, 6, 6]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 8, 8, 10, 10, 12, 12]),
            solos: generate(vec![7, 13]),
        };
        assert_eq!(x, y);
        assert_eq!(r, generate(vec![7, 7]));

        let r1 = x.hand_in_greater_from_strategy(&generate(vec![9, 9]), Pattern::Pair);
        let z = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![4, 4, 5, 5, 6, 6]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 8, 8, 12, 12]),
            solos: generate(vec![7, 13]),
        };
        assert_eq!(x, z);
        assert_eq!(r1, generate(vec![10, 10]));
    }

    #[test]
    fn hand_in_from_strategy_test3() {
        let mut x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![5,6,7,8,9,10]),
            trios: Vec::new(),
            pairs: generate(vec![3,3,10,10]),
            solos: generate(vec![7,8]),
        };
        let r = x.hand_in_greater_from_strategy(&generate(vec![6,7,8,9,10]), Pattern::SoloChain);
        let y = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![5,6,7,8,9,10]),
            trios: Vec::new(),
            pairs: generate(vec![3,3,10,10]),
            solos: generate(vec![7,8]),
        };
        assert_eq!(x, y);
        assert_eq!(r, generate(vec![]));

        let r1 = x.hand_in_greater_from_strategy(&generate(vec![5,6,7,8,9]), Pattern::SoloChain);
        let z = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![5]),
            trios: Vec::new(),
            pairs: generate(vec![3,3,10,10]),
            solos: generate(vec![7,8]),
        };
        assert_eq!(x, z);
        assert_eq!(r1, generate(vec![6,7,8,9,10]));
    }
}
