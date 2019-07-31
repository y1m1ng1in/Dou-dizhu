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
    bombs: Vec<Vec<Card>>,
    airplanes: Vec<Vec<Card>>,
    pairchains: Vec<Vec<Card>>,
    solochains: Vec<Vec<Card>>,
    trios: Vec<Vec<Card>>,
    pairs: Vec<Vec<Card>>,
    solos: Vec<Card>,
}

pub struct ComputerPlayer {
    owned_cards: Vec<Card>,
    strategy: Strategy,
}

pub fn split_from_indice(cards: &mut Vec<Card>, indices: &Vec<usize>) -> Vec<Card> {
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
            airplanes: Vec::new(),
            pairchains: Vec::new(),
            solochains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: Vec::new(),
        }
    }

    pub fn construct(self: &mut Self, cards: &mut Vec<Card>) {
        let airplanes_and_chains;

        self.bombs = Strategy::search_bombs_trios_pairs(cards, 1u32);

        airplanes_and_chains = Strategy::search_airplanes_and_chains(cards);
        self.airplanes = airplanes_and_chains.0;
        self.pairchains = airplanes_and_chains.1;
        self.solochains = airplanes_and_chains.2;

        self.trios = Strategy::search_bombs_trios_pairs(cards, 2u32);
        self.pairs = Strategy::search_bombs_trios_pairs(cards, 3u32);
        self.solos = cards.to_vec();
    }

    fn search_airplanes_and_chains(
        cards: &mut Vec<Card>,
    ) -> (Vec<Vec<Card>>, Vec<Vec<Card>>, Vec<Vec<Card>>) {
        let mut airplanes: Vec<Vec<Card>> = Vec::new();
        let mut pairchains: Vec<Vec<Card>> = Vec::new();
        let mut solochains: Vec<Vec<Card>> = Vec::new();
        let mut indices;
        let mut has_more = true;

        while has_more {
            indices = Strategy::search_longest_from_airplane_or_chain(cards);
            if indices.0.is_empty() {
                has_more = false;
            } else {
                match indices.1 {
                    1u32 => airplanes.push(split_from_indice(cards, &indices.0)),
                    2u32 => pairchains.push(split_from_indice(cards, &indices.0)),
                    3u32 => solochains.push(split_from_indice(cards, &indices.0)),
                    _ => (),
                };
            }
        }

        (airplanes, pairchains, solochains)
    }

    fn search_longest_from_airplane_or_chain(cards: &Vec<Card>) -> (Vec<usize>, u32) {
        let airplane = Airplane::search_longest_cards(cards);
        let pairchain = PairChain::search_longest_cards(cards);
        let solochain = SoloChain::search_longest_cards(cards);

        let airplane_indices = match airplane {
            Some(x) => x,
            None => Vec::new(),
        };
        let pairchain_indices = match pairchain {
            Some(x) => x,
            None => Vec::new(),
        };
        let solochain_indices = match solochain {
            Some(x) => x,
            None => Vec::new(),
        };

        let airplane_len = airplane_indices.len();
        let pairchain_len = pairchain_indices.len();
        let solochain_len = solochain_indices.len();

        if airplane_len != 0 || pairchain_len != 0 || solochain_len != 0 {
            if airplane_len > pairchain_len && airplane_len > solochain_len {
                (airplane_indices, 1)
            } else if pairchain_len > airplane_len && pairchain_len > solochain_len {
                (pairchain_indices, 2)
            } else {
                (solochain_indices, 3)
            }
        } else {
            (Vec::new(), 0)
        }
    }

    fn search_bombs_trios_pairs(cards: &mut Vec<Card>, pattern: u32) -> Vec<Vec<Card>> {
        let mut has_more = true;
        let mut result: Vec<Vec<Card>> = Vec::new();
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
                result.push(item);
            }
        }

        result
    }

    // pattern obtained by last turn handed cards
    pub fn hand_in_greater_from_strategy(
        self: &mut Self,
        greater_than: &Vec<Card>,
        pattern: Pattern,
    ) -> Vec<Card> {
        let mut invalid: Vec<Vec<Card>> = Vec::new();
        let candidates = match pattern {
            Pattern::Bomb => &mut self.bombs,
            Pattern::Airplane => &mut self.airplanes,
            Pattern::PairChain => &mut self.pairchains,
            Pattern::SoloChain => &mut self.solochains,
            Pattern::Trio => &mut self.trios,
            Pattern::Pair => &mut self.pairs,
            _ => &mut invalid,
        };
        let mut result: Vec<Card> = Vec::new();

        if pattern != Pattern::Solo {
            for i in 0..candidates.len() {
                if compare_known_pattern(&candidates[i], greater_than, pattern) == 1 {
                    result = candidates.remove(i).to_vec();
                    break;
                }
            }
        } else {
            for i in 0..self.solos.len() {
                if compare_known_pattern(&vec![self.solos[i]], greater_than, pattern) == 1 {
                    result = vec![self.solos.remove(i)];
                    break;
                }
            }
        }

        result
    }

    pub fn hand_in_greater_by_merged(
        self: &mut Self,
        greater_than: &Vec<Card>,
        pattern: Pattern,
    ) -> Vec<Card> {
        unimplemented!("merge strategy and then find greater cards.");
    }
}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        self.bombs == other.bombs
            && self.airplanes == other.airplanes
            && self.pairchains == other.pairchains
            && self.solochains == other.solochains
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
        let mut cards = generate(vec![3, 3, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10]);
        let mut s = Strategy::new();
        let x = Strategy {
            bombs: Vec::new(),
            airplanes: Vec::new(),
            pairchains: Vec::new(),
            solochains: vec![generate(vec![3, 4, 5, 6, 7, 8, 9, 10])],
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
}
