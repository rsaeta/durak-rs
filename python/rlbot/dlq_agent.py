"""
Deep Learning Q-Network agent for the Durak game.
"""

from durak_rt import GameEnv, GamePlayer
import numpy as np
from typing import List, Optional


class DLQAgent(GamePlayer):
    def __init__(self, seed: Optional[int] = None):
        self.np_random = np.random.RandomState(seed)

    def choose_action(
        self,
        state: ObservableGameState,
        actions: ActionList,
        history: Optional[List[ObservableGameState]] = None,
    ) -> int:
        return self.np_random.randint(len(actions))
