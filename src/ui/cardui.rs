use super::super::cards::card::Card;
use super::super::cards::card::Suit;
use yew::prelude::*;

pub struct CardUI {
    card: Card,
    onsignal: Option<Callback<(Card)>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub card: Card,
    pub onsignal: Option<Callback<(Card)>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            card: Card::new(0u32, Suit::Club, false),
            onsignal: None,
        }
    }
}

impl Component for CardUI {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CardUI {
            card: props.card,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref onsignal) = self.onsignal {
                    onsignal.emit(self.card);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.card = props.card;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<CardUI> for CardUI {
    fn view(&self) -> Html<Self> {
        let (val, suit, color) = display_card(self.card);
        let style1 = "card-text ".to_string() + &color;
        let style2 = "card-img ".to_string() + &color;
        html! {
            <div class="card-small" onclick=|_| Msg::Clicked>
                <p class=style1>{ val }</p>
                <p class=style2>{ suit }</p>
            </div>
        }
    }
}

fn display_card(card: Card) -> (String, String, String) {
    let str_value: String;
    let str_suit: String;
    let str_class: String;

    if card.value >= 3 && card.value <= 10 {
        str_value = card.value.to_string();
    } else if card.value == 11 {
        str_value = "J".to_string();
    } else if card.value == 12 {
        str_value = "Q".to_string();
    } else if card.value == 13 {
        str_value = "K".to_string();
    } else if card.value == 14 {
        str_value = "A".to_string();
    } else if card.value == 15 {
        str_value = 2.to_string();
    } else {
        str_value = "?".to_string();
    }

    if card.suit == Suit::Heart {
        str_suit = "♥".to_string();
        str_class = " red".to_string();
    } else if card.suit == Suit::Spade {
        str_suit = "♠".to_string();
        str_class = " black".to_string();
    } else if card.suit == Suit::Club {
        str_suit = "♣".to_string();
        str_class = " black".to_string();
    } else if card.suit == Suit::Diamond {
        str_suit = "♦".to_string();
        str_class = " red".to_string();
    } else {
        str_suit = "?".to_string();
        str_class = " ?".to_string();
    }

    (str_value, str_suit, str_class)
}
