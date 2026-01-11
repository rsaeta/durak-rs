"""
Example player implementations for the Durak game.

This module provides several example player implementations that can be used
as starting points for developing AI players.
"""

from durak_rt import GameEnv, GamePlayer, ObservableGameState, ActionList
import numpy as np
from typing import List, Optional


class RandomPlayer(GamePlayer):
    """A player that chooses actions randomly."""

    def __init__(self, seed: Optional[int] = None):
        """Initialize a random player.

        Args:
            seed: Optional random seed for reproducibility.
        """
        self.np_random = np.random.RandomState(seed)

    def choose_action(
        self,
        state: ObservableGameState,
        actions: ActionList,
        history: Optional[List[ObservableGameState]] = None,
    ) -> int:
        """Choose a random action from the available actions.

        Args:
            state: The current observable game state.
            actions: The list of available actions.
            history: Optional history of previous game states.

        Returns:
            The index of the chosen action.
        """
        if len(actions) == 0:
            raise ValueError("No actions available")
        return self.np_random.randint(len(actions))


class GreedyPlayer(GamePlayer):
    """A player that tries to minimize cards in hand by playing aggressively."""

    def choose_action(
        self,
        state: ObservableGameState,
        actions: ActionList,
        history: Optional[List[ObservableGameState]] = None,
    ) -> int:
        """Choose an action greedily.

        Prefers:
        1. Attacking with high-value cards
        2. Defending when possible
        3. Taking cards only as last resort

        Args:
            state: The current observable game state.
            actions: The list of available actions.
            history: Optional history of previous game states.

        Returns:
            The index of the chosen action.
        """
        if len(actions) == 0:
            raise ValueError("No actions available")

        action_strings = actions.actions

        # Prefer defending over taking
        if "Take" in action_strings:
            take_idx = action_strings.index("Take")
            # Only take if no other options
            if len(action_strings) > 1:
                # Return first non-Take action
                for i, action_str in enumerate(action_strings):
                    if action_str != "Take":
                        return i
            return take_idx

        # Prefer StopAttack if available and we have few cards
        if "StopAttack" in action_strings and len(state.player_hand) <= 3:
            return action_strings.index("StopAttack")

        # Otherwise, choose the first available action (could be improved with card ranking)
        return 0


class HumanPlayer(GamePlayer):
    """A player that prompts for human input."""

    def choose_action(
        self,
        state: ObservableGameState,
        actions: ActionList,
        history: Optional[List[ObservableGameState]] = None,
    ) -> int:
        """Prompt the user to choose an action.

        Args:
            state: The current observable game state.
            actions: The list of available actions.
            history: Optional history of previous game states.

        Returns:
            The index of the chosen action.
        """
        print("\n" + "=" * 50)
        print("Current Game State:")
        print(f"  Acting Player: {state.acting_player}")
        print(f"  Your Hand: {state.player_hand}")
        print(f"  Attack Table: {state.attack_table}")
        print(f"  Defense Table: {state.defense_table}")
        print(f"  Deck Size: {state.deck_size}")
        print(f"  Opponent Cards: {state.cards_in_opp_hand}")
        print("=" * 50)

        print("\nAvailable Actions:")
        sorted_actions = sorted(actions.actions)
        for i, action in enumerate(sorted_actions):
            print(f"  {i}: {action}")

        while True:
            try:
                choice = int(input("\nChoose action (by sorted index): "))
                if 0 <= choice < len(sorted_actions):
                    # Find the original index
                    chosen_action = sorted_actions[choice]
                    return actions.actions.index(chosen_action)
                else:
                    print(
                        f"Invalid choice. Please enter a number between 0 and {len(sorted_actions) - 1}"
                    )
            except ValueError:
                print("Invalid input. Please enter a number.")
            except KeyboardInterrupt:
                print("\nGame interrupted by user.")
                raise


def example_step_by_step():
    """Example of using the step-by-step API."""
    print("=== Step-by-Step Game Example ===\n")

    class SimplePlayer(GamePlayer):
        def choose_action(self, state, actions, history=None):
            return 0  # Always choose first action

    env = GameEnv(SimplePlayer())

    # Reset the game
    state = env.reset()
    print(f"Initial state - Acting player: {state.acting_player}")

    step_count = 0
    while not env.is_done() and step_count < 100:  # Safety limit
        legal_actions = env.get_legal_actions()
        print(f"\nStep {step_count + 1}: {len(legal_actions)} legal actions")

        # Choose first action
        action_idx = 0
        if len(legal_actions) > 0:
            observation, reward, done, info = env.step(action_idx)
            print(f"  Reward: {reward}, Done: {done}")
            if done:
                break
        else:
            print("  No legal actions available!")
            break

        step_count += 1

    rewards = env.get_rewards()
    winner = env.get_winner()
    print(f"\nGame finished!")
    print(f"Final rewards: Player1={rewards[0]}, Player2={rewards[1]}")
    if winner is not None:
        print(f"Winner: Player {winner + 1}")


def example_full_game():
    """Example of playing a full game."""
    print("=== Full Game Example ===\n")

    player1 = RandomPlayer(seed=42)
    player2 = RandomPlayer(seed=43)

    env = GameEnv(player1, player2=player2)
    rewards = env.play()

    print(f"Game finished!")
    print(f"Player 1 reward: {rewards[0]}")
    print(f"Player 2 reward: {rewards[1]}")

    if rewards[0] > rewards[1]:
        print("Player 1 wins!")
    elif rewards[1] > rewards[0]:
        print("Player 2 wins!")
    else:
        print("It's a tie!")


if __name__ == "__main__":
    # Run examples
    example_step_by_step()
    print("\n" + "=" * 50 + "\n")
    example_full_game()
