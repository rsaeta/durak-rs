use pyo3::{Py, PyAny, Python};

use crate::{
    game::{
        actions::{Action, ActionList},
        gamestate::ObservableGameState,
        player::Player,
    },
    ObservableGameStatePy,
};

use super::actions_py::ActionListPy;

pub struct PyPlayer(pub Py<PyAny>);

impl Player for PyPlayer {
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

        let res = Python::with_gil(|py| {
            let action = (*self)
                .0
                .call_method(
                    py,
                    "choose_action",
                    (state_py, actions_py, history_py),
                    None,
                )
                .unwrap();
            action.extract::<u8>(py).unwrap()
        });
        actions.0[res as usize]
    }
}
