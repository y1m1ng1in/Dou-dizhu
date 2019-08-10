#![recursion_limit = "256"]

pub mod cardbufui;
pub mod cards;
pub mod cardui;
pub mod player;

use cardbufui::CardBufUI;
use cards::card::Card;
use cards::card::Suit;
use cards::utils;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    player_cards: Vec<Card>,
    player_buffer: Vec<Card>,
    player_message: String,
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
        let c = get_cards();

        Model {
            console: ConsoleService::new(),
            player_cards: c,
            player_buffer: vec![],
            player_message: String::new(),
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
                self.player_message = utils::get_pattern(&self.player_buffer).to_string();
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
fn get_cards() -> Vec<Card> {
    let c1 = Card::new(3u32, Suit::Club, false);
    let c2 = Card::new(5u32, Suit::Spade, false);
    let c3 = Card::new(7u32, Suit::Spade, false);
    let c4 = Card::new(7u32, Suit::Heart, false);
    let c5 = Card::new(7u32, Suit::Club, false);
    let c6 = Card::new(7u32, Suit::Diamond, false);
    let c7 = Card::new(8u32, Suit::Spade, false);
    let c8 = Card::new(8u32, Suit::Heart, false);
    let c9 = Card::new(8u32, Suit::Diamond, false);
    let c10 = Card::new(9u32, Suit::Spade, false);
    let c = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10];

    c
}
