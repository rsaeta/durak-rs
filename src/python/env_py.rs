use crate::game::actions::num_actions;
use crate::game::game::{Game, GameLogic};
use crate::game::gamestate::GamePlayer;
use crate::game::player::{Player, RandomPlayer};
use crate::python::player_py::PyPlayer;
use pyo3::{pyclass, pymethods, Py, PyAny, PyResult};

#[pyclass(name = "GameEnv", unsendable)]
pub struct GameEnvPy {
    game: Box<Game>,
    player1: Box<PyPlayer>,
}

impl Into<GamePlayer> for u8 {
    fn into(self) -> GamePlayer {
        match self {
            1 => GamePlayer::Player1,
            2 => GamePlayer::Player2,
            _ => panic!("Invalid player number"),
        }
    }
}

#[pymethods]
impl GameEnvPy {
    #[new]
    pub fn new(player1: Py<PyAny>) -> Self {
        GameEnvPy {
            game: Box::new(Game::new()),
            player1: Box::new(PyPlayer(player1)),
        }
    }

    #[staticmethod]
    pub fn num_actions() -> u8 {
        num_actions()
    }

    #[staticmethod]
    pub fn state_shape() -> PyResult<Vec<usize>> {
        let game = Game::new();
        let state = game.game_state.observe(GamePlayer::Player1);
        Ok(state.to_numpy().unwrap().shape().to_vec())
    }

    pub fn play(&mut self) -> PyResult<(f32, f32)> {
        let mut p2 = Box::new(RandomPlayer::new(None)) as Box<dyn Player>;
        let p1 = &mut self.player1; // Box::new(PyPlayer(player1)) as Box<dyn Player>;
        let mut game_over = false;
        while !game_over {
            let pta = self.game.game_state.acting_player;
            let actions = self.game.legal_actions();
            let player = match pta {
                GamePlayer::Player1 => p1.as_mut(),
                GamePlayer::Player2 => p2.as_mut(),
            };
            let history = self.game.history.iter().map(|x| x.observe(pta)).collect();
            let action = player.choose_action(self.game.game_state.observe(pta), actions, history);
            match self.game.step(action) {
                Ok(_) => (),
                Err(_e) => (),
            };

            game_over = self.game.is_over();
        }
        Ok(self.game.get_rewards())
    }
}
