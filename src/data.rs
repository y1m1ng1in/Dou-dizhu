use super::cards::card::Card;
use super::cards::card::Suit;

pub fn get_cards(choose: u32) -> (Vec<Card>, Vec<Card>) {
    match choose {
        1 => get_cards_1(),
        2 => get_cards_2(),
        3 => get_cards_3(),
        4 => get_cards_4(),
        5 => get_cards_5(),
        6 => get_cards_6(),
        _ => (vec![], vec![]),
    }
}

fn get_cards_1() -> (Vec<Card>, Vec<Card>) {
    let comp1 = Card::new(3u32, Suit::Club, false);
    let comp2 = Card::new(5u32, Suit::Spade, false);
    let comp3 = Card::new(7u32, Suit::Spade, false);
    let comp4 = Card::new(7u32, Suit::Heart, false);
    let comp5 = Card::new(7u32, Suit::Club, false);
    let comp6 = Card::new(7u32, Suit::Diamond, false);
    let comp7 = Card::new(8u32, Suit::Spade, false);
    let comp8 = Card::new(8u32, Suit::Heart, false);
    let comp9 = Card::new(8u32, Suit::Diamond, false);
    let comp10 = Card::new(9u32, Suit::Spade, false);
    let c = vec![
        comp1, comp2, comp3, comp4, comp5, comp6, comp7, comp8, comp9, comp10,
    ];

    let user1 = Card::new(3u32, Suit::Club, false);
    let user2 = Card::new(5u32, Suit::Spade, false);
    let user3 = Card::new(9u32, Suit::Spade, false);
    let user4 = Card::new(9u32, Suit::Heart, false);
    let user5 = Card::new(9u32, Suit::Club, false);
    let user6 = Card::new(10u32, Suit::Diamond, false);
    let user7 = Card::new(10u32, Suit::Spade, false);
    let user8 = Card::new(10u32, Suit::Heart, false);
    let user9 = Card::new(13u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9,
    ];

    (u, c)
}

fn get_cards_2() -> (Vec<Card>, Vec<Card>) {
    let comp1 = Card::new(4u32, Suit::Club, false);
    let comp2 = Card::new(5u32, Suit::Spade, false);
    let comp3 = Card::new(6u32, Suit::Spade, false);
    let comp4 = Card::new(7u32, Suit::Heart, false);
    let comp5 = Card::new(8u32, Suit::Club, false);
    let comp6 = Card::new(12u32, Suit::Diamond, false);
    let comp7 = Card::new(12u32, Suit::Spade, false);
    let comp8 = Card::new(12u32, Suit::Heart, false);
    let comp9 = Card::new(14u32, Suit::Diamond, false);
    let comp10 = Card::new(14u32, Suit::Spade, false);
    let c = vec![
        comp1, comp2, comp3, comp4, comp5, comp6, comp7, comp8, comp9, comp10,
    ];

    let user1 = Card::new(3u32, Suit::Club, false);
    let user2 = Card::new(4u32, Suit::Spade, false);
    let user3 = Card::new(5u32, Suit::Spade, false);
    let user4 = Card::new(6u32, Suit::Heart, false);
    let user5 = Card::new(7u32, Suit::Club, false);
    let user6 = Card::new(8u32, Suit::Diamond, false);
    let user7 = Card::new(9u32, Suit::Spade, false);
    let user8 = Card::new(11u32, Suit::Heart, false);
    let user9 = Card::new(11u32, Suit::Diamond, false);
    let user10 = Card::new(15u32, Suit::Heart, false);
    let user11 = Card::new(15u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9, user10, user11,
    ];

    (u, c)
}

fn get_cards_3() -> (Vec<Card>, Vec<Card>) {
    let comp1 = Card::new(7u32, Suit::Club, false);
    let comp2 = Card::new(7u32, Suit::Spade, false);
    let comp3 = Card::new(8u32, Suit::Spade, false);
    let comp4 = Card::new(8u32, Suit::Heart, false);
    let comp5 = Card::new(9u32, Suit::Club, false);
    let comp6 = Card::new(9u32, Suit::Diamond, false);
    let comp7 = Card::new(10u32, Suit::Spade, false);
    let comp8 = Card::new(10u32, Suit::Heart, false);
    let comp9 = Card::new(14u32, Suit::Diamond, false);
    let c = vec![
        comp1, comp2, comp3, comp4, comp5, comp6, comp7, comp8, comp9,
    ];

    let user1 = Card::new(4u32, Suit::Club, false);
    let user2 = Card::new(4u32, Suit::Spade, false);
    let user3 = Card::new(5u32, Suit::Spade, false);
    let user4 = Card::new(5u32, Suit::Heart, false);
    let user5 = Card::new(6u32, Suit::Club, false);
    let user6 = Card::new(6u32, Suit::Diamond, false);
    let user7 = Card::new(11u32, Suit::Spade, false);
    let user8 = Card::new(11u32, Suit::Heart, false);
    let user9 = Card::new(11u32, Suit::Diamond, false);
    let user10 = Card::new(14u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9, user10,
    ];

    (u, c)
}

fn get_cards_4() -> (Vec<Card>, Vec<Card>) {
    let comp1 = Card::new(3u32, Suit::Club, false);
    let comp2 = Card::new(4u32, Suit::Spade, false);
    let comp3 = Card::new(8u32, Suit::Spade, false);
    let comp4 = Card::new(10u32, Suit::Heart, false);
    let comp5 = Card::new(12u32, Suit::Club, false);
    let comp6 = Card::new(13u32, Suit::Diamond, false);
    let comp7 = Card::new(13u32, Suit::Spade, false);
    let comp8 = Card::new(14u32, Suit::Heart, false);
    let comp9 = Card::new(14u32, Suit::Diamond, false);
    let comp10 = Card::new(15u32, Suit::Spade, false);
    let c = vec![
        comp1, comp2, comp3, comp4, comp5, comp6, comp7, comp8, comp9, comp10,
    ];

    let user1 = Card::new(3u32, Suit::Club, false);
    let user2 = Card::new(4u32, Suit::Spade, false);
    let user3 = Card::new(5u32, Suit::Spade, false);
    let user4 = Card::new(6u32, Suit::Heart, false);
    let user5 = Card::new(7u32, Suit::Club, false);
    let user6 = Card::new(7u32, Suit::Diamond, false);
    let user7 = Card::new(9u32, Suit::Spade, false);
    let user8 = Card::new(10u32, Suit::Heart, false);
    let user9 = Card::new(10u32, Suit::Diamond, false);
    let user10 = Card::new(11u32, Suit::Heart, false);
    let user11 = Card::new(13u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9, user10, user11,
    ];

    (u, c)
}

fn get_cards_5() -> (Vec<Card>, Vec<Card>) {
    let comp1 = Card::new(5u32, Suit::Club, false);
    let comp2 = Card::new(5u32, Suit::Spade, false);
    let comp3 = Card::new(5u32, Suit::Spade, false);
    let comp4 = Card::new(6u32, Suit::Heart, false);
    let comp5 = Card::new(6u32, Suit::Club, false);
    let comp6 = Card::new(6u32, Suit::Diamond, false);
    let comp7 = Card::new(8u32, Suit::Spade, false);
    let comp8 = Card::new(11u32, Suit::Heart, false);
    let comp9 = Card::new(12u32, Suit::Diamond, false);
    let comp10 = Card::new(13u32, Suit::Spade, false);
    let c = vec![
        comp1, comp2, comp3, comp4, comp5, comp6, comp7, comp8, comp9, comp10,
    ];

    let user1 = Card::new(3u32, Suit::Club, false);
    let user2 = Card::new(3u32, Suit::Spade, false);
    let user3 = Card::new(3u32, Suit::Spade, false);
    let user4 = Card::new(4u32, Suit::Heart, false);
    let user5 = Card::new(4u32, Suit::Club, false);
    let user6 = Card::new(4u32, Suit::Diamond, false);
    let user7 = Card::new(9u32, Suit::Spade, false);
    let user8 = Card::new(10u32, Suit::Heart, false);
    let user9 = Card::new(10u32, Suit::Diamond, false);
    let user10 = Card::new(14u32, Suit::Heart, false);
    let user11 = Card::new(15u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9, user10, user11,
    ];

    (u, c)
}

fn get_cards_6() -> (Vec<Card>, Vec<Card>) {
    let comp1 = Card::new(3u32, Suit::Club, false);
    let comp2 = Card::new(3u32, Suit::Spade, false);
    let comp3 = Card::new(3u32, Suit::Spade, false);
    let comp4 = Card::new(6u32, Suit::Heart, false);
    let comp5 = Card::new(7u32, Suit::Club, false);
    let comp6 = Card::new(8u32, Suit::Diamond, false);
    let comp7 = Card::new(9u32, Suit::Spade, false);
    let comp8 = Card::new(10u32, Suit::Heart, false);
    let comp9 = Card::new(14u32, Suit::Diamond, false);
    let comp10 = Card::new(14u32, Suit::Spade, false);
    let c = vec![
        comp1, comp2, comp3, comp4, comp5, comp6, comp7, comp8, comp9, comp10,
    ];

    let user1 = Card::new(3u32, Suit::Club, false);
    let user2 = Card::new(5u32, Suit::Diamond, false);
    let user3 = Card::new(5u32, Suit::Spade, false);
    let user4 = Card::new(5u32, Suit::Heart, false);
    let user5 = Card::new(5u32, Suit::Club, false);
    let user6 = Card::new(6u32, Suit::Diamond, false);
    let user7 = Card::new(6u32, Suit::Spade, false);
    let user8 = Card::new(9u32, Suit::Heart, false);
    let user9 = Card::new(10u32, Suit::Diamond, false);
    let user10 = Card::new(13u32, Suit::Heart, false);
    let user11 = Card::new(15u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9, user10, user11,
    ];

    (u, c)
}