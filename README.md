# Durak-RS

A high-performance Rust implementation of the Durak card game with Python bindings, designed for AI/ML research and development.

## Features

- **High Performance**: Core game logic implemented in Rust for maximum speed
- **Python-Friendly API**: Clean, intuitive Python interface for easy AI player development
- **Gym-like Interface**: Step-by-step API similar to OpenAI Gym for reinforcement learning
- **Flexible Player System**: Easy to implement custom AI players in Python
- **Type Hints**: Full type annotations for better IDE support and developer experience

## Installation

```bash
# Install from source
pip install -e .

# Or build with maturin
maturin develop
```

## Quick Start

### Basic Usage

```python
from durak_rt import GameEnv, GamePlayer
import numpy as np

class MyPlayer(GamePlayer):
    def choose_action(self, state, actions, history=None):
        # Your AI logic here
        return np.random.randint(len(actions))

# Create environment with your player
env = GameEnv(MyPlayer())

# Play a full game
rewards = env.play()
print(f"Rewards: {rewards}")
```

### Step-by-Step API (Gym-like)

```python
from durak_rt import GameEnv, GamePlayer

class SimplePlayer(GamePlayer):
    def choose_action(self, state, actions, history=None):
        return 0  # Choose first action

env = GameEnv(SimplePlayer())

# Reset to initial state
state = env.reset()

# Step through the game
while not env.is_done():
    legal_actions = env.get_legal_actions()
    action_idx = 0  # Your action selection logic
    
    observation, reward, done, info = env.step(action_idx)
    
    if done:
        break

# Get final results
rewards = env.get_rewards()
winner = env.get_winner()
```

### Two Custom Players

```python
from durak_rt import GameEnv, GamePlayer

class Player1(GamePlayer):
    def choose_action(self, state, actions, history=None):
        # Player 1 logic
        return 0

class Player2(GamePlayer):
    def choose_action(self, state, actions, history=None):
        # Player 2 logic
        return 0

# Create environment with both players
env = GameEnv(Player1(), player2=Player2())
rewards = env.play()
```

## API Reference

### GameEnv

The main game environment class.

#### Methods

- `__init__(player1, player2=None, seed=None)`: Create a new game environment
  - `player1`: Required. A `GamePlayer` instance
  - `player2`: Optional. A `GamePlayer` instance. If None, uses a random player
  - `seed`: Optional. Random seed for reproducibility

- `reset(seed=None)`: Reset the game to initial state
  - Returns: Initial observable game state

- `step(action_index)`: Execute one game step
  - `action_index`: Index of action from `get_legal_actions()`
  - Returns: `(observation, reward, done, info)` tuple

- `get_state(player=None)`: Get current observable game state
  - `player`: Optional player index (0 or 1). Defaults to current acting player
  - Returns: Observable game state

- `get_legal_actions()`: Get list of legal actions for current state
  - Returns: `ActionList` object

- `is_done()`: Check if game is over
  - Returns: `True` if game is finished

- `get_rewards()`: Get rewards for both players
  - Returns: `(player1_reward, player2_reward)` tuple

- `get_winner()`: Get the winner (if game is over)
  - Returns: `0` for Player 1, `1` for Player 2, or `None` if no winner

- `play()`: Play a full game to completion
  - Returns: `(player1_reward, player2_reward)` tuple

#### Static Methods

- `num_actions()`: Get total number of possible actions
- `state_shape()`: Get shape of game state numpy array

### GamePlayer

Base class for implementing game players. Subclass this and implement `choose_action`.

#### Methods

- `choose_action(state, actions, history=None)`: Choose an action
  - `state`: Current `ObservableGameState`
  - `actions`: `ActionList` of available actions
  - `history`: Optional list of previous game states
  - Returns: Index of chosen action (integer)

### ObservableGameState

Represents the game state from a player's perspective.

#### Properties

- `acting_player`: Current acting player (0 or 1)
- `player_hand`: List of cards in player's hand
- `attack_table`: List of cards on attack table
- `defense_table`: List of cards on defense table
- `deck_size`: Number of cards remaining in deck
- `visible_card`: The visible trump card
- `defender_has_taken`: Whether defender has taken cards
- `defender`: Current defender (0 or 1)
- `cards_in_opp_hand`: Number of cards in opponent's hand

#### Methods

- `to_numpy()`: Convert state to numpy array for ML models

### ActionList

Container for available actions.

#### Properties

- `actions`: List of action strings (e.g., `["Attack(6♠)", "StopAttack"]`)

#### Methods

- `to_indices()`: Get action indices
- `to_bitmap()`: Get action bitmap as numpy array
- `__len__()`: Number of available actions
- `__getitem__(index)`: Get action at index

## Examples

See `python/durak_rt/examples.py` for comprehensive examples including:

- `RandomPlayer`: Random action selection
- `GreedyPlayer`: Simple heuristic-based player
- `HumanPlayer`: Interactive player for testing
- Step-by-step game loop examples
- Full game examples

## Project Structure

```
durak-rs/
├── src/
│   ├── game/          # Core game logic (Rust)
│   │   ├── actions.rs
│   │   ├── cards.rs
│   │   ├── game.rs
│   │   ├── gamestate.rs
│   │   └── player.rs
│   └── python/         # Python bindings (PyO3)
│       ├── env_py.rs
│       ├── player_py.rs
│       ├── actions_py.rs
│       └── gamestate_py.rs
├── python/
│   └── durak_rt/      # Python package
│       ├── __init__.py
│       ├── examples.py
│       └── ...
├── Cargo.toml
└── pyproject.toml
```

## Development

### Building

```bash
# Development build
maturin develop

# Release build
maturin build --release
```

### Running Tests

```bash
# Rust tests
cargo test

# Python tests (if available)
pytest
```

## Performance Considerations

- The core game logic runs in Rust for maximum performance
- Python callbacks (player actions) have minimal overhead
- State observations are efficiently converted to numpy arrays
- Consider batching multiple games for training AI models

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust and Python style guidelines
2. All tests pass
3. Documentation is updated
4. Type hints are included for Python code

## License

[Add your license here]

## Acknowledgments

Built with [PyO3](https://pyo3.rs/) for Python-Rust interop.
