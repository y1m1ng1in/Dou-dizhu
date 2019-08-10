use super::airplane::Airplane;
use super::bomb::*;
use super::card::Card;
use super::pair::Pair;
use super::pairchain::PairChain;
use super::solochain::SoloChain;
use super::trio::Trio;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Pattern {
    Bomb,
    Airplane,
    PairChain,
    SoloChain,
    Trio,
    Pair,
    Solo,
    Invalid,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pattern::Bomb => write!(f, "Bomb"),
            Pattern::Airplane => write!(f, "Airplane"),
            Pattern::PairChain => write!(f, "PairChain"),
            Pattern::SoloChain => write!(f, "SoloChain"),
            Pattern::Trio => write!(f, "Trio"),
            Pattern::Pair => write!(f, "Pair"),
            Pattern::Solo => write!(f, "Solo"),
            Pattern::Invalid => write!(f, "Invalid"),
        }
    }
}

pub fn get_pattern(cards: &[Card]) -> Pattern {
    if Airplane::is_airplane(&cards) {
        Pattern::Airplane
    } else if Bomb::is_bomb(cards) {
        Pattern::Bomb
    } else if PairChain::is_pair_chain(cards) {
        Pattern::PairChain
    } else if SoloChain::is_solo_chain(cards) {
        Pattern::SoloChain
    } else if Trio::is_trio(cards) {
        Pattern::Trio
    } else if Pair::is_pair(cards) {
        Pattern::Pair
    } else if cards.len() == 1 {
        Pattern::Solo
    } else {
        Pattern::Invalid
    }
}

pub fn compare(c1: &[Card], c2: &[Card]) -> i32 {
    let p = get_pattern(c1);

    if get_pattern(c2) == p {
        match p {
            Pattern::Bomb => Bomb::compare(c1, c2),
            Pattern::Airplane => Airplane::compare(c1, c2),
            Pattern::PairChain => PairChain::compare(c1, c2),
            Pattern::SoloChain => SoloChain::compare(c1, c2),
            Pattern::Trio => Trio::compare(c1, c2),
            Pattern::Pair => Pair::compare(c1, c2),
            Pattern::Solo => Card::compare(&c1[0], &c2[0]),
            _ => -1,
        }
    } else {
        -1
    }
}

pub fn compare_known_pattern(c1: &[Card], c2: &[Card], p: Pattern) -> i32 {
    match p {
        Pattern::Bomb => Bomb::compare(c1, c2),
        Pattern::Airplane => Airplane::compare(c1, c2),
        Pattern::PairChain => PairChain::compare(c1, c2),
        Pattern::SoloChain => SoloChain::compare(c1, c2),
        Pattern::Trio => Trio::compare(c1, c2),
        Pattern::Pair => Pair::compare(c1, c2),
        Pattern::Solo => Card::compare(&c1[0], &c2[0]),
        _ => -1,
    }
}
