use super::cards::card::Card;
use crate::cardui::CardUI;
use yew::prelude::*;

pub struct CardBufUI {
    cards: Vec<Card>,
    onsignal: Option<Callback<Card>>,
}

pub enum Msg {
    CardClicked(Card),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub cards: Vec<Card>,
    pub onsignal: Option<Callback<Card>>,
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
            Msg::CardClicked(c) => {
                if let Some(ref onsignal) = self.onsignal {
                    onsignal.emit(c);
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

impl Renderable<CardBufUI> for CardBufUI {
    fn view(&self) -> Html<Self> {
        let c = |x| {
            html! {
                <CardUI card=x onsignal=Msg::CardClicked />
            }
        };
        html! {
            <div>
                { for self.cards.iter().map(c) }
            </div>
        }
    }
}
