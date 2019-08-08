use super::cards::card::Card;
use super::cards::card::Suit;
use crate::cardui::CardUI;
use yew::prelude::*;

pub struct CardBufUI {
    cards: Vec<Card>,
    onsignal: Option<Callback<()>>, // Callback<Vec<Card>> ?
}

pub enum Msg {
    CardClicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub cards: Vec<Card>,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            cards: Vec::new(),
            onsignal: None,
        }
    }
}

impl Component for CardBufUI {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CardBufUI {
            cards: props.cards,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CardClicked => {
                if let Some(ref onsignal) = self.onsignal {
                    ()
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cards = props.cards;
        self.onsignal = props.onsignal;
        true
    }
}

fn view_card(c: &Card) -> Html<CardUI> {
    html! {
        <CardUI card=c onsignal=Msg::CardClicked />
    }
}

impl Renderable<CardBufUI> for CardBufUI {
    fn view(&self) -> Html<Self> {
        let c = |x| html! { 
            <CardUI card=x onsignal=Msg::CardClicked />
        };
        let len = self.cards.len();
        html! {
            <div>
                { for self.cards.iter().map(c) }
            </div>
        }
    }
}
