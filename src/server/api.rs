use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::game::actions::Action;
use crate::game::cards::{Card, Suit};
use crate::game::game::GameLogic;
use crate::game::gamestate::{GamePlayer, ObservableGameState};
use crate::server::{game_session::GameSessions, GameSession};

#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub game_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameStateResponse {
    pub game_id: String,
    pub game_state: ObservableGameStateDto,
    pub legal_actions: Vec<ActionDto>,
    pub is_over: bool,
    pub winner: Option<String>,
    pub action_history: Vec<ActionHistoryEntryDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionHistoryEntryDto {
    pub player: String,
    pub action: ActionDto,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ObservableGameStateDto {
    pub player: String,
    pub num_cards_in_deck: u8,
    pub attack_table: Vec<CardDto>,
    pub defense_table: Vec<CardDto>,
    pub hand: Vec<CardDto>,
    pub visible_card: CardDto,
    pub defender_has_taken: bool,
    pub acting_player: String,
    pub defender: String,
    pub cards_in_opponent: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CardDto {
    pub suit: String,
    pub rank: u8,
}

impl From<Card> for CardDto {
    fn from(card: Card) -> Self {
        CardDto {
            suit: match card.suit {
                Suit::Spades => "Spades".to_string(),
                Suit::Hearts => "Hearts".to_string(),
                Suit::Diamonds => "Diamonds".to_string(),
                Suit::Clubs => "Clubs".to_string(),
            },
            rank: card.rank,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ActionDto {
    pub action_type: String,
    pub card: Option<CardDto>,
}

impl From<Action> for ActionDto {
    fn from(action: Action) -> Self {
        match action {
            Action::StopAttack => ActionDto {
                action_type: "StopAttack".to_string(),
                card: None,
            },
            Action::Take => ActionDto {
                action_type: "Take".to_string(),
                card: None,
            },
            Action::Attack(card) => ActionDto {
                action_type: "Attack".to_string(),
                card: Some(CardDto::from(card)),
            },
            Action::Defend(card) => ActionDto {
                action_type: "Defend".to_string(),
                card: Some(CardDto::from(card)),
            },
        }
    }
}

impl From<ObservableGameState> for ObservableGameStateDto {
    fn from(state: ObservableGameState) -> Self {
        ObservableGameStateDto {
            player: format!("{:?}", state.player),
            num_cards_in_deck: state.num_cards_in_deck,
            attack_table: state
                .attack_table
                .iter()
                .map(|c| CardDto::from(*c))
                .collect(),
            defense_table: state
                .defense_table
                .iter()
                .map(|c| CardDto::from(*c))
                .collect(),
            hand: state.hand.0.iter().map(|c| CardDto::from(*c)).collect(),
            visible_card: CardDto::from(state.visible_card),
            defender_has_taken: state.defender_has_taken,
            acting_player: format!("{:?}", state.acting_player),
            defender: format!("{:?}", state.defender),
            cards_in_opponent: state.cards_in_opponent,
        }
    }
}

#[derive(Deserialize)]
pub struct MakeMoveRequest {
    pub action_type: String,
    pub card: Option<CardDto>,
}

pub async fn create_game(
    State(sessions): State<GameSessions>,
) -> Result<Json<CreateGameResponse>, StatusCode> {
    let session = GameSession::new();
    let game_id = session.id;

    sessions
        .write()
        .await
        .insert(game_id, Arc::new(tokio::sync::RwLock::new(session)));

    Ok(Json(CreateGameResponse {
        game_id: game_id.to_string(),
    }))
}

pub async fn get_game_state(
    State(sessions): State<GameSessions>,
    Path(game_id): Path<String>,
) -> Result<Json<GameStateResponse>, StatusCode> {
    let uuid = Uuid::parse_str(&game_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let sessions_read = sessions.read().await;
    let session = sessions_read.get(&uuid).ok_or(StatusCode::NOT_FOUND)?;

    let mut game = session.write().await;

    // Make AI moves if it's Player2's turn - process_player_turns handles the loop
    game.make_ai_move_if_needed();

    // Use GameLogic methods for consistency
    let observable_state = game.game.game_state.observe(GamePlayer::Player1);
    let legal_actions = game.game.get_actions();
    let is_over = game.game.is_over();
    let winner = game.game.get_winner().map(|p| format!("{:?}", p));
    let action_history: Vec<ActionHistoryEntryDto> = game
        .action_history
        .iter()
        .map(|entry| ActionHistoryEntryDto {
            player: format!("{:?}", entry.player),
            action: ActionDto::from(entry.action),
            timestamp: entry.timestamp,
        })
        .collect();

    Ok(Json(GameStateResponse {
        game_id: game_id.clone(),
        game_state: ObservableGameStateDto::from(observable_state),
        legal_actions: legal_actions
            .0
            .iter()
            .map(|a| ActionDto::from(*a))
            .collect(),
        is_over,
        winner,
        action_history,
    }))
}

pub async fn make_move(
    State(sessions): State<GameSessions>,
    Path(game_id): Path<String>,
    Json(request): Json<MakeMoveRequest>,
) -> Result<Json<GameStateResponse>, StatusCode> {
    let uuid = Uuid::parse_str(&game_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let sessions_read = sessions.read().await;
    let session = sessions_read.get(&uuid).ok_or(StatusCode::NOT_FOUND)?;

    let mut game = session.write().await;

    // Convert request to Action
    let action = match request.action_type.as_str() {
        "StopAttack" => Action::StopAttack,
        "Take" => Action::Take,
        "Attack" => {
            let card_dto = request.card.ok_or(StatusCode::BAD_REQUEST)?;
            let card = Card {
                suit: match card_dto.suit.as_str() {
                    "Spades" => Suit::Spades,
                    "Hearts" => Suit::Hearts,
                    "Diamonds" => Suit::Diamonds,
                    "Clubs" => Suit::Clubs,
                    _ => return Err(StatusCode::BAD_REQUEST),
                },
                rank: card_dto.rank,
            };
            Action::Attack(card)
        }
        "Defend" => {
            let card_dto = request.card.ok_or(StatusCode::BAD_REQUEST)?;
            let card = Card {
                suit: match card_dto.suit.as_str() {
                    "Spades" => Suit::Spades,
                    "Hearts" => Suit::Hearts,
                    "Diamonds" => Suit::Diamonds,
                    "Clubs" => Suit::Clubs,
                    _ => return Err(StatusCode::BAD_REQUEST),
                },
                rank: card_dto.rank,
            };
            Action::Defend(card)
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Get the current acting player before the move
    let acting_player = game.game.game_state.acting_player;

    // Execute the action using GameLogic::step
    game.game
        .step(action)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Record the player's action
    game.record_action(acting_player, action);

    // Make AI moves if it's now Player2's turn - process_player_turns handles the loop
    game.make_ai_move_if_needed();

    // Get updated state using GameLogic methods
    let observable_state = game.game.game_state.observe(GamePlayer::Player1);
    let legal_actions = game.game.get_actions();
    let is_over = game.game.is_over();
    let winner = game.game.get_winner().map(|p| format!("{:?}", p));
    let action_history: Vec<ActionHistoryEntryDto> = game
        .action_history
        .iter()
        .map(|entry| ActionHistoryEntryDto {
            player: format!("{:?}", entry.player),
            action: ActionDto::from(entry.action),
            timestamp: entry.timestamp,
        })
        .collect();

    Ok(Json(GameStateResponse {
        game_id: game_id.clone(),
        game_state: ObservableGameStateDto::from(observable_state),
        legal_actions: legal_actions
            .0
            .iter()
            .map(|a| ActionDto::from(*a))
            .collect(),
        is_over,
        winner,
        action_history,
    }))
}

pub fn create_api_router(sessions: GameSessions) -> Router {
    Router::new()
        .route("/games", post(create_game))
        .route("/games/:game_id", get(get_game_state))
        .route("/games/:game_id/move", post(make_move))
        .with_state(sessions)
}
