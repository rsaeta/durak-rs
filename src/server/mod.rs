pub mod api;
pub mod game_session;
pub mod websocket;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::game::actions::Action;
use crate::game::game::{Game, GameLogic};
use crate::game::gamestate::{GamePlayer, ObservableGameHistory};
use crate::game::player::Player;
use std::time::{SystemTime, UNIX_EPOCH};

pub type GameSessions = Arc<RwLock<HashMap<Uuid, Arc<RwLock<GameSession>>>>>;

#[derive(Clone)]
pub struct ActionHistoryEntry {
    pub player: GamePlayer,
    pub action: Action,
    pub timestamp: u64,
}

pub struct GameSession {
    pub id: Uuid,
    pub game: Game,
    pub player1_id: Option<String>,
    pub player2_id: Option<String>,
    pub action_history: Vec<ActionHistoryEntry>,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            game: Game::new(),
            player1_id: None,
            player2_id: None,
            action_history: Vec::new(),
        }
    }

    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn record_action(&mut self, player: GamePlayer, action: Action) {
        self.action_history.push(ActionHistoryEntry {
            player,
            action,
            timestamp: Self::get_timestamp(),
        });
        // Keep only last 100 actions
        if self.action_history.len() > 100 {
            self.action_history.remove(0);
        }
    }

    /// Process turns for a specific player until it's the other player's turn or the game is over.
    /// Uses GameLogic trait methods to handle game state.
    pub fn process_player_turns<F>(&mut self, player: GamePlayer, mut get_player: F) -> bool
    where
        F: FnMut() -> Box<dyn Player>,
    {
        let mut made_move = false;
        while self.game.game_state.acting_player == player && !self.game.is_over() {
            // Use GameLogic methods instead of direct access
            let actions = self.game.get_actions();
            let current_player = self.game.game_state.acting_player;
            let history: Vec<_> = self
                .game
                .history
                .iter()
                .map(|state| state.observe(current_player))
                .collect();

            let mut player_instance = get_player();
            let action = player_instance.choose_action(
                self.game.game_state.observe(current_player),
                actions,
                ObservableGameHistory(history),
            );

            // Use GameLogic::step instead of direct step call
            if self.game.step(action).is_ok() {
                // Record the action in history
                self.record_action(current_player, action);
                made_move = true;
            } else {
                // If step fails, break to avoid infinite loop
                break;
            }
        }
        made_move
    }

    /// Make AI moves if it's Player2's turn, reusing GameLogic functionality
    pub fn make_ai_move_if_needed(&mut self) -> bool {
        use crate::game::player::RandomPlayer;
        self.process_player_turns(GamePlayer::Player2, || Box::new(RandomPlayer::new(None)))
    }

    pub fn get_player_id(&self, player: GamePlayer) -> Option<String> {
        match player {
            GamePlayer::Player1 => self.player1_id.clone(),
            GamePlayer::Player2 => self.player2_id.clone(),
        }
    }

    pub fn assign_player(&mut self, player: GamePlayer, player_id: String) -> bool {
        match player {
            GamePlayer::Player1 => {
                if self.player1_id.is_none() {
                    self.player1_id = Some(player_id);
                    true
                } else {
                    false
                }
            }
            GamePlayer::Player2 => {
                if self.player2_id.is_none() {
                    self.player2_id = Some(player_id);
                    true
                } else {
                    false
                }
            }
        }
    }
}
