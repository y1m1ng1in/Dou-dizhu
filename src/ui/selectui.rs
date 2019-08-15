use yew::prelude::*;

pub struct SelectUI {
    display: bool,
    message: String,
    onsignal: Option<Callback<(bool)>>,
}

pub enum Msg {
    AgainClicked,
    NextClicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub display: bool,
    pub message: String,
    pub onsignal: Option<Callback<(bool)>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            display: false,
            message: "".to_string(),
            onsignal: None,
        }
    }
}

impl Component for SelectUI {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SelectUI {
            display: props.display,
            message: props.message,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AgainClicked => {
                if let Some(ref onsignal) = self.onsignal {
                    onsignal.emit(false);
                }
            }
            Msg::NextClicked => {
                if let Some(ref onsignal) = self.onsignal {
                    onsignal.emit(true);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.display = props.display;
        self.onsignal = props.onsignal;
        self.message = props.message;
        true
    }
}

impl Renderable<SelectUI> for SelectUI {
    fn view(&self) -> Html<Self> {
        let show = if self.display {
            "display: block"
        } else {
            "display: none"
        };
        html! {
            <div style=show class="select-container">
                <p style="text-align: center">{ &self.message }</p>
                <button onclick=|_| Msg::NextClicked>{ "Play Next Game" }</button>
                <button onclick=|_| Msg::AgainClicked>{ "Play Again" }</button>
            </div>
        }
    }
}
