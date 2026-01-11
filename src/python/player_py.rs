use pyo3::{exceptions::PyNotImplementedError, types::PyList, Py, PyAny, Python};
use pyo3::{pyclass, pymethods, IntoPy, PyErr, PyResult};

use crate::{
    game::{
        actions::{Action, ActionList},
        gamestate::ObservableGameState,
        player::Player,
    },
    ObservableGameStatePy,
};

use super::actions_py::ActionListPy;

/// Base class for game players that can be subclassed in Python.
/// Subclasses must implement the `choose_action` method.
#[pyclass(name = "GamePlayer", subclass)]
pub struct GamePlayerPy;

#[pymethods]
impl GamePlayerPy {
    #[new]
    fn new() -> Self {
        Self
    }

    /// Choose an action given the current game state and available actions.
    /// This method must be overridden by subclasses.
    fn choose_action(
        &self,
        _state: &ObservableGameStatePy,
        _actions: &ActionListPy,
        _history: &PyList,
    ) -> PyResult<u8> {
        Err(PyErr::new::<PyNotImplementedError, _>(
            "choose_action must be implemented by subclasses",
        ))
    }
}

/// Internal wrapper that implements the Rust Player trait for Python GamePlayer instances
pub struct PlayerPy(pub Py<GamePlayerPy>);

impl Player for PlayerPy {
    fn choose_action(
        &mut self,
        state: ObservableGameState,
        actions: ActionList,
        history: Vec<ObservableGameState>,
    ) -> Action {
        let state_py = ObservableGameStatePy { game_state: state };
        let actions_py = ActionListPy(actions.clone());
        let history_py: Vec<ObservableGameStatePy> = history
            .iter()
            .map(|x| ObservableGameStatePy {
                game_state: x.clone(),
            })
            .collect();

        let res = Python::with_gil(|py| -> u8 {
            let history_list = PyList::empty(py);
            for item in history_py {
                history_list
                    .append(item.into_py(py))
                    .expect("Failed to append history item to list");
            }
            let player_any: &PyAny = self.0.as_ref(py);
            let action_result = player_any
                .call_method1("choose_action", (state_py, actions_py, history_list))
                .expect("Failed to call choose_action on Python player");
            action_result
                .extract::<u8>()
                .expect("choose_action did not return a valid action index")
        });

        if res as usize >= actions.0.len() {
            panic!(
                "Python player returned invalid action index {} (max: {})",
                res,
                actions.0.len() - 1
            );
        }
        actions.0[res as usize]
    }
}
