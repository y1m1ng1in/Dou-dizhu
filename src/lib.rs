#![recursion_limit = "256"]

pub mod cards;
pub mod data;
pub mod player;
pub mod ui;

use cards::card::Card;
use cards::utils;
use data::*;
use player::computer::ComputerPlayer;
use ui::cardbufui::CardBufUI;
use ui::selectui::SelectUI;
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
    computer_pass: bool,
    has_result: bool,
    mission: u32,
    game_message: String,
    total_mission: u32,
}

pub enum Msg {
    PlayerCardClicked(Card),
    PlayerBufferClicked(Card),
    ObtainCard(bool),
    PlayerHandIn,
    PlayerPass,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let (user, computer) = get_cards(1u32);
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
            computer_pass: false,
            has_result: false,
            mission: 1u32,
            game_message: String::new(),
            total_mission: 6u32,
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
                let mut rc = 1;

                if pattern != utils::Pattern::Invalid {
                    if !self.computer_buffer.is_empty() {
                        rc = utils::compare_known_pattern(
                            &self.player_buffer,
                            &self.computer_buffer,
                            pattern,
                        );
                    }

                    if rc == 1 {
                        if self.player_cards.is_empty() {
                            self.has_result = true;
                            self.game_message = "Player Wins!".to_string();
                            self.computer_pass = false;
                        } else {
                            self.computer_buffer = self
                                .computer_strategy
                                .hand_in_follow(&self.player_buffer, pattern);
                            self.player_buffer = vec![];

                            if self.computer_buffer.is_empty() {
                                self.computer_pass = true;
                            } else {
                                self.computer_pass = false;
                            }
                            self.computer_cards = self.computer_strategy.display();
                        }
                    } else {
                        self.player_cards.append(&mut self.player_buffer);
                        self.player_cards.sort();
                    }

                    if rc == 0 {
                        self.player_message =
                            "Should hand in cards with greater value..".to_string();
                    } else if rc == -1 {
                        self.player_message =
                            "Pattern doesn't matching with computer player..".to_string();
                    } else {
                        self.player_message = pattern.to_string();
                    }
                } else {
                    self.player_cards.append(&mut self.player_buffer);
                    self.player_cards.sort();
                    self.player_message = pattern.to_string();
                }

                self.console.log(&self.computer_strategy.to_string());
            }
            Msg::PlayerPass => {
                self.player_cards.append(&mut self.player_buffer);
                self.player_cards.sort();
                self.player_message = String::new();

                self.computer_buffer = self.computer_strategy.hand_in_first(&self.player_cards);
                self.computer_cards = self.computer_strategy.display();
                self.computer_pass = false;

                if self.computer_buffer.is_empty() {
                    self.player_message = "Internal error occurs!".to_string();
                }

                if self.computer_cards.is_empty() {
                    self.has_result = true;
                    self.game_message = "Computer player wins!".to_string();
                } else {
                    self.has_result = false;
                    self.game_message = String::new();
                }
            }
            Msg::ObtainCard(selection) => {
                if selection {
                    self.mission += 1;
                    if self.mission > self.total_mission {
                        self.mission = 1;
                    }
                }
                let (user, computer) = data::get_cards(self.mission);
                let strategy = ComputerPlayer::new(computer);
                let computer_card = strategy.display();

                self.computer_strategy = strategy;
                self.computer_cards = computer_card;
                self.computer_buffer = vec![];
                self.computer_pass = false;
                self.player_cards = user;
                self.player_buffer = vec![];
                self.player_message = String::new();
                self.has_result = false;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let pass_btn = match self.computer_buffer.len() {
            0 => "display: none",
            _ => "display: inline",
        };
        html! {
            <div>
                <SelectUI display=&self.has_result message=&self.game_message onsignal=Msg::ObtainCard />
                <div class="computer-container">
                    <CardBufUI cards=&self.computer_cards />
                    <CardBufUI cards=&self.computer_buffer ispass=self.computer_pass />
                </div>
                <div class="user-container">
                    <CardBufUI cards=&self.player_buffer onsignal=Msg::PlayerBufferClicked />
                    <CardBufUI cards=&self.player_cards onsignal=Msg::PlayerCardClicked />
                    <div class="user-button-container">
                        <button onclick=|_| Msg::PlayerHandIn>{ "Hand in" }</button>
                        <button style=pass_btn onclick=|_| Msg::PlayerPass>{ "Pass" }</button>
                    </div>
                    <p>{ &self.player_message }</p>
                </div>
            </div>
        }
    }
}
