use super::super::cards::airplane::Airplane;
use super::super::cards::bomb::*;
use super::super::cards::card::Card;
use super::super::cards::pair::Pair;
use super::super::cards::pair::PairSearch;
use super::super::cards::pairchain::PairChain;
use super::super::cards::solochain::SoloChain;
use super::super::cards::trio::Trio;
use super::super::cards::trio::TrioSearch;
use super::super::cards::utils::*;
use std::fmt;

#[derive(Debug)]
pub struct Strategy {
    bombs: Vec<Card>,
    chains: Vec<Card>,
    trios: Vec<Card>,
    pairs: Vec<Card>,
    solos: Vec<Card>,
}

pub struct ComputerPlayer {
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

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        s = s + "Bombs:\n";
        for i in &self.bombs {
            s = s + &i.to_string();
        }
        s = s + "Chains:\n";
        for i in &self.chains {
            s = s + &i.to_string();
        }
        s = s + "Trios:\n";
        for i in &self.trios {
            s = s + &i.to_string();
        }
        s = s + "Pairs:\n";
        for i in &self.pairs {
            s = s + &i.to_string();
        }
        s = s + "Solos:\n";
        for i in &self.solos {
            s = s + &i.to_string();
        }
        write!(f, "{}", &s)
    }
}

impl fmt::Display for ComputerPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.strategy)
    }
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

    pub fn all(self: &Self) -> Vec<Card> {
        let mut all_cards = Vec::new();

        all_cards.append(&mut self.bombs.clone());
        all_cards.append(&mut self.chains.clone());
        all_cards.append(&mut self.trios.clone());
        all_cards.append(&mut self.pairs.clone());
        all_cards.append(&mut self.solos.clone());

        all_cards.sort_unstable();

        all_cards
    }

    fn search_chains(cards: &mut Vec<Card>) -> Vec<Card> {
        let mut indices;
        let mut has_more = true;
        let mut chains: Vec<Card> = Vec::new();

        while has_more {
            indices = Strategy::search_longest_chain(cards).0;
            match indices {
                Some(i) => chains.extend(split_from_indice(cards, &i)),
                None => has_more = false,
            }
        }

        chains.sort_unstable();

        chains
    }

    fn search_longest_chain(cards: &[Card]) -> (Option<Vec<usize>>, Pattern) {
        let mut airplane = Airplane::search_longest_cards(cards).unwrap_or(Vec::new());
        let pairchain = PairChain::search_longest_cards(cards).unwrap_or(Vec::new());
        let solochain = SoloChain::search_longest_cards(cards).unwrap_or(Vec::new());

        let airplane_len = airplane.len();
        let pairchain_len = pairchain.len();
        let solochain_len = solochain.len();

        if airplane_len != 0 || pairchain_len != 0 || solochain_len != 0 {
            if airplane_len > pairchain_len && airplane_len > solochain_len {
                airplane.sort_unstable();
                (Some(airplane), Pattern::Airplane)
            } else if pairchain_len > airplane_len && pairchain_len > solochain_len {
                (Some(pairchain), Pattern::PairChain)
            } else {
                (Some(solochain), Pattern::SoloChain)
            }
        } else {
            (None, Pattern::Invalid)
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
        let mut indices;

        match pattern {
            Pattern::Bomb => {
                indices =
                    Bomb::search_greater_cards(&self.bombs, greater_than).unwrap_or(Vec::new());
                split_from_indice(&mut self.bombs, &indices)
            }
            Pattern::Airplane => {
                indices = Airplane::search_greater_cards(&self.chains, greater_than)
                    .unwrap_or(Vec::new());
                indices.sort_unstable();
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

    // search player's largest card for each pattern and hand in
    pub fn hand_in_first(self: &mut Self, player: &[Card]) -> Vec<Card> {
        let mut no_greater = false;
        let mut removed: Vec<Card> = self.hand_in_first_from_chain(player);

        if removed.is_empty() {
            removed = self.hand_in_first_from_other(player, Pattern::Trio);
            if removed.is_empty() {
                removed = self.hand_in_first_from_other(player, Pattern::Pair);
                if removed.is_empty() {
                    removed = self.hand_in_first_from_other(player, Pattern::Solo);
                    if removed.is_empty() {
                        no_greater = true;
                    }
                }
            }
        }

        if no_greater {
            let indices = Strategy::search_longest_chain(&self.chains).0;
            match indices {
                Some(i) => split_from_indice(&mut self.chains, &i),
                None => {
                    removed = Trio::split_from_cards(&mut self.trios);
                    if removed.is_empty() {
                        removed = Pair::split_from_cards(&mut self.pairs);
                        if removed.is_empty() {
                            if !self.solos.is_empty() {
                                removed = vec![self.solos.remove(0)];
                            } else {
                                removed = Bomb::split_from_cards(&mut self.bombs);
                            }
                        }
                    }
                    removed
                }
            }
        } else {
            removed
        }
    }

    fn hand_in_first_from_chain(self: &mut Self, player: &[Card]) -> Vec<Card> {
        let (indices, pattern) = Strategy::search_longest_chain(&self.chains);
        let chain_indices = indices.unwrap_or(Vec::new());
        let candidate = Strategy::clone_cards_from_indices(&chain_indices, &self.chains);
        let has_greater: Option<Vec<usize>>;

        if !candidate.is_empty() {
            has_greater = match pattern {
                Pattern::Airplane => Airplane::search_greater_cards(player, &candidate),
                Pattern::PairChain => PairChain::search_greater_cards(player, &candidate),
                Pattern::SoloChain => SoloChain::search_greater_cards(player, &candidate),
                _ => None,
            };
            if has_greater.is_none() {
                split_from_indice(&mut self.chains, &chain_indices)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    fn hand_in_first_from_other(self: &mut Self, player: &[Card], pattern: Pattern) -> Vec<Card> {
        let indices;
        let candidate: Vec<Card>;
        let smallest;

        match pattern {
            Pattern::Trio => {
                indices = TrioSearch(&self.trios)
                    .into_iter()
                    .last()
                    .unwrap_or(Vec::new());
                smallest = TrioSearch(&self.trios)
                    .into_iter()
                    .nth(0)
                    .unwrap_or(Vec::new());
                candidate = Strategy::clone_cards_from_indices(&indices, &self.trios);
                if Trio::search_greater_cards(player, &candidate).is_none() {
                    split_from_indice(&mut self.trios, &smallest)
                } else {
                    vec![]
                }
            }
            Pattern::Pair => {
                indices = PairSearch(&self.pairs)
                    .into_iter()
                    .last()
                    .unwrap_or(Vec::new());
                smallest = PairSearch(&self.pairs)
                    .into_iter()
                    .nth(0)
                    .unwrap_or(Vec::new());
                candidate = Strategy::clone_cards_from_indices(&indices, &self.pairs);
                if Pair::search_greater_cards(player, &candidate).is_none() {
                    split_from_indice(&mut self.pairs, &smallest)
                } else {
                    vec![]
                }
            }
            Pattern::Solo => {
                if !self.solos.is_empty() {
                    indices = vec![self.solos.len() - 1];
                    candidate = Strategy::clone_cards_from_indices(&indices, &self.solos);
                    if Card::search_greater_cards(player, &candidate).is_none() {
                        split_from_indice(&mut self.solos, &vec![0])
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    fn clone_cards_from_indices(indices: &[usize], cards: &[Card]) -> Vec<Card> {
        let mut result = Vec::new();

        if indices.iter().max().unwrap_or(&cards.len()) < &cards.len() {
            for i in indices {
                result.push(cards[*i].clone());
            }
        }

        result
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
    pub fn new(mut cards: Vec<Card>) -> ComputerPlayer {
        let mut s = Strategy::new();
        s.construct(&mut cards);

        ComputerPlayer { strategy: s }
    }

    pub fn hand_in_follow(self: &mut Self, greater_than: &[Card], pattern: Pattern) -> Vec<Card> {
        let handed = self
            .strategy
            .hand_in_greater_from_strategy(greater_than, pattern);

        if handed.is_empty() {
            self.strategy
                .hand_in_greater_by_merged(greater_than, pattern)
        } else {
            self.strategy.reconstruct();
            handed
        }
    }

    pub fn hand_in_first(self: &mut Self, player: &[Card]) -> Vec<Card> {
        let handed = self.strategy.hand_in_first(player);

        self.strategy.reconstruct();
        handed
    }

    pub fn display(self: &Self) -> Vec<Card> {
        self.strategy.all()
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
            chains: generate(vec![5, 6, 7, 8, 9, 10]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 10, 10]),
            solos: generate(vec![7, 8]),
        };
        let r =
            x.hand_in_greater_from_strategy(&generate(vec![6, 7, 8, 9, 10]), Pattern::SoloChain);
        let y = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![5, 6, 7, 8, 9, 10]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 10, 10]),
            solos: generate(vec![7, 8]),
        };
        assert_eq!(x, y);
        assert_eq!(r, generate(vec![]));

        let r1 =
            x.hand_in_greater_from_strategy(&generate(vec![5, 6, 7, 8, 9]), Pattern::SoloChain);
        let z = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![5]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 10, 10]),
            solos: generate(vec![7, 8]),
        };
        assert_eq!(x, z);
        assert_eq!(r1, generate(vec![6, 7, 8, 9, 10]));
    }

    #[test]
    fn hand_in_merged_test1() {
        let mut x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![7, 7, 8, 8, 9, 9]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 13, 13]),
            solos: generate(vec![6, 10]),
        };
        let r = x.hand_in_greater_by_merged(&generate(vec![5, 6, 7, 8, 9]), Pattern::SoloChain);
        let y = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 13, 13]),
            solos: generate(vec![7, 8, 9]),
        };
        assert_eq!(x, y);
        assert_eq!(r, generate(vec![6, 7, 8, 9, 10]));

        let r1 =
            x.hand_in_greater_by_merged(&generate(vec![9, 10, 11, 12, 13]), Pattern::SoloChain);
        let z = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 13, 13]),
            solos: generate(vec![7, 8, 9]),
        };
        assert_eq!(x, z);
        assert_eq!(r1, generate(vec![]));

        let r2 = x.hand_in_greater_by_merged(&generate(vec![10]), Pattern::Solo);
        let z1 = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![3, 3]),
            solos: generate(vec![7, 8, 9, 13]),
        };
        assert_eq!(x, z1);
        assert_eq!(r2, generate(vec![13]));
    }

    #[test]
    fn hand_in_first_test1() {
        let p = generate(vec![3, 3, 4, 8, 8, 9, 9, 10, 10, 13, 14]);
        let mut x = Strategy {
            bombs: Vec::new(),
            chains: generate(vec![4, 4, 5, 5, 6, 6, 7, 7]),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 5, 5]),
            solos: generate(vec![7, 8, 9, 11]),
        };
        let y = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 5, 5]),
            solos: generate(vec![7, 8, 9, 11]),
        };
        let h = x.hand_in_first(&p);
        assert_eq!(h, generate(vec![4, 4, 5, 5, 6, 6, 7, 7]));
        assert_eq!(x, y);
    }

    #[test]
    fn hand_in_first_test2() {
        let p = generate(vec![3, 3, 4, 4, 7, 7, 9, 12, 13, 15]);
        let mut x = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![3, 3, 5, 5, 8, 8]),
            solos: generate(vec![7, 8, 9, 11]),
        };
        let y = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![5, 5, 8, 8]),
            solos: generate(vec![7, 8, 9, 11]),
        };
        let h = x.hand_in_first(&p);
        assert_eq!(h, generate(vec![3, 3]));
        assert_eq!(x, y);
    }

    #[test]
    fn hand_in_first_test3() {
        let p = generate(vec![3, 3, 4, 4, 7, 9, 12]);
        let mut x = Strategy {
            bombs: generate(vec![5, 5, 5, 5]),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![7, 8, 9, 11]),
        };
        let y = Strategy {
            bombs: generate(vec![5, 5, 5, 5]),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![8, 9, 11]),
        };
        let h = x.hand_in_first(&p);
        assert_eq!(h, generate(vec![7]));
        assert_eq!(x, y);
    }

    #[test]
    fn hand_in_first_test4() {
        let p = generate(vec![7, 9, 12]);
        let mut x = Strategy {
            bombs: generate(vec![5, 5, 5, 5]),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: Vec::new(),
        };
        let y = Strategy {
            bombs: Vec::new(),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: Vec::new(),
        };
        let h = x.hand_in_first(&p);
        assert_eq!(h, generate(vec![5, 5, 5, 5]));
        assert_eq!(x, y);
    }

    #[test]
    fn hand_in_first_test5() {
        let p = generate(vec![3, 4, 6, 8, 8, 8, 9, 9, 9, 10, 13]);
        let mut x = Strategy {
            bombs: generate(vec![8, 8, 8, 8]),
            chains: generate(vec![4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 9, 9]),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![10, 11, 13]),
        };
        let y = Strategy {
            bombs: generate(vec![8, 8, 8, 8]),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: Vec::new(),
            solos: generate(vec![10, 11, 13]),
        };
        let h = x.hand_in_first(&p);
        assert_eq!(h, generate(vec![4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 9, 9]));
        assert_eq!(x, y);
    }

    #[test]
    fn hand_in_first_test6() {
        let p = generate(vec![6, 7, 8, 10, 11, 11, 14, 14]);
        let mut x = Strategy {
            bombs: generate(vec![8, 8, 8, 8]),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![4, 4, 6, 6, 7, 7, 9, 9]),
            solos: generate(vec![3, 4, 5, 7, 9, 15]),
        };
        let y = Strategy {
            bombs: generate(vec![8, 8, 8, 8]),
            chains: Vec::new(),
            trios: Vec::new(),
            pairs: generate(vec![4, 4, 6, 6, 7, 7, 9, 9]),
            solos: generate(vec![4, 5, 7, 9, 15]),
        };
        let h = x.hand_in_first(&p);
        assert_eq!(h, generate(vec![3]));
        assert_eq!(x, y);
    }
}
