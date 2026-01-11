"""
This module contains the Experience Replay class for the Durak game.
"""

from os import PathLike
from pathlib import Path
from durak_rt import GameEnv, GamePlayer, ObservableGameState, ActionList
import numpy as np
from typing import List, Optional


class ExperienceReplay:
    def __init__(self, save_dir: PathLike):
        self.save_dir = Path(save_dir)
        self.save_dir.mkdir(parents=True, exist_ok=True)
        self.experience_file = self.save_dir / "experience.npz"
        self.experience_file.touch()

    def save_experience(
        self,
        experience: List[
            Tuple[ObservableGameState, ActionList, float, ObservableGameState]
        ],
    ):
        with np.load(self.experience_file, allow_pickle=True) as data:
            data["experience"] = experience
        np.savez(self.experience_file, experience=experience)

    def load_experience(self):
        with np.load(self.experience_file, allow_pickle=True) as data:
            return data["experience"]
