#![recursion_limit = "256"]

pub mod cards;
pub mod player;
pub mod cardui;

use cards::card::Card;
use cards::card::Suit;
use cardui::CardUI;
use yew::services::ConsoleService;
use yew::prelude::*;

pub struct Model {
    console: ConsoleService,
    dummy: u32,
}

pub enum Msg {
    Foo,
    CardClicked(Card),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            dummy: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Foo => {
                self.console.log("A try to console service");
            },
            Msg::CardClicked(c) => {
                self.console.log(&c.value.to_string());
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let c1 = Card::new(3u32, Suit::Club, false);
        let c2 = Card::new(5u32, Suit::Spade, false);
        let c3 = Card::new(12u32, Suit::Diamond, false);
        let c4 = Card::new(14u32, Suit::Heart, false);

        html! {
            <div>
                <h1>{ "Dou dizhu (tranditional Chinese card game) ---- A frontend project in Rust!" }</h1>
                <h3>{ "Not implemented yet!" }</h3>
                <button onclick=|_| Msg::Foo>{ "Look At Console" }</button>

                <div>
                    <CardUI card=c1 onsignal=Msg::CardClicked />
                    <CardUI card=c2 onsignal=Msg::CardClicked />
                    <CardUI card=c3 onsignal=Msg::CardClicked />
                    <CardUI card=c4 onsignal=Msg::CardClicked />
                </div>
            </div>
        }
    }
}
