"""
Durak game runtime package.

This package provides a high-performance Rust implementation of the Durak card game
exposed to Python, designed for AI/ML research and development.

Example:
    ```python
    from durak_rt import GameEnv, GamePlayer
    import numpy as np

    class MyPlayer(GamePlayer):
        def choose_action(self, state, actions, history=None):
            # Your AI logic here
            return np.random.randint(len(actions))

    env = GameEnv(MyPlayer())
    rewards = env.play()
    ```
"""

from .rust import GameEnv, GamePlayer, ObservableGameState, ActionList, Card

__version__ = "0.1.0"
__all__ = ["GameEnv", "GamePlayer", "ObservableGameState", "ActionList", "Card"]
