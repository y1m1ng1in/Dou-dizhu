use wasm_bindgen_test::wasm_bindgen_test as test;

use doudizhu::cards::airplane::Airplane;
use doudizhu::cards::bomb::*;
use doudizhu::cards::card::Card;
use doudizhu::cards::card::Suit;
use doudizhu::cards::pair::Pair;
use doudizhu::cards::pairchain::PairChain;
use doudizhu::cards::solochain::SoloChain;
use doudizhu::cards::trio::Trio;
use doudizhu::player::computer::Strategy;

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

    let cards1 = vec![c1, c2, c3, c4, c5];
    let cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10];

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

    let cards1 = vec![c1, c2, c3, c4, c5, c6];
    let cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5],
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

    let cards1 = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];
    let cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5], &cards1[6],
            &cards1[7], &cards1[8], &cards1[9],
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

    let cards1 = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];
    let cards2 = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5], &cards1[6],
            &cards1[7], &cards1[8], &cards1[9],
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

    let cards1 = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];
    let cards2 = vec![
        d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15,
    ];

    {
        let v1 = vec![
            &cards1[0], &cards1[1], &cards1[2], &cards1[3], &cards1[4], &cards1[5], &cards1[6],
            &cards1[7], &cards1[8], &cards1[9],
        ];
        assert_eq!(true, Airplane::is_airplane(&v1));
        let x = Airplane::new(&v1);
        let r = Airplane::search_greater_cards(&cards2, &x).unwrap();
        assert_eq!(r, vec![8, 9, 10, 11, 12, 13, 0, 1, 3, 4]);
    }
}

#[test]
fn solochain_searching_longest_1() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(7_u32, Suit::Club, false);
    let d5 = Card::new(7_u32, Suit::Club, false);
    let d6 = Card::new(8_u32, Suit::Club, false);
    let d7 = Card::new(9_u32, Suit::Club, false);
    let d8 = Card::new(10_u32, Suit::Club, false);
    let d9 = Card::new(10_u32, Suit::Club, false);
    let d10 = Card::new(10_u32, Suit::Club, false);
    let d11 = Card::new(11_u32, Suit::Club, false);
    let d12 = Card::new(12_u32, Suit::Club, false);
    let d13 = Card::new(13_u32, Suit::Club, false);
    let d14 = Card::new(14_u32, Suit::Club, false);
    let d15 = Card::new(16_u32, Suit::Club, false);

    let cs = vec![
        d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15,
    ];

    {
        let x = SoloChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![3, 5, 6, 7, 10, 11, 12, 13]);
    }
}

#[test]
fn solochain_searching_longest_2() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(6_u32, Suit::Club, false);
    let d5 = Card::new(7_u32, Suit::Club, false);
    let d6 = Card::new(7_u32, Suit::Club, false);
    let d7 = Card::new(9_u32, Suit::Club, false);
    let d8 = Card::new(10_u32, Suit::Club, false);
    let d9 = Card::new(11_u32, Suit::Club, false);
    let d10 = Card::new(12_u32, Suit::Club, false);
    let d11 = Card::new(13_u32, Suit::Club, false);
    let d12 = Card::new(14_u32, Suit::Club, false);
    let d13 = Card::new(15_u32, Suit::Club, false);
    let d14 = Card::new(20_u32, Suit::Club, false);
    let d15 = Card::new(20_u32, Suit::Club, false);

    let cs = vec![
        d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15,
    ];

    {
        let x = SoloChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![6, 7, 8, 9, 10, 11, 12]);
    }
}

#[test]
fn solochain_searching_longest_3() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(6_u32, Suit::Club, false);
    let d5 = Card::new(7_u32, Suit::Club, false);
    let d6 = Card::new(7_u32, Suit::Club, false);
    let d7 = Card::new(9_u32, Suit::Club, false);
    let d8 = Card::new(10_u32, Suit::Club, false);
    let d9 = Card::new(11_u32, Suit::Club, false);
    let d10 = Card::new(12_u32, Suit::Club, false);
    let d11 = Card::new(13_u32, Suit::Club, false);
    let d12 = Card::new(14_u32, Suit::Club, false);
    let d13 = Card::new(15_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

    {
        let x = SoloChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![6, 7, 8, 9, 10, 11, 12]);
    }
}

#[test]
fn solochain_searching_longest_4() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(6_u32, Suit::Club, false);
    let d5 = Card::new(8_u32, Suit::Club, false);
    let d6 = Card::new(10_u32, Suit::Club, false);
    let d7 = Card::new(11_u32, Suit::Club, false);
    let d8 = Card::new(12_u32, Suit::Club, false);
    let d9 = Card::new(13_u32, Suit::Club, false);
    let d10 = Card::new(15_u32, Suit::Club, false);
    let d11 = Card::new(16_u32, Suit::Club, false);
    let d12 = Card::new(18_u32, Suit::Club, false);
    let d13 = Card::new(19_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

    {
        let x = SoloChain::search_longest_cards(&cs);
        assert_eq!(x, None);
    }
}

#[test]
fn solochain_searching_longest_5() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(6_u32, Suit::Club, false);
    let d5 = Card::new(8_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5];

    {
        let x = SoloChain::search_longest_cards(&cs);
        assert_eq!(x, None);
    }
}

#[test]
fn solochain_searching_longest_6() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(6_u32, Suit::Club, false);
    let d5 = Card::new(7_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5];

    {
        let x = SoloChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4]);
    }
}

#[test]
fn pairchain_searching_longest_1() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(4_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(5_u32, Suit::Club, false);
    let d6 = Card::new(5_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6];

    {
        let x = PairChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5]);
    }
}

#[test]
fn pairchain_searching_longest_2() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(4_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(5_u32, Suit::Club, false);
    let d6 = Card::new(5_u32, Suit::Club, false);
    let d7 = Card::new(6_u32, Suit::Club, false);
    let d8 = Card::new(6_u32, Suit::Club, false);
    let d9 = Card::new(7_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10];

    {
        let x = PairChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[test]
fn pairchain_searching_longest_3() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(4_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(6_u32, Suit::Club, false);
    let d6 = Card::new(6_u32, Suit::Club, false);
    let d7 = Card::new(7_u32, Suit::Club, false);
    let d8 = Card::new(7_u32, Suit::Club, false);
    let d9 = Card::new(8_u32, Suit::Club, false);
    let d10 = Card::new(8_u32, Suit::Club, false);
    let d11 = Card::new(8_u32, Suit::Club, false);
    let d12 = Card::new(9_u32, Suit::Club, false);
    let d13 = Card::new(9_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

    {
        let x = PairChain::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![4, 5, 6, 7, 8, 9, 11, 12]);
    }
}

#[test]
fn airplane_searching_longest_1() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(4_u32, Suit::Club, false);
    let d3 = Card::new(5_u32, Suit::Club, false);
    let d4 = Card::new(5_u32, Suit::Club, false);
    let d5 = Card::new(5_u32, Suit::Club, false);
    let d6 = Card::new(6_u32, Suit::Club, false);
    let d7 = Card::new(6_u32, Suit::Club, false);
    let d8 = Card::new(6_u32, Suit::Club, false);
    let d9 = Card::new(7_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);
    let d11 = Card::new(7_u32, Suit::Club, false);
    let d12 = Card::new(8_u32, Suit::Club, false);
    let d13 = Card::new(9_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

    {
        let x = Airplane::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 11]);
    }
}

#[test]
fn airplane_searching_longest_2() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(3_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(4_u32, Suit::Club, false);
    let d6 = Card::new(4_u32, Suit::Club, false);
    let d7 = Card::new(6_u32, Suit::Club, false);
    let d8 = Card::new(6_u32, Suit::Club, false);
    let d9 = Card::new(7_u32, Suit::Club, false);
    let d10 = Card::new(8_u32, Suit::Club, false);
    let d11 = Card::new(9_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11];

    {
        let x = Airplane::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }
}

#[test]
fn airplane_searching_longest_3() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(3_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(4_u32, Suit::Club, false);
    let d6 = Card::new(4_u32, Suit::Club, false);
    let d7 = Card::new(5_u32, Suit::Club, false);
    let d8 = Card::new(5_u32, Suit::Club, false);
    let d9 = Card::new(5_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);
    let d11 = Card::new(7_u32, Suit::Club, false);
    let d12 = Card::new(8_u32, Suit::Club, false);
    let d13 = Card::new(8_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

    {
        let x = Airplane::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    }
}

#[test]
fn airplane_searching_longest_4() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(3_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(4_u32, Suit::Club, false);
    let d6 = Card::new(4_u32, Suit::Club, false);
    let d7 = Card::new(5_u32, Suit::Club, false);
    let d8 = Card::new(5_u32, Suit::Club, false);
    let d9 = Card::new(5_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10];

    {
        let x = Airplane::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

#[test]
fn airplane_searching_longest_5() {
    // 333-444-7-8
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(3_u32, Suit::Club, false);
    let d4 = Card::new(4_u32, Suit::Club, false);
    let d5 = Card::new(4_u32, Suit::Club, false);
    let d6 = Card::new(4_u32, Suit::Club, false);
    let d7 = Card::new(5_u32, Suit::Club, false);
    let d8 = Card::new(5_u32, Suit::Club, false);
    let d9 = Card::new(5_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);
    let d11 = Card::new(8_u32, Suit::Club, false);

    let cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11];

    {
        let x = Airplane::search_longest_cards(&cs).unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 9, 10]);
    }
}

#[test]
fn bomb_split() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(3_u32, Suit::Club, false);
    let d4 = Card::new(3_u32, Suit::Club, false);
    let d5 = Card::new(4_u32, Suit::Club, false);
    let d6 = Card::new(4_u32, Suit::Club, false);
    let d7 = Card::new(5_u32, Suit::Club, false);
    let d8 = Card::new(5_u32, Suit::Club, false);
    let d9 = Card::new(5_u32, Suit::Club, false);
    let d10 = Card::new(7_u32, Suit::Club, false);
    let d11 = Card::new(8_u32, Suit::Club, false);

    let mut cs = vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11];

    let r1 = vec![
        Card::new(3_u32, Suit::Club, false),
        Card::new(3_u32, Suit::Club, false),
        Card::new(3_u32, Suit::Club, false),
        Card::new(3_u32, Suit::Club, false),
    ];

    let r2 = vec![
        Card::new(4_u32, Suit::Club, false),
        Card::new(4_u32, Suit::Club, false),
        Card::new(5_u32, Suit::Club, false),
        Card::new(5_u32, Suit::Club, false),
        Card::new(5_u32, Suit::Club, false),
        Card::new(7_u32, Suit::Club, false),
        Card::new(8_u32, Suit::Club, false),
    ];

    {
        let x = Bomb::split_from_cards(&mut cs);
        assert_eq!(x, r1);
        assert_eq!(cs.to_vec(), r2);
    }
}

#[test]
fn strategy_test_1() {
    let d1 = Card::new(3_u32, Suit::Club, false);
    let d2 = Card::new(3_u32, Suit::Club, false);
    let d3 = Card::new(3_u32, Suit::Club, false);
    let d4 = Card::new(3_u32, Suit::Club, false);
    let d5 = Card::new(4_u32, Suit::Club, false);
    let d6 = Card::new(4_u32, Suit::Club, false);
    let d7 = Card::new(5_u32, Suit::Club, false);
    let d8 = Card::new(5_u32, Suit::Club, false);
    let d9 = Card::new(6_u32, Suit::Club, false);
    let d10 = Card::new(6_u32, Suit::Club, false);
    let d11 = Card::new(8_u32, Suit::Club, false);

    let mut s = Strategy::new();
    
    let mut x = vec![d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11]; 

    s.construct(&mut x);

    assert_eq!(x, vec![Card::new(8_u32, Suit::Club, false),]);
}