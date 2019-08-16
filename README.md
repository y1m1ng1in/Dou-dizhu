
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
- **Individual cards are ranked.**<br> 2 > Ace (A) > King (K) > Queen (Q) > Jack (J) > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3.

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
pub struct Model {
    console: ConsoleService,    // browser console log
    player_cards: Vec<Card>,    // player owned card
    player_buffer: Vec<Card>,   // player selected card to hand in
    player_message: String,     // message output for player
    computer_cards: Vec<Card>,  // computer owned card
    computer_buffer: Vec<Card>, // computer handed cards in the last turn
    computer_strategy: ComputerPlayer,  // computer's cards pattern strategy (in order to                                                          // reserve chains and bombs)
    computer_pass: bool,        // computer pass this turn 
    has_result: bool,           // whether current game has a winner
    mission: u32,               // current game mission number
    game_message: String,       // message for the whole game
    total_mission: u32,         // total missions in this game
}

pub enum Msg {
    PlayerCardClicked(Card),    // event that player has selected a card in owned cards
    PlayerBufferClicked(Card),  // event thta player has deselected a card in card buffer 
    ObtainCard(bool),           // whether need to grab new cards from data.rs
    PlayerHandIn,               // event that player hand in card
    PlayerPass,                 // event that player pass this turn
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // Initialize the game state
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Fetch user's cards and computer player's cards
        let (user, computer) = get_cards(1u32);

        // Build up computer player's strategy: Pattern division by bombs, chains, trios, pairs
        let strategy = ComputerPlayer::new(computer);

        // Get all computer cards from strategy to display
        let computer_card = strategy.display();

        Model {
            console: ConsoleService::new(), // initialize browser's console log function
            player_cards: user,             // initialize user's owned cards
            player_buffer: vec![],          // initialize an empty buffer for player buffer
            player_message: String::new(),  // initialize empty string for user message
            computer_cards: computer_card,  // initialize computer player's card
            computer_buffer: vec![],        // initialize an empty buffer for computer player
            computer_strategy: strategy,    // setup computer strategy
            computer_pass: false,           // computer player cannot pass without a previous turn
            has_result: false,              // do not have a result at the beginning
            mission: 1u32,                  // the first mission
            game_message: String::new(),    // initialize empty string for game message 
            total_mission: 6u32,            // current total missions in this game
        }
    }

    // this function trigger the HTML page updating, all the HTML elements are bound to the 
    // strcut Model
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlayerCardClicked(card) => {
                // event: player clicked a card from its owned cards
                // remove the clicked Card object from the vector, player_cards
                // and append it to the player_buffer
            }
            Msg::PlayerBufferClicked(card) => {
                // event: player clicked a card from its card buffer
                // remove the clicked Card object from the vector of player_buffer
                // and append it to the vector, player_cards
            }
            Msg::PlayerHandIn => {
                // event: player clicked "hand in" button
                // if the pattern is invalid or the value is not greater than computer
                // player handed cards, move all the cards in player_buffer to the
                // player_cards
                // otherwise, clear player_buffer, and computer player search for greater
                // cards in the same pattern
                // if computer player doesn't have such cards, pass
            }
            Msg::PlayerPass => {
                // event: player clicked "pass" button
                // clear player_buffer vector, and append all the cards in player_buffer
                // to the player_cards
                // computer player hand in cards based on its created "strategy"
            }
            Msg::ObtainCard(selection) => {
                // obtain new two vectors of Card, from data.rs, one for user and the other 
                // for computer player
                // start up a new game
                // setup all the fields in Model with initial states
            }
            true
        }
    }
}

// The top-level HTML code.
// The values in each element are bound to the struct Model
// User triggered the changing of Model via HTML, and changed values will
// trigger update() so the HTML page will be re-rendered without refresh the 
// webpage
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
```

#### An Example Illustrates the **Core** Algorithm for pattern searching
```rust
// The following example demonstates the way that all the patterns are
// searched from a slice of Card whose order has already been sorted 
// in ascending form
// 
// Pair, Trio, SoloChain, PairChain implements this algorithm
// Airplane is formed by Trios, and Pairs or Solos, which can reuse
// the code in Trios, Solos and Pairs.
//
// Bomb is simple, and special in Dou-dizhu, thus not implement it. 
//
// Wrap up a slice of Card and search solo chains from that slice
pub struct SoloChainSearch<'a> {
    cards: &'a [Card],
}

// Fields are used to reserve current state before calling next()
impl<'a> IntoIterator for SoloChainSearch<'a> {
    type Item = Vec<usize>;
    type IntoIter = SoloChainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SoloChainIterator {
            cards: self.cards,
            index: 0,
            previous: 0,    
            start_val: 0,   
        }
    }
}

pub struct SoloChainIterator<'a> {
    cards: &'a [Card],  // a slice of Card
    index: usize,       // current index of that slice
    previous: u32,      // previous value of that Card
    start_val: u32,     // the starting value of next()
}

// Iterator all the solochains from a slice of Card, next() returns 
// a vector of indices in that slice
// 
// For example:
// We have a group of Card with the following values:
//     [3,4,5,6,7,8,9,10,13,14]
// 
// The 1st time next() returns:
//     [0,1,2,3,4,5,6,7]
// The 2nd time next() returns:
//     [1,2,3,4,5,6,7]
// The 3rd time next() returns:
//     [2,3,4,5,6,7]
// The 4rd time next() returns:
//     [3,4,5,6,7] 
// 
// Notice that the length of a solochain must be greater than 4.
// The above example demonstrates that all the solo chains are 
// iterated
impl<'a> Iterator for SoloChainIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        let mut result: Vec<usize> = Vec::new();
        let mut start_at = self.index;
        let mut start_at_moved = false;

        if self.index < self.cards.len() {
            while self.index < self.cards.len() {
                // Get the current value of current index (self.index)
                let current = self.cards[self.index].value;

                // if the current value is same as starting value
                // increment the starting index
                if self.start_val + 1 == current && !start_at_moved {
                    start_at = self.index;
                    start_at_moved = true;
                }

                if current == self.previous {
                    // if the current value is same as the previous value
                    // increment the self.index without doing anything
                    self.index += 1;
                } else if current == self.previous + 1 {
                    // if the current value is greater than the previous value by 1
                    // save that index and increment the self.index
                    result.push(self.index);
                    self.previous = current;
                    self.index += 1;
                } else if result.len() >= 5 {
                    // if the current result vector of indices has already greater than 4
                    // __AND__ previous value plus one is not equal to current value
                    // which means the chain is broken
                    // return the indices
                    self.index = start_at;
                    self.start_val = current;
                    return Some(result);
                } else {
                    // if the chain is broken, and the length of index vector is not 
                    // greater than 4. Clear the previous index vector, and let current 
                    // self.index as the starting point
                    result = Vec::new();
                    result.push(self.index);
                    start_at_moved = false;
                    self.start_val = current;
                    self.previous = current;
                    self.index += 1;
                }
            }
            if result.len() >= 5 {
                self.start_val = self.cards[start_at].value;
                self.index = start_at;
                Some(result)
            } else {
                None
            }
        } else {
            None
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

Then open your browser: use the following url:<br>
localhost:8000

