"""
Example script demonstrating basic usage of the Durak game environment.

This is a simple example - see examples.py for more comprehensive examples.
"""

from durak_rt import GameEnv
from durak_rt.examples import RandomPlayer


def main():
    """Run a simple game example."""
    # Create a random player
    player = RandomPlayer(seed=42)

    # Create game environment (player2 will be random by default)
    env = GameEnv(player)

    # Play a full game
    print("Playing a game...")
    rewards = env.play()

    print(f"\nGame finished!")
    print(f"Player 1 reward: {rewards[0]}")
    print(f"Player 2 reward: {rewards[1]}")

    env.save_game("game.json")
    breakpoint()
    env2 = GameEnv.from_file("game.json")


if __name__ == "__main__":
    main()
