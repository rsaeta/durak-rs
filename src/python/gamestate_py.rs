use numpy::PyArray1;
use numpy::{Ix1, Ix2, PyArray, PyArray2};
use pyo3::exceptions::PyException;
use pyo3::{pyclass, pymethods, PyErr, PyResult, Python};

use crate::game::cards::Card;
use crate::game::gamestate::{GamePlayer, GameState, ObservableGameHistory, ObservableGameState};

use super::card_py::CardPy;

#[pyclass(name = "ObservableGameState")]
pub struct ObservableGameStatePy {
    pub game_state: ObservableGameState,
}

#[pyclass(name="ObservableGameHistory")]
pub struct ObservableGameHistoryPy {
    pub history: ObservableGameHistory,
}

#[pymethods]
impl ObservableGameHistoryPy {
  pub fn __repr__(&self) -> PyResult<String> {
    Ok(format!("ObservableGameHistory: {:?}", self.history))
  }

  pub fn __str__(&self) -> PyResult<String> {
    Ok(format!("ObservableGameHistory: {:?}", self.history))
  }

  pub fn to_numpy(&self) -> PyResult<pyo3::Py<PyArray<u8, Ix2>>> {
    match self.history.clone().to_numpy() {
      Ok(array) => Ok(Python::with_gil(|py| {
        PyArray2::from_array(py, &array).to_owned()
      })),
      Err(e) => Err(PyErr::new::<PyException, _>(e))
    }
  }
}

#[pyclass(name = "GameState")]
pub struct GameStatePy {
    pub game_state: GameState,
}

#[pyclass(name = "GamePlayer")]
pub struct GamePlayerPy {
    #[pyo3(get)]
    pub player: u8,
}

impl From<GamePlayer> for u8 {
    fn from(player: GamePlayer) -> Self {
        match player {
            GamePlayer::Player1 => 0,
            GamePlayer::Player2 => 1,
        }
    }
}

fn get_cards_py(cards: Vec<Card>) -> Vec<CardPy> {
    cards.iter()
        .map(|c| CardPy{ card: *c })
        .collect()
}

#[pymethods]
impl ObservableGameStatePy {
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "GameState(\n  Acting player: {},\n  Player hand: {:?},\n  Attack table: {:?},\n  Defense table: {:?},\n  Deck size: {},\n  Visible card: {:?},\n  Defender has taken: {},\n  Defender: {},\n  Cards in opponent's hand: {}\n)",
            u8::from(self.game_state.acting_player), 
            self.game_state.hand, 
            self.game_state.attack_table, 
            self.game_state.defense_table, 
            self.game_state.num_cards_in_deck, 
            self.game_state.visible_card, 
            self.game_state.defender_has_taken, 
            u8::from(self.game_state.defender), 
            self.game_state.cards_in_opponent
        ))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "GameState(\n  Acting player: {},\n  Player hand: {:?},\n  Attack table: {:?},\n  Defense table: {:?},\n  Deck size: {},\n  Visible card: {:?},\n  Defender has taken: {},\n  Defender: {},\n  Cards in opponent's hand: {}\n)",
            u8::from(self.game_state.acting_player), 
            self.game_state.hand, 
            self.game_state.attack_table, 
            self.game_state.defense_table, 
            self.game_state.num_cards_in_deck, 
            self.game_state.visible_card, 
            self.game_state.defender_has_taken, 
            u8::from(self.game_state.defender), 
            self.game_state.cards_in_opponent
        ))
    }
    #[getter]
    fn get_acting_player(&self) -> PyResult<u8> {
        Ok(u8::from(self.game_state.acting_player))
    }

    #[getter]
    fn get_player_hand(&self) -> PyResult<Vec<CardPy>> {
        Ok(get_cards_py(self.game_state.hand.0.clone()))
    }

    #[getter]
    fn get_attack_table(&self) -> PyResult<Vec<CardPy>> {
        Ok(get_cards_py(self.game_state.attack_table.clone()))
    }

    #[getter]
    fn get_defense_table(&self) -> PyResult<Vec<CardPy>> {
        Ok(get_cards_py(self.game_state.defense_table.clone()))
    }

    #[getter]
    fn get_deck_size(&self) -> PyResult<u8> {
        Ok(self.game_state.num_cards_in_deck)
    }

    #[getter]
    fn get_visible_card(&self) -> PyResult<CardPy> {
        Ok(CardPy { card: self.game_state.visible_card })
    }

    #[getter]
    fn get_defender_has_taken(&self) -> PyResult<bool> {
        Ok(self.game_state.defender_has_taken)
    }

    #[getter]
    fn get_defender(&self) -> PyResult<u8> {
        Ok(u8::from(self.game_state.defender))
    }

    #[getter]
    fn get_cards_in_opp_hand(&self) -> PyResult<u8> {
        Ok(self.game_state.cards_in_opponent)
    }

    pub fn to_numpy(&self) -> PyResult<pyo3::Py<PyArray<u8, Ix1>>> {
        match self.game_state.clone().to_numpy() {
            Ok(a) => Ok(Python::with_gil(|py| {
                PyArray1::from_array(py, &a).to_owned()
            })),
            Err(s) => Err(PyErr::new::<PyException, _>(s))
        }
    }
}

#[pymethods]
impl GameStatePy {
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("GameState: {:?}", self.game_state))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("GameState: {:?}", self.game_state))
    }

    pub fn to_numpy(&self) -> PyResult<pyo3::Py<PyArray<u8, Ix1>>> {
        Ok(Python::with_gil(|py| {
            PyArray1::from_array(py, &self.game_state.to_numpy()).to_owned()
        }))
    }
}