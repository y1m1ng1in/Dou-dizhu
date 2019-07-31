use super::airplane::Airplane;
use super::bomb::*;
use super::card::Card;
use super::pair::Pair;
use super::pairchain::PairChain;
use super::solochain::SoloChain;
use super::trio::Trio;

#[derive(PartialEq)]
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

pub fn get_pattern(cards: &Vec<Card>) -> Pattern {
    if Airplane::is_airplane(&cards) {
        Pattern::Airplane
    } else if Bomb::is_bomb(&cards) || Rocket::is_rocket(&cards) {
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

pub fn compare(c1: &Vec<Card>, c2: &Vec<Card>) -> i32 {
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
