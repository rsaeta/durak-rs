use pyo3::prelude::*;
use python::{
    actions_py::ActionListPy, card_py::CardPy, env_py::GameEnvPy,
    gamestate_py::ObservableGameStatePy,
};
mod game;
mod python;

#[pymodule]
#[pyo3(name = "rust")]
pub fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CardPy>()?;
    m.add_class::<GameEnvPy>()?;
    m.add_class::<ObservableGameStatePy>()?;
    m.add_class::<ActionListPy>()?;
    Ok(())
}
