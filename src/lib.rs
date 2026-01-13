pub mod game;
pub mod server;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use python::{
    actions_py::ActionListPy,
    card_py::CardPy,
    env_py::GameEnvPy,
    gamestate_py::{ObservableGameHistoryPy, ObservableGameStatePy},
    player_py::GamePlayerPy,
};

#[cfg(feature = "python")]
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CardPy>()?;
    m.add_class::<GameEnvPy>()?;
    m.add_class::<ObservableGameStatePy>()?;
    m.add_class::<ActionListPy>()?;
    m.add_class::<GamePlayerPy>()?;
    m.add_class::<ObservableGameHistoryPy>()?;
    Ok(())
}
