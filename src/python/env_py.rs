use std::path::PathBuf;

use crate::game::actions::num_actions;
use crate::game::game::{Game, GameLogic};
use crate::game::gamestate::GamePlayer;
use crate::game::player::{Player, RandomPlayer};
use crate::python::player_py::PlayerPy;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, types::PyString, IntoPy, Py, PyResult, Python};
use rand::{rngs::StdRng, SeedableRng};

/// Python wrapper for the game environment.
///
/// This class provides a gym-like interface for the Durak card game, allowing
/// Python players to interact with the game through a step-by-step API or
/// by playing full games.
#[pyclass(name = "GameEnv", unsendable)]
pub struct GameEnvPy {
    game: Box<Game>,
    player1: Option<Box<PlayerPy>>,
    player2: Option<Box<PlayerPy>>,
    random_player2: Option<Box<RandomPlayer>>,
}

#[pymethods]
impl GameEnvPy {
    #[classmethod]
    pub fn from_file(_cls: Py<PyType>, file_path: Py<PyString>) -> PyResult<Self> {
        let game = Game::from_file(&PathBuf::from(file_path.to_string()));
        Ok(GameEnvPy {
            game: Box::new(game),
            player1: None,
            player2: None,
            random_player2: None,
        })
    }

    /// Create a new game environment.
    ///
    /// Args:
    ///     player1: The first player (required). Must be a subclass of GamePlayer.
    ///     player2: Optional second player. If None, a random player will be used.
    ///     seed: Optional random seed for reproducible games.
    #[new]
    #[pyo3(signature = (player1, player2=None, seed=None))]
    pub fn new(
        player1: Py<crate::python::player_py::GamePlayerPy>,
        player2: Option<Py<crate::python::player_py::GamePlayerPy>>,
        seed: Option<u64>,
    ) -> PyResult<Self> {
        let game = Box::new(Game::new());

        // Apply seed if provided
        if let Some(_seed_val) = seed {
            // Note: This requires modifying Game::new() to accept a seed
            // For now, we'll create the game and note that full seed support
            // requires changes to the Game struct
        }

        let player1_wrapped = Box::new(PlayerPy(player1));
        let (player2_wrapped, random_p2) = match player2 {
            Some(p2) => (Some(Box::new(PlayerPy(p2))), None),
            None => {
                let rng =
                    seed.map(|s| Box::new(StdRng::seed_from_u64(s)) as Box<dyn rand::RngCore>);
                (None, Some(Box::new(RandomPlayer::new(rng))))
            }
        };

        Ok(GameEnvPy {
            game,
            player1: Some(player1_wrapped),
            player2: player2_wrapped,
            random_player2: random_p2,
        })
    }

    /// Reset the game to its initial state.
    ///
    /// Args:
    ///     seed: Optional random seed for reproducible games (not yet fully supported).
    ///
    /// Returns:
    ///     The initial observable game state for player 1.
    pub fn reset(
        &mut self,
        _seed: Option<u64>,
    ) -> PyResult<super::gamestate_py::ObservableGameStatePy> {
        self.game = Box::new(Game::new());
        // TODO: Apply seed when Game::new() supports it
        Ok(super::gamestate_py::ObservableGameStatePy {
            game_state: self.game.game_state.observe(GamePlayer::Player1),
        })
    }

    /// Execute one step in the game.
    ///
    /// Args:
    ///     action_index: The index of the action to take from the legal actions list.
    ///
    /// Returns:
    ///     Tuple of (observation, reward, done, info) where:
    ///     - observation: The new observable game state
    ///     - reward: The reward for the current player (0.0 during game, ±1.0 at end)
    ///     - done: Whether the game is over
    ///     - info: Dictionary with additional information
    pub fn step(
        &mut self,
        action_index: u8,
    ) -> PyResult<(
        super::gamestate_py::ObservableGameStatePy,
        f32,
        bool,
        Py<pyo3::types::PyDict>,
    )> {
        let acting_player = self.game.game_state.acting_player;
        let legal_actions = self.game.legal_actions();

        if action_index as usize >= legal_actions.0.len() {
            return Err(PyIndexError::new_err(format!(
                "Action index {} out of range. Legal actions: {}",
                action_index,
                legal_actions.0.len()
            )));
        }

        let action = legal_actions.0[action_index as usize];

        // Execute the action
        match self.game.step(action) {
            Ok(_) => {}
            Err(e) => {
                return Err(PyValueError::new_err(format!("Illegal action: {}", e)));
            }
        }

        let is_done = self.game.is_over();
        let rewards = self.game.get_rewards();
        let current_reward = match acting_player {
            GamePlayer::Player1 => rewards.0,
            GamePlayer::Player2 => rewards.1,
        };

        // Get the new observation for the next acting player
        let next_acting_player = self.game.game_state.acting_player;
        let observation = super::gamestate_py::ObservableGameStatePy {
            game_state: self.game.game_state.observe(next_acting_player),
        };

        // Create info dict
        let info = Python::with_gil(|py| -> Py<pyo3::types::PyDict> {
            let dict = pyo3::types::PyDict::new(py);
            dict.set_item("legal_actions_count", legal_actions.0.len())
                .unwrap();
            dict.set_item("acting_player", u8::from(acting_player))
                .unwrap();
            dict.into_py(py)
        });

        Ok((observation, current_reward, is_done, info))
    }

    /// Get the current observable game state for a player.
    ///
    /// Args:
    ///     player: The player to observe as (0 for Player1, 1 for Player2). Defaults to current acting player.
    ///
    /// Returns:
    ///     The observable game state.
    pub fn get_state(
        &self,
        player: Option<u8>,
    ) -> PyResult<super::gamestate_py::ObservableGameStatePy> {
        let player_enum = match player {
            Some(0) => GamePlayer::Player1,
            Some(1) => GamePlayer::Player2,
            Some(_) => return Err(PyValueError::new_err("Player must be 0 or 1")),
            None => self.game.game_state.acting_player,
        };
        Ok(super::gamestate_py::ObservableGameStatePy {
            game_state: self.game.game_state.observe(player_enum),
        })
    }

    // Get the actual game state
    pub fn get_game_state(&self) -> super::gamestate_py::GameStatePy {
        super::gamestate_py::GameStatePy {
            game_state: self.game.game_state.clone(),
        }
    }

    /// Get the legal actions for the current state.
    ///
    /// Returns:
    ///     The list of legal actions.
    pub fn get_legal_actions(&self) -> super::actions_py::ActionListPy {
        super::actions_py::ActionListPy(self.game.legal_actions())
    }

    /// Check if the game is over.
    ///
    /// Returns:
    ///     True if the game is over, False otherwise.
    pub fn is_done(&self) -> bool {
        self.game.is_over()
    }

    /// Get the rewards for both players.
    ///
    /// Returns:
    ///     Tuple of (player1_reward, player2_reward).
    pub fn get_rewards(&self) -> (f32, f32) {
        self.game.get_rewards()
    }

    /// Get the winner of the game, if any.
    ///
    /// Returns:
    ///     The winner (0 for Player1, 1 for Player2) or None if game is not over or tied.
    pub fn get_winner(&self) -> Option<u8> {
        self.game.get_winner().map(|p| u8::from(p))
    }

    /// Play a full game to completion.
    ///
    /// This method runs the game loop until completion, using the configured players.
    ///
    /// Returns:
    ///     Tuple of (player1_reward, player2_reward).
    pub fn play(&mut self) -> PyResult<(f32, f32)> {
        let p1 = self
            .player1
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("Player1 is not set"))?;

        let mut game_over = false;
        while !game_over {
            let pta = self.game.game_state.acting_player;
            let actions = self.game.legal_actions();
            let history = self.game.history.iter().map(|x| x.observe(pta)).collect();

            let action = match pta {
                GamePlayer::Player1 => {
                    p1.choose_action(self.game.game_state.observe(pta), actions.clone(), history)
                }
                GamePlayer::Player2 => match &mut self.player2 {
                    Some(p2_py) => p2_py.choose_action(
                        self.game.game_state.observe(pta),
                        actions.clone(),
                        history,
                    ),
                    None => self
                        .random_player2
                        .as_mut()
                        .ok_or_else(|| PyValueError::new_err("No player2 available"))?
                        .choose_action(self.game.game_state.observe(pta), actions.clone(), history),
                },
            };

            match self.game.step(action) {
                Ok(_) => (),
                Err(e) => {
                    return Err(PyValueError::new_err(format!(
                        "Error during game step: {}",
                        e
                    )));
                }
            };

            game_over = self.game.is_over();
        }
        Ok(self.game.get_rewards())
    }

    #[pyo3(signature = (file_path))]
    pub fn save_game(&self, file_path: Py<PyString>) -> PyResult<()> {
        self.game.save_game(&PathBuf::from(file_path.to_string()));
        Ok(())
    }

    /// Get the number of possible actions in the game.
    #[staticmethod]
    pub fn num_actions() -> u8 {
        num_actions()
    }

    /// Get the shape of the game state as a numpy array.
    #[staticmethod]
    pub fn state_shape() -> PyResult<Vec<usize>> {
        let game = Game::new();
        let state = game.game_state.observe(GamePlayer::Player1);
        match state.to_numpy() {
            Ok(arr) => Ok(arr.shape().to_vec()),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(format!(
                "Failed to get state shape: {}",
                e
            ))),
        }
    }
}
