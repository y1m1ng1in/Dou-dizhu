#![recursion_limit = "256"]

pub mod cards;
pub mod player;
pub mod cardui;
pub mod cardbufui;

use cards::card::Card;
use cards::card::Suit;
use cardbufui::CardBufUI;
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
                self.console.log(&(card.value.to_string() + " in PlayerCard"));
                self.player_cards.retain(|&c| c != card);
                self.player_buffer.push(card);
                self.player_buffer.sort();
            }
            Msg::PlayerBufferClicked(card) => {
                self.console.log(&(card.value.to_string() + " in PlayerBuffer"));
                self.player_buffer.retain(|&c| c != card);
                self.player_cards.push(card);
                self.player_cards.sort();
            }
            Msg::PlayerHandIn => {
                // figure out pattern in player buffer
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
                <CardBufUI cards=&self.player_buffer onsignal=Msg::PlayerBufferClicked />
                <CardBufUI cards=&self.player_cards onsignal=Msg::PlayerCardClicked />
                <button onclick=|_| Msg::PlayerHandIn>{ "Hand in" }</button>
                <button onclick=|_| Msg::PlayerPass>{ "Pass" }</button>
                <p>{ &self.player_message }</p>
            </div>
        }
    }
}

// connect to sqlite3 or hard-coded?
fn get_cards() -> Vec<Card> {
    let c1 = Card::new(3u32, Suit::Club, false);
    let c2 = Card::new(5u32, Suit::Spade, false);
    let c3 = Card::new(12u32, Suit::Diamond, false);
    let c4 = Card::new(14u32, Suit::Heart, false);
    let c = vec![c1, c2, c3, c4];

    c
}
