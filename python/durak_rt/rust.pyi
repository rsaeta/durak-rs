"""
This file is a wrapper around the raw output from py03 to add type hints 
and whatnot to the classes
"""
from typing import List, Tuple
import numpy as np

class Card:
    rank: int
    suit: int
    def __init__(self, rank: int, suit: int) -> None: ...

class ObservableGameState:
    @property
    def acting_player(self) -> int:
        """Returns the acting player"""
        ...

    @property
    def player_hand(self) -> List[Card]:
        """Returns the player's hand"""
        ...

    @property
    def attack_table(self) -> List[Card]:
        """Returns the attack table"""
        ...

    @property
    def defense_table(self) -> List[Card]:
        """Returns the defense table"""
        ...

    @property
    def deck_size(self) -> int:
        """Returns the deck size"""
        ...

    @property
    def visible_card(self) -> Card:
        """Returns the visible card"""
        ...

    @property
    def defender_has_taken(self) -> bool:
        """Returns whether the defender has taken"""
        ...

    @property
    def defender(self) -> int:
        """Returns the defender"""
        ...

    @property
    def cards_in_opp_hand(self) -> int:
        """Returns the number of cards in opponent's hand"""
        ...

    def __repr__(self) -> str:
        """Returns a string representation of the game state"""
        ...
    
    def __str__(self) -> str:
        """Returns a string representation of the game state"""
        ...

    def to_numpy(self) -> np.ndarray:
        """Converts the game state to a numpy array"""
        ...

class ActionList:
  
    def actions(self) -> List[str]:
        """Returns the actions as a list of strings"""
        ...

    def to_indices(self) -> List[int]:
        """Returns the actions as a list of indices"""
        ...

    def to_bitmap(self) -> np.ndarray:
        """Returns the actions as a bitmap"""
        ...

    def __len__(self) -> int:
        """Returns the number of actions"""
        ...
    
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __getitem__(self, index: int) -> str: ...


class GamePlayer:
    def choose_action(self, state: ObservableGameState, actions: ActionList, history: List[ObservableGameState]) -> int:
        ...


class GameEnv:
    def play(self) -> Tuple[float, float]:
        ...
    
    @staticmethod
    def state_shape() -> np.shape:
        """Returns the shape of the game state as a numpy array"""
        ...
    
    @staticmethod
    def num_actions() -> int:
        """Returns the number of possible actions"""
        ...


