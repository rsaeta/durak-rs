use rand::{thread_rng, Rng, RngCore};

use super::{
    actions::{Action, ActionList},
    gamestate::ObservableGameState,
};

pub trait Player {
    fn choose_action(
        &mut self,
        game_state: ObservableGameState,
        actions: ActionList,
        history: Vec<ObservableGameState>,
    ) -> Action;
}

pub struct RandomPlayer {
    rng: Box<dyn RngCore>,
}

impl RandomPlayer {
    pub fn new(_rng: Option<Box<dyn RngCore>>) -> RandomPlayer {
        match _rng {
            Some(rng) => RandomPlayer { rng },
            None => RandomPlayer {
                rng: Box::new(thread_rng()),
            },
        }
    }
}

impl Player for RandomPlayer {
    fn choose_action(
        &mut self,
        _state: ObservableGameState,
        actions: ActionList,
        _history: Vec<ObservableGameState>,
    ) -> Action {
        let choice = match actions.0.len() {
            0 => panic!("No actions available"),
            1 => 0,
            _ => self.rng.gen_range(0..actions.0.len()),
        };
        actions.0[choice]
    }
}
