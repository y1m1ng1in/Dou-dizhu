use super::cards::card::Card;
use crate::cardui::CardUI;
use yew::prelude::*;

pub struct CardBufUI {
    cards: Vec<Card>,
    onsignal: Option<Callback<Card>>,
    ispass: bool,
}

pub enum Msg {
    CardClicked(Card),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub cards: Vec<Card>,
    pub onsignal: Option<Callback<Card>>,
    pub ispass: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            cards: Vec::new(),
            onsignal: None,
            ispass: false,
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
            ispass: props.ispass,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CardClicked(card) => {
                if let Some(ref onsignal) = self.onsignal {
                    onsignal.emit(card);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cards = props.cards;
        self.onsignal = props.onsignal;
        self.ispass = props.ispass;
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
        match self.ispass {
            true => {
                html! {
                    <div class="cards-container">
                        <p class="pass-text">{ "Pass" }</p>
                    </div>
                }
            }
            false => {
                html! {
                    <div class="cards-container">
                        { for self.cards.iter().map(c) }
                    </div>
                }
            }
        }
    }
}
