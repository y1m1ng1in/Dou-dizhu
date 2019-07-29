use doudizhu::cards::card::{Card, Suit};
use doudizhu::cards::pair::Pair;
use doudizhu::cards::trio::Trio;
use doudizhu::cards::solochain::SoloChain;
use doudizhu::cards::pairchain::PairChain;
use doudizhu::cards::airplane::Airplane;
use doudizhu::cards::bomb::{Bomb, Rocket};

struct Strategy {
    bombs: Vec<Vec<Card>>,
    airplanes: Vec<Vec<Card>>,
    pairchains: Vec<Vec<Card>>,
    solochains: Vec<Vec<Card>>,
    trios: Vec<Vec<Card>>,
    pairs: Vec<Vec<Card>>,
    solos: Vec<Card>,
}

struct ComputerPlayer {
    owned_cards: Vec<Card>,
    strategy: Strategy,
}

pub fn split_from_indice(cards: &mut Vec<Card>, indices: &Vec<usize>) -> Vec<Card> {
    let mut shifted: usize = 0;
    let mut result = Vec::new();

    for i in &indices {
        i -= shifted;
        result.push(cards.remove(i));
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

    pub fn construct(self: &Self, cards: &mut Vec<Card>) {
        let airplanes_and_chains;

        self.bombs = search_bombs_trios_pairs(cards, 1u32);
        
        airplanes_and_chains = search_airplanes_and_chains(cards);
        self.airplanes = airplanes_and_chains.0;
        self.pairchains = airplanes_and_chains.1;
        self.solochains = airplanes_and_chains.2;

        self.trios = search_bombs_trios_pairs(cards, 2u32);
        self.pairs = search_bombs_trios_pairs(cards, 3u32);
        self.solos = cards.to_vec();
    }

    fn search_airplanes_and_chains(cards: &mut Vec<Card>) -> (Vec<Vec<Card>>, Vec<Vec<Card>>, Vec<Vec<Card>>) {
        let mut airplanes: Vec<Vec<Card>> = Vec::new();
        let mut pairchains: Vec<Vec<Card>> = Vec::new();
        let mut solochains: Vec<Vec<Card>> = Vec::new();
        let mut indices;
        let mut has_more = true;

        while has_more {
            indices = search_longest_from_airplane_or_chain(cards);
            if indices.0.is_empty() {
                has_more = false;
            } else {
                match indices.1 {
                    1 => airplanes.push(split_from_indice(cards, indices.0)),
                    2 => pairchains.push(split_from_indice(cards, indices.0)),
                    3 => solochains.push(split_from_indice(cards, indices.0)),
                };
            }
        }

        (airplanes, pairchains, solochains)
    }

    fn search_longest_from_airplane_or_chain(cards: &Vec<Card>) -> (Vec<usize>, u32) {
        let airplane = Airplane::search_longest_cards(cards);
        let pairchain = PairChain::search_longest_cards(cards);
        let solochain = SoloChain::search_longest_cards(cards);
        
        let airplane_len = match airplane {
            Some(x) => x.len(),
            None => 0,
        };
        let pairchain_len = match pairchain {
            Some(x) => x.len(),
            None => 0,
        };
        let solochain_len = match solochain {
            Some(x) => x.len(),
            None => 0,
        };

        if airplane_len != 0 || pairchain_len != 0 || solochain_len != 0 {
            if airplane_len > pairchain_len && airplane_len > solochain_len {
                (airplane.unwrap(), 1)
            } else if pairchain_len > airplane_len && pairchain_len > solochain_len {
                (pairchain.unwrap(), 2)
            } else {
                (solochain.unwrap(), 3)
            }
        } else {
            (Vec::new(), 0)
        }
    }   

    fn search_bombs_trios_pairs(cards: &mut Vec<Card>, type: u32) -> Vec<Vec<Card>> {
        let mut has_more = true;
        let mut result: Vec<Vec<Card>> = Vec::new();
        let mut item: Vec<Card>;

        while has_more {
            item = match type {
                1 => Bomb::split_from_cards(cards),
                2 => Trio::split_from_cards(cards),
                3 => Pair::split_from_cards(cards)
            };
            if a_trio.is_empty() {
                has_more = false;
            } else {
                result.push(item);
            }
        }

        result
    }
}

impl ComputerPlayer {
    pub fn new(cards: Vec<Card>) -> ComputerPlayer {
        ComputerPlayer {
            owned_cards: cards,
            strategy: Strategy::new(cards.clone()),
        }
    }
}

