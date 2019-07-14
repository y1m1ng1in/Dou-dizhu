
## Dou dizhu -- A traditional Chinese card game inplementation

The project is built for CS410P the Rust Programming course in Portland State University (Summer 2019). 

Keywords:
- Frontend project
- Webapp
- Yew (the framework used in the project)

#### License
[License](https://github.com/y1m1ng1in/Dou-dizhu/blob/master/LICENSE)

**Table of Contents**
- [Dou dizhu -- A traditional Chinese card game inplementation](#dou-dizhu----a-traditional-chinese-card-game-inplementation)
    + [License](#license)
    + [Group Member](#group-member)
    + [Project Details](#project-details)
      - [Overview](#overview)
      - [Game Process](#game-process)
      - [The Rules of Card Pattern and Value](#the-rules-of-card-pattern-and-value)
    + [An Example Illustrates the Operation of the Code](#an-example-illustrates-the-operation-of-the-code)
    + [Build and Run](#build-and-run)
    + [Schedule and Milestone](#schedule-and-milestone)

#### Group Member
Yiming Lin (Email address: yl6@pdx.edu)

#### Project Details

##### Overview

This is a frontend project that implements the traditional Chinese card game named "Dou dizhu". The framework used in this project is "Yew", which is a frontend web application framework. 

This game has tons of ways to play, there are lots of mode to play as well. In this project, one of the game mode called "final phase" is implemented.  It requires *One* player to play with computer. The player and the computer play **in turn**: discard cards by a set of rules, or give up current turn's right to discard cards. If the player has no card left, the player wins the computer; otherwise, the computer wins the player. 


##### Game Process

1. In this project, player always discard cards firstly, then computer does. 
2. Once player finishes discarding card, the computer searches its owned cards that have the same pattern, also the value is greater than the cards the player has already discarded to discard; otherwase, the computer is forced to give up right to discard cards in this turn. 
*Note: this step will be explained in detail in "The Rules of Card Pattern and Value"*
3. After the computer discarding cards or giving up its right to discard cards, the turn is back to player. If the player can't discard cards (becase the player doesn't have cards that have the same pattern as computer discarded, or its value is less than computer discarded), the turn goes to computer again. 
4. Repeat process 2 and 3 until either the player or the computer has no card left first. 

##### The Rules of Card Pattern and Value
- **Individual cards are ranked.**<br> Colored Joker > Black & White Joker > 2 > Ace (A) > King (K) > Queen (Q) > Jack (J) > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3.

- **Rocket** <br>*Colored Joker + black-and-white Joker* : <br>It can beat everything in the game. 

- **Bomb** <br>*3-3-3-3 (the lowest ranking Bomb)<br>
2-2-2-2 (the highest ranking Bomb)*<br>
It can beat any other category (pattern) and individual card except Rocket or another Bomb with a higher or equal rank. 

- **Compare only the same Category (Pattern)** <br>
A player can only beat the prior hand using of the **same category (pattern)** but not the others.

- **Compare only the Chains with the same length** except rocket and bomb<br>
For example, although both 9-10-J-Q-K and 3-4-5-6-7-8-9 are Single Chains, 9-10-J-Q-K cannot beat 3-4-5-6-7-8-9, nor vice versa.

- **Jokers and 2 are non-consecutive cards**<br>
Examples of illegal Chain: 2-3-4-5-6, 2-2-2-3-3-3 w/ A-A-7-7, K-A-2 + B&W Joker + Colored Joker.

- **Categories (patterns)** 
1. solo: Any single card.
2. solo chain: ≥ Five consecutive individual cards (like 3-4-5-6-7, 10-J-Q-K-A ...)
3. pair: Two matching cards of equal rank (like 3-3, 5-5, 2-2 ...)
4. pair chain: ≥ Three consecutive pairs (like 3-3-4-4-5-5, 10-10-J-J-Q-Q-K-K ...)
5. Trio: Three individual cards of the same rank (like 3-3-3, 4-4-4, 2-2-2 ...)
6. Airplane: ≥ Two consecutive trios (like 3-3-3-4-4-4, 7-7-7-8-8-8 ...)
 *Note: airplane can followed by two random solos, or two random pairs (like 5-5-5-6-6-6-J-A, 4-4-4-5-5-5-A-A-2-2 ...)*


#### An Example Illustrates the Operation of the Code

```rust
// Yew is a frondend framework that employs Model-View-Controller pattern (MVC).
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

// the top-level model(M) of the app.
pub struct Model {
    console: ConsoleService,
    dummy: u32,
}

// all the updates operations to model 
pub enum Msg {
    Foo,
}

// Implement controller(C) functions to manipulate model
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

// the top-level view(V) for model
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
```

#### Build and Run

In your command line:
```commandline
cargo install --force cargo-web
cargo web build
cargo web start
```

Then open your browser: use the following url:
localhost:8000

#### Schedule and Milestone
July 20th: Finish game logic<br>
July 27th: Finish game UI<br>
Augest 3rd: Finish testing and bugs fixing<br>
Augest 10th: Optimize UI and final delivery<br>