"""
This file is a wrapper around the raw output from py03 to add type hints 
and whatnot to the classes
"""
from typing import List, Tuple
import numpy as np
from rust import (
    Card as _Card, 
    GameEnv as _GameEnv, 
    ObservableGameState as _ObservableGameState,
    ActionList as _ActionList
)


class Card:
    def __init__(self, card: _Card):
        self.card = card

    @property
    def suit(self) -> str:
        """Returns the suit of the card"""
        return self.card.suit

    @property
    def rank(self) -> int:
        """Returns the rank of the card"""
        return self.card.rank


class ObservableGameState:
    def __init__(self, state: _ObservableGameState):
        self.state = state

    @property
    def acting_player(self) -> int:
        """Returns the acting player"""
        return self.state.acting_player

    @property
    def player_hand(self) -> List[Card]:
        """Returns the player's hand"""
        return self.state.player_hand

    @property
    def attack_table(self) -> List[Card]:
        """Returns the attack table"""
        return self.state.attack_table

    @property
    def defense_table(self) -> List[Card]:
        """Returns the defense table"""
        return self.state.defense_table

    @property
    def deck_size(self) -> int:
        """Returns the deck size"""
        return self.state.deck_size

    @property
    def visible_card(self) -> Card:
        """Returns the visible card"""
        return self.state.visible_card

    @property
    def defender_has_taken(self) -> bool:
        """Returns whether the defender has taken"""
        return self.state.defender_has_taken

    @property
    def defender(self) -> int:
        """Returns the defender"""
        return self.state.defender

    @property
    def cards_in_opp_hand(self) -> int:
        """Returns the number of cards in opponent's hand"""
        return self.state.cards_in_opp_hand
    

    def __repr__(self) -> str:
        """Returns a string representation of the game state"""
        return self.state.__repr__()
    
    def __str__(self) -> str:
        """Returns a string representation of the game state"""
        return self.state.__repr__()

    def to_numpy(self) -> np.ndarray:
        """Converts the game state to a numpy array"""
        return self.state.to_numpy()


class ActionList:
    def __init__(self, action_list: _ActionList):
        self.action_list = action_list

    @property
    def actions(self) -> List[str]:
        """Returns the actions as a list of strings"""
        return self.action_list.get_actions()

    def to_indices(self) -> List[int]:
        """Returns the actions as a list of indices"""
        return self.action_list.to_indices()

    def to_bitmap(self) -> List[int]:
        """Returns the actions as a bitmap"""
        return self.action_list.to_bitmap()


class GamePlayer:
    def choose_action(self, state: ObservableGameState, actions: ActionList, history: List[ObservableGameState]) -> int:
        raise NotImplementedError("choose_action not implemented")


class GameEnv():
    def __init__(self, player: GamePlayer):
        self.env = _GameEnv(player)

    def play(self) -> Tuple[float, float]:
        return self.env.play()
    
    @staticmethod
    def state_shape() -> np.shape:
        """Returns the shape of the game state as a numpy array"""
        return _GameEnv.state_shape()
    
    @staticmethod
    def num_actions() -> int:
        """Returns the number of possible actions"""
        return _GameEnv.num_actions()


