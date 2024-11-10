use std::{collections::HashSet, vec};

use super::{
    actions::{Action, ActionList},
    cards::{Card, Deck, Hand, Suit},
    gamestate::{GamePlayer, GameState},
    player::{Player, RandomPlayer},
};

pub struct Game {
    pub history: Vec<GameState>,
    pub game_state: GameState,
}

fn det_first_attacker(hand1: &Hand, hand2: &Hand, suit: Suit) -> GamePlayer {
    let min1c = hand1
        .0
        .iter()
        .filter(|x| x.suit == suit)
        .min_by_key(|x| x.rank);
    let min2c = hand2
        .0
        .iter()
        .filter(|x| x.suit == suit)
        .min_by_key(|x| x.rank);
    match (min1c, min2c) {
        (
            Some(&Card {
                suit: _,
                rank: rank1,
            }),
            Some(&Card {
                suit: _,
                rank: rank2,
            }),
        ) => match rank1 < rank2 {
            true => GamePlayer::Player1,
            false => GamePlayer::Player2,
        },
        (Some(_), None) => GamePlayer::Player1,
        (None, Some(_)) => GamePlayer::Player2,
        (None, None) => GamePlayer::Player1,
    }
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new(6);
        deck.shuffle();
        let hand1 = Hand(deck.draw_n(6));
        let hand2 = Hand(deck.draw_n(6));
        let visible_card = deck.get_first().unwrap();
        let first_attacker = det_first_attacker(&hand1, &hand2, visible_card.suit);
        let game_state = GameState::new(
            deck,
            Vec::new(),
            Vec::new(),
            hand1,
            hand2,
            first_attacker,
            !first_attacker,
            visible_card,
            false,
            Vec::new(),
        );

        Game {
            game_state,
            history: Vec::new(),
        }
    }

    fn defender_hand(&self) -> &Hand {
        match self.game_state.defending_player {
            GamePlayer::Player1 => &self.game_state.hand1,
            GamePlayer::Player2 => &self.game_state.hand2,
        }
    }

    fn _attacker_hand(&mut self) -> &mut Hand {
        match self.game_state.defending_player.other() {
            GamePlayer::Player1 => &mut self.game_state.hand1,
            GamePlayer::Player2 => &mut self.game_state.hand2,
        }
    }

    fn attacker_hand(&self) -> &Hand {
        match self.game_state.defending_player.other() {
            GamePlayer::Player1 => &self.game_state.hand1,
            GamePlayer::Player2 => &self.game_state.hand2,
        }
    }

    /// This function should be called after a round of the game has ended and the cards on the table have been added to the defender's hand.
    /// It refills the hands of the players up to 6 cards, starting with the player who will be attacking in the next round.
    fn refill_hands(&mut self) {
        let refill_order = match self.game_state.defending_player {
            GamePlayer::Player2 => vec![GamePlayer::Player1, GamePlayer::Player2],
            GamePlayer::Player1 => vec![GamePlayer::Player2, GamePlayer::Player1],
        };
        for player in refill_order.iter() {
            let hand = match player {
                GamePlayer::Player1 => &mut self.game_state.hand1,
                GamePlayer::Player2 => &mut self.game_state.hand2,
            };
            let num_cards: i8 = 6 - hand.0.len() as i8;
            if num_cards > 0 {
                let mut new_cards = self.game_state.deck.draw_n(num_cards as usize);
                hand.0.append(&mut new_cards);
            }
        }
    }

    fn add_table_to_defender(&mut self) {
        // Temporarily take mutable references to the tables you want to modify.
        let defense_table = &mut self.game_state.defense_table;
        let attack_table = &mut self.game_state.attack_table;

        // Borrow `self` mutably once to get a mutable reference to the defender's hand.
        let hand = match self.game_state.defending_player {
            GamePlayer::Player1 => &mut self.game_state.hand1.0,
            GamePlayer::Player2 => &mut self.game_state.hand2.0,
        };

        // Now, you can append the tables to the hand without violating Rust's borrowing rules,
        // because `hand`, `defense_table`, and `attack_table` are clearly separate mutable references.
        hand.append(defense_table);
        hand.append(attack_table);
    }

    fn clear_table(&mut self) {
        self.game_state
            .graveyard
            .append(&mut self.game_state.attack_table);
        self.game_state
            .graveyard
            .append(&mut self.game_state.defense_table);
    }

    fn handle_take(&mut self) {
        // check whether attacker can add more cards
        let num_attack = self.game_state.attack_table.len() as u8;
        let num_defend = self.game_state.defense_table.len() as u8;
        if num_attack == 6 || (num_attack - num_defend) >= self.defender_hand().0.len() as u8 {
            // here we need to give defender all cards, round is over
            self.add_table_to_defender();
            self.refill_hands();
            self.game_state.acting_player = self.game_state.acting_player.other();
        } else {
            // just need to give controller back to attacker after setting flag
            self.game_state.defender_has_taken = true;
            self.game_state.acting_player = self.game_state.acting_player.other();
        }
    }

    // Function to handle the stop attack action
    fn handle_stop_attack(&mut self) {
        // If the defender has taken the cards
        if self.game_state.defender_has_taken {
            // Add the table cards to the defender's hand
            self.add_table_to_defender();
            // Refill the hands of the players
            self.refill_hands();
        } else {
            // If there are no undefended cards on the table
            if self.game_state.num_undefended() == 0 {
                // Clear the table
                self.clear_table();
                // Switch the defending player
                self.game_state.defending_player = self.game_state.defending_player.other();
                // Refill the hands of the players
                self.refill_hands();
            }
            // Switch the acting player
            self.game_state.acting_player = self.game_state.acting_player.other();
        }
        // Reset the flag indicating that the defender has taken the cards
        self.game_state.defender_has_taken = false;
    }

    fn handle_attack(&mut self, card: Card) {
        self.game_state.attack_table.push(card);
        // remove card from player hand
        let hand = self._attacker_hand();
        let index = hand.0.iter().position(|x| *x == card).unwrap();
        hand.0.remove(index);
    }

    // Function to handle the defense action
    fn handle_defense(&mut self, card: Card) {
        // Add the card to the defense table
        self.game_state.defense_table.push(card);
        {
            // Determine the hand of the defending player
            let hand = match self.game_state.defending_player {
                GamePlayer::Player1 => &mut self.game_state.hand1,
                GamePlayer::Player2 => &mut self.game_state.hand2,
            };
            // Find the position of the card in the hand
            let index = hand.0.iter().position(|x| *x == card).unwrap();
            // Remove the card from the hand
            hand.0.remove(index);
        }
        // If the defense table is full or the defender has no cards left
        if self.game_state.defense_table.len() == 6 || self.defender_hand().0.len() == 0 {
            // Clear the table
            self.clear_table();
            // Refill the hands of the players
            self.refill_hands();
            // Reset the flag indicating that the defender has taken the cards
            self.game_state.defender_has_taken = false;
            // Switch the defending player
            self.game_state.defending_player = self.game_state.defending_player.other();
        }
        // If there are no undefended cards on the table
        else if self.game_state.num_undefended() == 0 {
            // Switch the acting player
            self.game_state.acting_player = self.game_state.acting_player.other();
        }
    }

    fn ranks(&self) -> HashSet<u8> {
        let mut ranks = HashSet::new();
        for card in self.game_state.attack_table.iter() {
            ranks.insert(card.rank);
        }
        for card in self.game_state.defense_table.iter() {
            ranks.insert(card.rank);
        }
        ranks
    }

    // This function determines the legal attack actions for the current game state
    fn legal_attacks(&self) -> Vec<Action> {
        // Initialize an empty vector to store the actions
        let mut actions = Vec::new();
        // Check the length of the attack table
        match self.game_state.attack_table.len() {
            // If the attack table is empty, all cards in the attacker's hand are legal attacks
            0 => self
                .attacker_hand()
                .0
                .iter()
                // Map each card in the attacker's hand to an Attack action
                .map(|card| Action::Attack(*card))
                .collect(),
            // If the attack table is not empty
            _ => {
                // Get the ranks of the cards on the table
                let ranks = self.ranks();
                // Add the StopAttack action to the list of actions
                actions.push(Action::StopAttack);
                // Append the legal attack actions to the list of actions
                actions.append(
                    &mut self
                        .attacker_hand()
                        .0
                        .iter()
                        // Filter the cards in the attacker's hand that have the same rank as the cards on the table
                        .filter(|card| ranks.contains(&card.rank))
                        // Map each card to an Attack action
                        .map(|card| Action::Attack(*card))
                        .collect(),
                );
                // Return the list of actions
                actions
            }
        }
    }

    // This function determines the legal defense actions for the current game state
    fn legal_defenses(&self) -> Vec<Action> {
        // Initialize an empty vector to store the actions
        let mut actions = Vec::new();
        // Add the Take action to the list of actions
        actions.push(Action::Take);
        // Get the last attack from the attack table
        let last_attack = self.game_state.attack_table[self.game_state.defense_table.len()];
        // Get the suit of the visible card
        let tsuit = self.game_state.visible_card.suit;
        // Initialize a vector to store the defense actions
        let mut defenses = self
            .defender_hand()
            .0
            .iter()
            // Filter the cards in the defender's hand that can legally defend against the last attack
            .filter(|card| match last_attack {
                Card {
                    suit: a_suit,
                    rank: a_rank,
                } if a_suit == tsuit => match card {
                    Card {
                        suit: d_suit,
                        rank: d_rank,
                    } if *d_suit == tsuit => *d_rank > a_rank,
                    _ => false,
                },
                Card {
                    suit: a_suit,
                    rank: a_rank,
                } => match card {
                    Card {
                        suit: d_suit,
                        rank: d_rank,
                    } => (*d_suit == tsuit) || (*d_suit == a_suit && *d_rank > a_rank),
                },
            })
            // Map each card to a Defend action
            .map(|i| Action::Defend(*i))
            // Collect the defense actions into a vector
            .collect::<Vec<Action>>();
        // Append the defense actions to the list of actions
        actions.append(&mut defenses);
        // Return the list of actions
        actions
    }

    pub fn legal_actions(&self) -> ActionList {
        let actions = match (
            self.game_state.acting_player,
            self.game_state.defending_player,
        ) {
            (a, b) if a == b => self.legal_defenses(),
            _ => self.legal_attacks(),
        };
        ActionList(actions)
    }

    #[allow(dead_code)]
    pub fn play(
        &mut self,
        mut player1: Box<dyn Player>,
        mut player2: Box<dyn Player>,
    ) -> Result<(f32, f32), &str> {
        let mut game_over = false;
        while !game_over {
            let pta = self.game_state.acting_player;
            let actions = self.legal_actions();
            let player = match pta {
                GamePlayer::Player1 => &mut player1,
                GamePlayer::Player2 => &mut player2,
            };
            let history = self.history.iter().map(|x| x.observe(pta)).collect();
            let action =
                player
                    .as_mut()
                    .choose_action(self.game_state.observe(pta), actions, history);
            match self.step(action) {
                Ok(_) => (),
                Err(_e) => (),
            };

            game_over = self.is_over();
        }
        Ok(self.get_rewards())
    }
}

pub trait GameLogic {
    fn step(&mut self, action: Action) -> Result<(), &str>;
    fn get_actions(&self) -> ActionList;
    fn get_winner(&self) -> Option<GamePlayer>;
    fn get_rewards(&self) -> (f32, f32);
    fn is_over(&self) -> bool;
}

impl GameLogic for Game {
    fn step(&mut self, action: Action) -> Result<(), &str> {
        let current_state = self.game_state.clone();
        self.history.push(current_state);
        let legal_actions = self.legal_actions();
        if !legal_actions.0.contains(&action) {
            return Err("Illegal action");
        }
        match action {
            Action::StopAttack => self.handle_stop_attack(),
            Action::Take => self.handle_take(),
            Action::Attack(card) => self.handle_attack(card),
            Action::Defend(card) => self.handle_defense(card),
        }

        Ok(())
    }

    fn get_actions(&self) -> ActionList {
        self.legal_actions()
    }

    fn get_winner(&self) -> Option<GamePlayer> {
        let sizes = vec![
            self.game_state.hand1.0.len(),
            self.game_state.hand2.0.len(),
            self.game_state.deck.len(),
        ];
        match sizes.as_slice() {
            [_, _, 1..=52] => None,
            [0, 0, 0] => None,
            [0, _, _] => Some(GamePlayer::Player1),
            [_, 0, _] => Some(GamePlayer::Player2),
            _ => None,
        }
    }

    fn get_rewards(&self) -> (f32, f32) {
        let winner = self.get_winner();
        match winner {
            Some(GamePlayer::Player1) => (1.0, -1.0),
            Some(GamePlayer::Player2) => (-1.0, 1.0),
            None => (0.0, 0.0),
        }
    }

    fn is_over(&self) -> bool {
        let sizes = vec![
            self.game_state.hand1.0.len(),
            self.game_state.hand2.0.len(),
            self.game_state.deck.len(),
        ];
        match sizes.as_slice() {
            [_, _, 1..=52] => false,
            [0, 0, 0] => true,
            [0, _, _] => true,
            [_, 0, _] => true,
            _ => false,
        }
    }
}

pub fn _run_game() -> (f32, f32) {
    let mut p1 = Box::new(RandomPlayer::new(None));
    let mut p2 = Box::new(RandomPlayer::new(None));
    let mut game = Game::new();
    let mut game_over = false;
    'game_loop: loop {
        if game_over {
            break 'game_loop;
        }
        let pta = game.game_state.acting_player;
        let actions = game.get_actions();
        let player = match pta {
            GamePlayer::Player1 => p1.as_mut(),
            GamePlayer::Player2 => p2.as_mut(),
        };
        let history = game.history.iter().map(|x| x.observe(pta)).collect();
        let action = player.choose_action(game.game_state.observe(pta), actions, history);
        'step_loop: loop {
            match game.step(action) {
                Ok(_) => break 'step_loop,
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        }
        game_over = game.is_over();
    }
    game.get_rewards()
    // println!("Rewards: {:?}", rewards);
}
