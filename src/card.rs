// - Store indice in struct?
// 3 with 1   3 with 2
// - sort card vector
// - pattern search from card vector

pub struct Card {
    value: u32,
    suit: Suit,
    selected: bool,
}

pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
    Joker,
}

//pub struct Solo<'a> {
//    pub card: &'a Card,
//}

pub type Solo<'a> = &'a Card;

pub struct Pair<'a> {
    card1: &'a Card,
    card2: &'a Card,
}

pub struct Trio<'a> {
    card1: &'a Card,
    card2: &'a Card,
    card3: &'a Card,
}

pub struct SoloChain<'a> {
    chain: Vec<Solo<'a>>,
}

pub struct PairChain<'a> {
    chain: Vec<Pair<'a>>,
}

pub struct Airplane<'a> {
    chain: Vec<&'a Card>,
    kicker: Vec<&'a Card>,
    size: usize,
}

pub struct Bomb<'a> {
    card1: &'a Card,
    card2: &'a Card,
    card3: &'a Card,
    card4: &'a Card,
}

pub struct Rocket<'a> {
    joker1: &'a Card,
    joker2: &'a Card,
}

impl Card {
    pub fn new(val: u32, s: Suit, sel: bool) -> Card {
        Card {
            value: val,
            suit: s,
            selected: sel,
        }
    }

    pub fn display(&self) {
        println! {"{}", self.value};
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Card) -> Option<usize> {
        let val = greater_than.value;

        for i in 0..cards.len() {
            if cards[i].value > val {
                return Some(i);
            }
        }

        None
    }
}

impl<'a> Pair<'a> {
    pub fn new(c1: &'a Card, c2: &'a Card) -> Pair<'a> {
        Pair {
            card1: c1,
            card2: c2,
        }
    }

    pub fn is_pair(cards: &Vec<&Card>) -> bool {
        if cards.len() != 2 {
            return false;
        }
        if cards[0].value != cards[1].value {
            false
        } else {
            true
        }
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Pair) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.card1.value;

        while i + 1 < cards.len() {
            if Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
                if cards[i].value > val {
                    return Some(vec![i, i + 1]);
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        None
    }
}

impl<'a> Trio<'a> {
    pub fn new(c1: &'a Card, c2: &'a Card, c3: &'a Card) -> Trio<'a> {
        Trio {
            card1: c1,
            card2: c2,
            card3: c3,
        }
    }

    pub fn is_trio(cards: &Vec<&Card>) -> bool {
        if cards.len() != 3 {
            return false;
        }
        if cards[0].value == cards[1].value && cards[1].value == cards[2].value {
            true
        } else {
            false
        }
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Trio) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.card1.value;

        while i + 2 < cards.len() {
            if Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]]) {
                if cards[i].value > val {
                    return Some(vec![i, i + 1, i + 2]);
                }
                i += 3;
            } else {
                i += 1;
            }
        }

        None
    }
}

impl<'a> SoloChain<'a> {
    pub fn new(cards: Vec<&Card>) -> SoloChain {
        SoloChain { chain: cards }
    }

    // cards' value has already sorted in ascending
    pub fn is_solo_chain(cards: &Vec<&Card>) -> bool {
        let mut previous: u32;

        if cards.len() < 5 {
            return false;
        }
        previous = cards[0].value;
        for card in &cards[1..] {
            if card.value - previous != 1 {
                return false;
            } else {
                previous = card.value;
            }
        }

        true
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &SoloChain) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.chain[0].value;
        let size = greater_than.chain.len();

        while i < cards.len() {
            if cards[i].value <= val {
                i += 1;
            } else {
                break;
            }
        }

        SoloChain::search_from_pos(cards, size, i)
    }

    fn search_from_pos(cards: &Vec<Card>, size: usize, start_from: usize) -> Option<Vec<usize>> {
        let mut i = start_from;
        let mut previous = 0; // the card value can never be 0
        let mut current_size = 0;
        let mut result = Vec::new();

        if start_from >= cards.len() || size + start_from > cards.len() {
            return None;
        }

        while i < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if cards[i].value == previous + 1 {
                result.push(i);
                previous = cards[i].value;
                i += 1;
                current_size += 1;
                if current_size == size {
                    return Some(result);
                }
            } else {
                result = Vec::new();
                result.push(i);
                previous = cards[i].value;
                i += 1;
                current_size = 1;
            }
        }

        if current_size == size {
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> PairChain<'a> {
    // assume cards pass the pattern matching
    pub fn new(cards: Vec<&Card>) -> PairChain {
        let mut result = Vec::new();

        for i in (0..cards.len()).step_by(2) {
            result.push(Pair::new(cards[i], cards[i + 1]));
        }

        PairChain { chain: result }
    }

    // cards' value has already sorted in ascending
    pub fn is_pair_chain(cards: &Vec<&Card>) -> bool {
        let mut previous: u32;

        if cards.len() < 6 || cards.len() % 2 != 0 {
            return false;
        }

        if Pair::is_pair(&vec![cards[0], cards[1]]) {
            previous = cards[0].value;
        } else {
            return false;
        }

        for i in (2..cards.len()).step_by(2) {
            if Pair::is_pair(&vec![cards[i], cards[i + 1]]) {
                if cards[i].value - previous != 1 {
                    return false;
                } else {
                    previous = cards[i].value;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &PairChain) -> Option<Vec<usize>> {
        let mut i: usize = 0;
        let val = greater_than.chain[0].card1.value;
        let size = greater_than.chain.len();

        while i < cards.len() {
            if cards[i].value <= val {
                i += 1;
            } else {
                break;
            }
        }

        PairChain::search_from_pos(cards, size, i)
    }

    fn search_from_pos(cards: &Vec<Card>, size: usize, start_from: usize) -> Option<Vec<usize>> {
        let mut i = start_from;
        let mut previous: u32 = 0;
        let mut current_size = 0; // the card value can never be 0
        let mut result = Vec::new();

        if start_from >= cards.len() || size * 2 + start_from > cards.len() {
            return None;
        }

        while i + 1 < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if Pair::is_pair(&vec![&cards[i], &cards[i + 1]])
                && cards[i].value == previous + 1
            {
                previous = cards[i].value;
                current_size += 1;
                result.push(i);
                result.push(i + 1);
                i += 2;
                if current_size == size {
                    return Some(result);
                }
            } else {
                result = Vec::new();
                while i + 1 < cards.len() {
                    if !Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
                        i += 1;
                    } else {
                        previous = cards[i].value;
                        current_size = 1;   // change += to = 
                        result.push(i);
                        result.push(i + 1);
                        i += 2;
                        break;
                    }
                }
            }
        }

        if current_size == size {
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> Airplane<'a> {
    pub fn new(cards: &Vec<&'a Card>) -> Airplane<'a> {
        let airplane_size = Airplane::find_size(cards);
        let start = airplane_size.0;
        let num = airplane_size.1;
        let mut trios: Vec<&Card> = Vec::new();
        let mut kickers: Vec<&Card> = Vec::new();

        for i in start..start + 3 * num {
            trios.push(cards[i]);
        }

        for i in 0..start {
            kickers.push(cards[i]);
        }
        for i in start + 3 * num..cards.len() {
            kickers.push(cards[i]);
        }

        Airplane {
            chain: trios,
            kicker: kickers,
            size: num,
        }
    }

    // cards' value has already sorted in ascending
    pub fn is_airplane(cards: &Vec<&Card>) -> bool {
        let airplane_size = Airplane::find_size(cards);

        if airplane_size.1 == 0 {
            return false;
        } else {
            return true;
        }
    }

    fn find_size(cards: &Vec<&'a Card>) -> (usize, usize) {
        let mut i: usize = 0;
        let mut previous = 0;
        let mut first_trio = true;
        let mut trio_start = 0;
        let mut max_size = 0;
        let mut max_size_trio_start = 0;
        let mut trio_nums = 0;

        if cards.len() < 6 {
            return (0, 0);
        }

        while i + 2 < cards.len() {
            if Trio::is_trio(&vec![cards[i], cards[i + 1], cards[i + 2]]) {
                if first_trio {
                    trio_start = i;
                    previous = cards[i].value;
                    first_trio = false;
                    trio_nums += 1;
                } else {
                    if cards[i].value - previous == 1 {
                        trio_nums += 1;
                        previous = cards[i].value;
                    } else {
                        if trio_nums > max_size {
                            max_size = trio_nums;
                            max_size_trio_start = trio_start;
                        }
                        trio_start = i;
                        previous = cards[i].value;
                        trio_nums = 1;
                    }
                }
                i += 3;
            } else {
                i += 1;
                if trio_nums > max_size {
                    max_size = trio_nums;
                    max_size_trio_start = trio_start;
                    trio_nums = 0;
                    first_trio = true;
                }
            }
        }

        if trio_nums > max_size {
            max_size = trio_nums;
            max_size_trio_start = trio_start;
        }

        if max_size == 0 {
            (0, 0)
        } else {
            (max_size_trio_start, max_size)
        }
    }

    pub fn search_greater_cards(cards: &Vec<Card>, greater_than: &Airplane) -> Option<Vec<usize>> {
        let pair_kicker: bool;
        let val = greater_than.chain[0].value;
        let mut i: usize = 0;

        if greater_than.kicker.len() == greater_than.size * 2 {
            pair_kicker = true;
        } else {
            pair_kicker = false;
        }

        while i < cards.len() {
            if cards[i].value <= val {
                i += 1;
            } else {
                break;
            }
        }

        Airplane::search_from_pos(cards, greater_than.size, i, pair_kicker)
    }

    fn search_from_pos(
        cards: &Vec<Card>,
        size: usize,
        start_from: usize,
        pair_kicker: bool,
    ) -> Option<Vec<usize>> {
        let mut i: usize = start_from;
        let mut previous = 0;
        let mut current_size = 0;
        let mut result = Vec::new(); 

        if start_from >= cards.len() || start_from + 3 * size >= cards.len() {
            return None;
        }

        while i + 2 < cards.len() {
            if cards[i].value == previous {
                i += 1;
            } else if Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]])
                && cards[i].value == previous + 1 
            {
                previous = cards[i].value;
                current_size += 1;

                result.push(i);
                result.push(i + 1);
                result.push(i + 2);

                if current_size == size {
                    break;
                } else {
                    i += 3;
                }
            } else {
                result = Vec::new();
                while i + 2 < cards.len() {
                    if !Trio::is_trio(&vec![&cards[i], &cards[i + 1], &cards[i + 2]]) {
                        i += 1;
                    } else {
                        previous = cards[i].value;
                        current_size = 1;

                        result.push(i);
                        result.push(i + 1);
                        result.push(i + 2);

                        i += 3;
                        break;
                    }
                }
            }
        }

        if current_size != size {
            return None;
        } 
            
        Airplane::search_kickers(cards, &mut result, size, pair_kicker)
    }

    fn search_kickers(cards: &Vec<Card>, trio_indices: &mut Vec<usize>, size: usize, pair_kicker: bool) -> Option<Vec<usize>> {
        let mut result = Vec::new();
        let mut i: usize = 0;
        let mut current_size = 0;

        if !pair_kicker {
            while i < trio_indices.len() {
                if !trio_indices.contains(&i) {
                    result.push(i);
                    current_size += 1;
                    i += 1;
                } else {
                    i += 1;
                }
                if current_size == size {
                    break;
                }
            }
        } else {
            while i + 1 < trio_indices.len() {
                if Pair::is_pair(&vec![&cards[i], &cards[i + 1]]) {
                    if !trio_indices.contains(&i) && !trio_indices.contains(&(i + 1)) {
                        result.push(i);
                        result.push(i + 1);
                        current_size += 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                    if current_size == size {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        }

        if current_size == size {
            for j in 0..result.len() {
                trio_indices.push(result[j]);
            }
            Some(trio_indices.to_vec())
        } else {
            None
        }
    }
}

impl<'a> Bomb<'a> {
    pub fn new(cards: &Vec<&'a Card>) -> Bomb<'a> {
        Bomb {
            card1: cards[0],
            card2: cards[1],
            card3: cards[2],
            card4: cards[3],
        }
    }

    pub fn is_bomb(cards: &Vec<&'a Card>) -> bool {
        let val;

        if cards.len() != 4 {
            return false;
        }

        val = cards[0].value;
        for card in &cards[1..] {
            if card.value != val {
                return false;
            }
        }

        true
    }
}

impl<'a> Rocket<'a> {
    pub fn is_rocket(cards: &Vec<&Card>) -> bool {
        if cards.len() != 2 {
            return false;
        }

        let type1 = match cards[0].suit {
            Suit::Joker => true,
            _ => false,
        };

        let type2 = match cards[1].suit {
            Suit::Joker => true,
            _ => false,
        };

        if type1 && type2 {
            true
        } else {
            false
        }
    }
}

#[test]
fn test_pattern_check() {
    let c1 = Card::new(3_u32, Suit::Club, false);
    let c2 = Card::new(3_u32, Suit::Club, false);
    let c3 = Card::new(4_u32, Suit::Club, false);
    let c4 = Card::new(4_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(5_u32, Suit::Club, false);
    let c7 = Card::new(6_u32, Suit::Club, false);
    let c8 = Card::new(6_u32, Suit::Club, false);
    let c9 = Card::new(7_u32, Suit::Club, false);
    let c10 = Card::new(7_u32, Suit::Club, false);

    let mut cards = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];

    {
        let s1 = vec![
            &cards[0], &cards[1], &cards[2], &cards[3], &cards[4], &cards[5], &cards[6], &cards[7],
            &cards[8], &cards[9],
        ];

        assert_eq!(true, PairChain::is_pair_chain(&s1));
    }

    {
        let s2 = vec![&cards[0], &cards[1]];

        assert_eq!(true, Pair::is_pair(&s2));
    }

    {
        let s3 = vec![
            &cards[2], &cards[3], &cards[4], &cards[5], &cards[6], &cards[7],
        ];

        assert_eq!(true, PairChain::is_pair_chain(&s3));
        assert_eq!(false, Pair::is_pair(&s3));
        assert_eq!(false, SoloChain::is_solo_chain(&s3));
        assert_eq!(false, Airplane::is_airplane(&s3));
    }

    let c11 = Card::new(4_u32, Suit::Club, false);
    let c12 = Card::new(4_u32, Suit::Club, false);
    let c13 = Card::new(4_u32, Suit::Club, false);
    let c14 = Card::new(4_u32, Suit::Club, false);
    let c15 = Card::new(5_u32, Suit::Club, false);
    let c16 = Card::new(5_u32, Suit::Club, false);
    let c17 = Card::new(5_u32, Suit::Club, false);
    let c18 = Card::new(9_u32, Suit::Club, false);
    let c19 = Card::new(10_u32, Suit::Club, false);
    let c20 = Card::new(10_u32, Suit::Club, false);

    cards = vec![c11, c12, c13, c14, c15, c16, c17, c18, c19, c20];

    {
        let s1 = vec![
            &cards[1], &cards[2], &cards[3], &cards[4], &cards[5], &cards[6], &cards[7], &cards[8],
        ];

        assert_eq!(false, PairChain::is_pair_chain(&s1));
        assert_eq!(true, Airplane::is_airplane(&s1));
        assert_eq!(false, Bomb::is_bomb(&s1));
    }

    {
        let s2 = vec![&cards[0], &cards[1], &cards[2], &cards[3]];

        assert_eq!(true, Bomb::is_bomb(&s2));
        assert_eq!(false, Airplane::is_airplane(&s2));
        assert_eq!(false, Trio::is_trio(&s2));
        assert_eq!(false, Pair::is_pair(&s2));
        assert_eq!(false, PairChain::is_pair_chain(&s2));
        assert_eq!(false, SoloChain::is_solo_chain(&s2));
    }

    let c21 = Card::new(3_u32, Suit::Club, false);
    let c22 = Card::new(4_u32, Suit::Club, false);
    let c23 = Card::new(5_u32, Suit::Club, false);
    let c24 = Card::new(6_u32, Suit::Club, false);
    let c25 = Card::new(7_u32, Suit::Club, false);
    let c26 = Card::new(8_u32, Suit::Club, false);
    let c27 = Card::new(9_u32, Suit::Club, false);
    let c28 = Card::new(10_u32, Suit::Club, false);
    let c29 = Card::new(11_u32, Suit::Club, false);
    let c30 = Card::new(12_u32, Suit::Club, false);

    cards = vec![c21, c22, c23, c24, c25, c26, c27, c28, c29, c30];

    {
        let s3 = vec![
            &cards[0], &cards[1], &cards[2], &cards[3], &cards[4], &cards[5], &cards[6], &cards[7],
            &cards[8], &cards[9],
        ];

        assert_eq!(true, SoloChain::is_solo_chain(&s3));
        assert_eq!(false, PairChain::is_pair_chain(&s3));
        assert_eq!(false, Airplane::is_airplane(&s3));
    }
}

#[test]
fn test_pattern_searching_greater_than() {
    let c1 = Card::new(3_u32, Suit::Club, false);
    let c2 = Card::new(3_u32, Suit::Club, false);
    let c3 = Card::new(4_u32, Suit::Club, false);
    let c4 = Card::new(4_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(5_u32, Suit::Club, false);
    let c7 = Card::new(6_u32, Suit::Club, false);
    let c8 = Card::new(6_u32, Suit::Club, false);
    let c9 = Card::new(7_u32, Suit::Club, false);
    let c10 = Card::new(7_u32, Suit::Club, false);

    let mut cards = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];

    {
        let mut v = vec![&cards[0], &cards[1]];
        let mut p = Pair::new(v[0], v[1]);
        let mut r = Pair::search_greater_cards(&cards, &p).unwrap();
        assert_eq!(r, vec![2, 3]);

        v = vec![&cards[4], &cards[5]];
        p = Pair::new(v[0], v[1]);
        r = Pair::search_greater_cards(&cards, &p).unwrap();
        assert_eq!(r, vec![6, 7]);
    }

    let c1 = Card::new(3_u32, Suit::Club, false);
    let c2 = Card::new(3_u32, Suit::Club, false);
    let c3 = Card::new(3_u32, Suit::Club, false);
    let c4 = Card::new(4_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(6_u32, Suit::Club, false);
    let c7 = Card::new(7_u32, Suit::Club, false);
    let c8 = Card::new(8_u32, Suit::Club, false);
    let c9 = Card::new(9_u32, Suit::Club, false);
    let c10 = Card::new(9_u32, Suit::Club, false);

    cards = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];

    {
        let v = vec![&cards[0], &cards[1]];
        let p = Pair::new(v[0], v[1]);
        let r = Pair::search_greater_cards(&cards, &p).unwrap();
        assert_eq!(r, vec![8, 9]);
    }

    let c1 = Card::new(3_u32, Suit::Club, false);
    let c2 = Card::new(3_u32, Suit::Club, false);
    let c3 = Card::new(3_u32, Suit::Club, false);
    let c4 = Card::new(4_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(6_u32, Suit::Club, false);
    let c7 = Card::new(6_u32, Suit::Club, false);
    let c8 = Card::new(6_u32, Suit::Club, false);
    let c9 = Card::new(7_u32, Suit::Club, false);
    let c10 = Card::new(7_u32, Suit::Club, false);

    cards = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];

    {
        let v = vec![&cards[0], &cards[1], &cards[2]];
        let t = Trio::new(v[0], v[1], v[2]);
        let r = Trio::search_greater_cards(&cards, &t).unwrap();
        assert_eq!(r, vec![5, 6, 7]);
    }
}

#[test]
fn solochain_searching_greater_than() {
    let c1 = Card::new(3_u32, Suit::Club, false);
    let c2 = Card::new(4_u32, Suit::Club, false);
    let c3 = Card::new(5_u32, Suit::Club, false);
    let c4 = Card::new(6_u32, Suit::Club, false);
    let c5 = Card::new(7_u32, Suit::Club, false);

    let d1 = Card::new(4_u32, Suit::Club, false);
    let d2 = Card::new(5_u32, Suit::Club, false);
    let d3 = Card::new(6_u32, Suit::Club, false);
    let d4 = Card::new(8_u32, Suit::Club, false);
    let d5 = Card::new(8_u32, Suit::Club, false);
    let d6 = Card::new(9_u32, Suit::Club, false);
    let d7 = Card::new(10_u32, Suit::Club, false);
    let d8 = Card::new(11_u32, Suit::Club, false);
    let d9 = Card::new(12_u32, Suit::Club, false);
    let d10 = Card::new(13_u32, Suit::Club, false);

    let mut cards1 = vec![c1, c2, c3, c4, c5];
    let mut cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10];

    {
        let v = vec![&cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4]];
        assert_eq!(true, SoloChain::is_solo_chain(&v));
        let sc = SoloChain::new(v);
        let r = SoloChain::search_greater_cards(&cards2, &sc).unwrap();
        assert_eq!(r, vec![3, 5, 6, 7, 8]);

        let v2 = vec![&cards2[3], &cards2[5], &cards2[6], &cards2[7], &cards2[8]];
        assert_eq!(true, SoloChain::is_solo_chain(&v2));
        let sc2 = SoloChain::new(v2);
        let r2 = SoloChain::search_greater_cards(&cards2, &sc2).unwrap();
        assert_eq!(r2, vec![5, 6, 7, 8, 9]);

        let v3 = vec![&cards2[5], &cards2[6], &cards2[7], &cards2[8], &cards2[9]];
        assert_eq!(true, SoloChain::is_solo_chain(&v3));
        let sc3 = SoloChain::new(v3);
        let r3 = SoloChain::search_greater_cards(&cards2, &sc3);
        assert_eq!(r3, None);
    }
}

#[test]
fn pairchain_searching_greater_than() {
    let c1 = Card::new(3_u32, Suit::Club, false);
    let c2 = Card::new(3_u32, Suit::Club, false);
    let c3 = Card::new(4_u32, Suit::Club, false);
    let c4 = Card::new(4_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(5_u32, Suit::Club, false);

    let d1 = Card::new(4_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(5_u32, Suit::Club, false);
    let d5 = Card::new(5_u32, Suit::Club, false);
    let d6 = Card::new(5_u32, Suit::Club, false);
    let d7 = Card::new(6_u32, Suit::Club, false);
    let d8 = Card::new(6_u32, Suit::Club, false);
    let d9 = Card::new(6_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);
    let d11 = Card::new(7_u32, Suit::Club, false);

    let mut cards1 = vec![c1, c2, c3, c4, c5, c6];
    let mut cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5],
        ];
        let v2 = vec![
            &cards2[0],
            &cards2[1],
            &cards2[2],
            &cards2[3],
            &cards2[4],
            &cards2[5],
            &cards2[6],
            &cards2[7],
            &cards2[8],
            &cards2[9],
            &cards2[10],
        ];
        let x = PairChain::new(v1);
        let r = PairChain::search_greater_cards(&cards2, &x).unwrap();
        assert_eq!(r, vec![0, 1, 2, 3, 6, 7]);
    }
}

#[test]
fn airplane_searching_greater_than_1() {
    // 444555-7788 < 999-10-10-10-4455

    let c1 = Card::new(4_u32, Suit::Club, false);
    let c2 = Card::new(4_u32, Suit::Club, false);
    let c3 = Card::new(4_u32, Suit::Club, false);
    let c4 = Card::new(5_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(5_u32, Suit::Club, false);
    let c7 = Card::new(7_u32, Suit::Club, false);
    let c8 = Card::new(7_u32, Suit::Club, false);
    let c9 = Card::new(8_u32, Suit::Club, false);
    let c10 = Card::new(8_u32, Suit::Club, false); 

    let d1 = Card::new(4_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(5_u32, Suit::Club, false);
    let d5 = Card::new(9_u32, Suit::Club, false);
    let d6 = Card::new(9_u32, Suit::Club, false);
    let d7 = Card::new(9_u32, Suit::Club, false);
    let d8 = Card::new(10_u32, Suit::Club, false);
    let d9 = Card::new(10_u32, Suit::Club, false);
    let d10 = Card::new(10_u32, Suit::Club, false);
    let d11 = Card::new(11_u32, Suit::Club, false);
    let d12 = Card::new(12_u32, Suit::Club, false);

    let mut cards1 = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];
    let mut cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5],
            &cards1[6], &cards1[7], &cards1[8], &cards1[9],
        ];
        let v2 = vec![
            &cards2[0],
            &cards2[1],
            &cards2[2],
            &cards2[3],
            &cards2[4],
            &cards2[5],
            &cards2[6],
            &cards2[7],
            &cards2[8],
            &cards2[9],
            &cards2[10],
            &cards2[11],
        ];
        assert_eq!(true, Airplane::is_airplane(&v1));
        let x = Airplane::new(&v1);
        let r = Airplane::search_greater_cards(&cards2, &x).unwrap();
        assert_eq!(r, vec![4, 5, 6, 7, 8, 9, 0, 1, 2, 3]);
    }

}

#[test]
fn airplane_searching_greater_than_2() {
    // 444555-7788 < 999-10-10-10-5566

    let c1 = Card::new(4_u32, Suit::Club, false);
    let c2 = Card::new(4_u32, Suit::Club, false);
    let c3 = Card::new(4_u32, Suit::Club, false);
    let c4 = Card::new(5_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(5_u32, Suit::Club, false);
    let c7 = Card::new(7_u32, Suit::Club, false);
    let c8 = Card::new(7_u32, Suit::Club, false);
    let c9 = Card::new(8_u32, Suit::Club, false);
    let c10 = Card::new(8_u32, Suit::Club, false); 

    let d1 = Card::new(5_u32, Suit::Club, false);
    let d2 = Card::new(5_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(6_u32, Suit::Club, false);
    let d5 = Card::new(6_u32, Suit::Club, false);
    let d6 = Card::new(9_u32, Suit::Club, false);
    let d7 = Card::new(9_u32, Suit::Club, false);
    let d8 = Card::new(9_u32, Suit::Club, false);
    let d9 = Card::new(10_u32, Suit::Club, false);
    let d10 = Card::new(10_u32, Suit::Club, false);
    let d11 = Card::new(10_u32, Suit::Club, false);
    let d12 = Card::new(12_u32, Suit::Club, false);

    let mut cards1 = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];
    let mut cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5],
            &cards1[6], &cards1[7], &cards1[8], &cards1[9],
        ];
        let v2 = vec![
            &cards2[0],
            &cards2[1],
            &cards2[2],
            &cards2[3],
            &cards2[4],
            &cards2[5],
            &cards2[6],
            &cards2[7],
            &cards2[8],
            &cards2[9],
            &cards2[10],
            &cards2[11],
        ];
        assert_eq!(true, Airplane::is_airplane(&v1));
        let x = Airplane::new(&v1);
        let r = Airplane::search_greater_cards(&cards2, &x).unwrap();
        assert_eq!(r, vec![5, 6, 7, 8, 9, 10, 0, 1, 3, 4]);
    }
}

#[test]
fn airplane_searching_greater_than_3() {
    // 444555-7788 < 10-10-10-JJJ-5577

    let c1 = Card::new(4_u32, Suit::Club, false);
    let c2 = Card::new(4_u32, Suit::Club, false);
    let c3 = Card::new(4_u32, Suit::Club, false);
    let c4 = Card::new(5_u32, Suit::Club, false);
    let c5 = Card::new(5_u32, Suit::Club, false);
    let c6 = Card::new(5_u32, Suit::Club, false);
    let c7 = Card::new(7_u32, Suit::Club, false);
    let c8 = Card::new(7_u32, Suit::Club, false);
    let c9 = Card::new(8_u32, Suit::Club, false);
    let c10 = Card::new(8_u32, Suit::Club, false); 

    let d1 = Card::new(5_u32, Suit::Club, false);
    let d2 = Card::new(5_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(7u32, Suit::Club, false);
    let d5 = Card::new(7_u32, Suit::Club, false);
    let d6 = Card::new(7_u32, Suit::Club, false);
    let d7 = Card::new(9_u32, Suit::Club, false);
    let d8 = Card::new(9_u32, Suit::Club, false);
    let d9 = Card::new(10_u32, Suit::Club, false);
    let d10 = Card::new(10_u32, Suit::Club, false);
    let d11 = Card::new(10_u32, Suit::Club, false);
    let d12 = Card::new(11_u32, Suit::Club, false);
    let d13 = Card::new(11_u32, Suit::Club, false);
    let d14 = Card::new(11_u32, Suit::Club, false);
    let d15 = Card::new(12_u32, Suit::Club, false);

    let mut cards1 = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];
    let mut cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5],
            &cards1[6], &cards1[7], &cards1[8], &cards1[9],
        ];
        let v2 = vec![
            &cards2[0],
            &cards2[1],
            &cards2[2],
            &cards2[3],
            &cards2[4],
            &cards2[5],
            &cards2[6],
            &cards2[7],
            &cards2[8],
            &cards2[9],
            &cards2[10],
            &cards2[11],
            &cards2[12],
            &cards2[13],
            &cards2[14],
        ];
        assert_eq!(true, Airplane::is_airplane(&v1));
        let x = Airplane::new(&v1);
        let r = Airplane::search_greater_cards(&cards2, &x).unwrap();
        assert_eq!(r, vec![8, 9, 10, 11, 12, 13, 0, 1, 3, 4]);
    }
}