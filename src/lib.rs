#![recursion_limit = "256"]

pub mod cards;
pub mod player;

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    console: ConsoleService,
    dummy: u32,
}

pub enum Msg {
    Foo,
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
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Dou dizhu (tranditional Chinese card game) ---- A frontend project in Rust!" }</h1>
                <h3>{ "Not implemented yet!" }</h3>
                <button onclick=|_| Msg::Foo>{ "Look At Console" }</button>
                <p>{ self.dummy }</p>
            </div>
        }
    }
}
