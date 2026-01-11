// This file will hold experience replay for Durak-RS.

use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game::{
    actions::Action,
    gamestate::{GameState, ObservableGameState},
};

pub struct ExperienceReplay {
    save_file: PathBuf,
    pub experience: Vec<Experience>,
}
pub struct Experience {
    pub state: GameState,
    pub action: Action,
    pub reward: f32,
    pub next_state: ObservableGameState,
}

pub struct SampleExperience {
    pub experiences: Vec<Experience>,
    // random state number generator
    rng: StdRng,
    prob_full_history: f64,
}

impl ExperienceReplay {
    pub fn new(save_dir: &PathBuf) -> Self {
        Self {
            experience: vec![],
            save_file: save_dir.join("experience.json"),
        }
    }

    pub fn add_experience(&mut self, experience: Experience) {
        self.experience.push(experience);
    }
}

impl SampleExperience {
    pub fn new(experiences: Vec<Experience>, seed: u64, prob_full_history: f64) -> Self {
        Self {
            experiences,
            rng: StdRng::seed_from_u64(seed),
            prob_full_history,
        }
    }
}
