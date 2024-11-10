from rust import GameEnv, GamePlayer
import numpy as np


class RandomPlayer(GamePlayer):
    def __init__(self):
        self.np_random = np.random.RandomState()

    def choose_action(self, state, actions, full_state=None):
        print(f"Actions: {actions}")
        print(f"State: {state}")
        breakpoint()
        choice = self.np_random.choice(len(actions.actions))
        print(f"Chose action: {actions[choice]}")
        return choice


class HumanPlayer(GamePlayer):
    def choose_action(self, state, actions, full_state=None):
        print("State:")
        print(state)
        sorted_actions = sorted(actions)
        print("Actions: {}".format(sorted_actions))
        action = -1
        while action not in range(len(actions)):
            try:
                action = int(input("Choose action: "))
            except ValueError:
                action = -1
            if action not in range(len(actions)):
                print("Invalid action {}".format(action))
        return actions.index(sorted_actions[action])


env = GameEnv(RandomPlayer())
(a, b) = env.play()
breakpoint()
