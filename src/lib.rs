#![recursion_limit = "256"]

pub mod cardbufui;
pub mod cards;
pub mod cardui;
pub mod player;

use cardbufui::CardBufUI;
use cards::card::Card;
use cards::card::Suit;
use cards::utils;
use player::computer::ComputerPlayer;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    player_cards: Vec<Card>,
    player_buffer: Vec<Card>,
    player_message: String,
    computer_cards: Vec<Card>,
    computer_buffer: Vec<Card>,
    computer_strategy: ComputerPlayer,
}

pub enum Msg {
    PlayerCardClicked(Card),
    PlayerBufferClicked(Card),
    PlayerHandIn,
    PlayerPass,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let (user, computer) = get_cards();
        let strategy = ComputerPlayer::new(computer);
        let computer_card = strategy.display();

        Model {
            console: ConsoleService::new(),
            player_cards: user,
            player_buffer: vec![],
            player_message: String::new(),
            computer_cards: computer_card,
            computer_buffer: vec![],
            computer_strategy: strategy,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlayerCardClicked(card) => {
                self.console
                    .log(&(card.value.to_string() + " in PlayerCard"));
                self.player_cards.retain(|&c| c != card);
                self.player_buffer.push(card);
                self.player_buffer.sort();
            }
            Msg::PlayerBufferClicked(card) => {
                self.console
                    .log(&(card.value.to_string() + " in PlayerBuffer"));
                self.player_buffer.retain(|&c| c != card);
                self.player_cards.push(card);
                self.player_cards.sort();
            }
            Msg::PlayerHandIn => {
                let pattern = utils::get_pattern(&self.player_buffer);

                if pattern != utils::Pattern::Invalid {
                    self.computer_buffer = self
                        .computer_strategy
                        .hand_in_follow(&self.player_buffer, pattern);
                    self.computer_cards = self.computer_strategy.display();
                } else {
                    self.player_cards.append(&mut self.player_buffer);
                    self.player_cards.sort();
                }
                self.player_message = pattern.to_string();
                self.console.log(&self.computer_strategy.to_string());
            }
            Msg::PlayerPass => {
                self.player_cards.append(&mut self.player_buffer);
                self.player_cards.sort();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div class="computer-container">
                    <CardBufUI cards=&self.computer_cards />
                    <CardBufUI cards=&self.computer_buffer />
                </div>
                <div class="user-container">
                    <CardBufUI cards=&self.player_buffer onsignal=Msg::PlayerBufferClicked />
                    <CardBufUI cards=&self.player_cards onsignal=Msg::PlayerCardClicked />
                    <div class="user-button-container">
                        <button onclick=|_| Msg::PlayerHandIn>{ "Hand in" }</button>
                        <button onclick=|_| Msg::PlayerPass>{ "Pass" }</button>
                    </div>
                    <p>{ &self.player_message }</p>
                </div>
            </div>
        }
    }
}

// connect to sqlite3 or hard-coded?
fn get_cards() -> (Vec<Card>, Vec<Card>) {
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
    let user3 = Card::new(7u32, Suit::Spade, false);
    let user4 = Card::new(7u32, Suit::Heart, false);
    let user5 = Card::new(7u32, Suit::Club, false);
    let user6 = Card::new(7u32, Suit::Diamond, false);
    let user7 = Card::new(8u32, Suit::Spade, false);
    let user8 = Card::new(8u32, Suit::Heart, false);
    let user9 = Card::new(8u32, Suit::Diamond, false);
    let u = vec![
        user1, user2, user3, user4, user5, user6, user7, user8, user9,
    ];

    (u, c)
}
