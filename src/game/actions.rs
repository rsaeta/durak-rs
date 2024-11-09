use super::cards::Card;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    StopAttack,
    Take,
    Attack(Card),
    Defend(Card),
}

pub fn num_actions() -> u8 {
    // one for take, one for stop attack, 36 attack, 36 defend
    1 + 1 + 36 + 36
}

impl From<Action> for u8 {
    fn from(action: Action) -> u8 {
        match action {
            Action::StopAttack => 0,
            Action::Take => 1,
            Action::Attack(c) => 2 + (<Card as Into<u8>>::into(c)),
            Action::Defend(c) => 38 + <Card as Into<u8>>::into(c),
        }
    }
}

impl From<u8> for Action {
    fn from(num: u8) -> Action {
        match num {
            0 => Action::StopAttack,
            1 => Action::Take,
            2..=37 => Action::Attack(Card::from(num - 2)),
            38..=73 => Action::Defend(Card::from(num - 38)),
            _ => panic!("Invalid action number"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ActionList(pub Vec<Action>);

#[allow(dead_code)]
impl ActionList {
    #[allow(dead_code)]
    pub fn to_strings(&self) -> Vec<String> {
        self.0.iter().map(|a| format!("{:?}", a)).collect()
    }

    #[allow(dead_code)]
    pub fn to_u8s(&self) -> Vec<u8> {
        self.0.iter().map(|&a| u8::from(a)).collect()
    }

    #[allow(dead_code)]
    pub fn to_bitmap(&self) -> Vec<bool> {
        let mut bitmap = vec![false; num_actions() as usize];
        for action in &self.0 {
            bitmap[<Action as Into<u8>>::into(*action) as usize] = true;
        }
        bitmap
    }

    #[allow(dead_code)]
    pub fn from_bitmap(bitmap: Vec<bool>) -> Self {
        let actions = bitmap
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b { Some(Action::from(i as u8)) } else { None })
            .collect();
        ActionList(actions)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::cards::Suit;

    use super::*;

    pub fn get_all_actions() -> Vec<Action> {
        let mut actions = vec![Action::StopAttack, Action::Take];
        // Generate all possible cards for Attack and Defend actions
        // Assuming a standard deck of cards with 4 suits and 9 ranks
        // Suits are represented by numbers 0 to 3 and ranks by numbers 0 to 8
        for suit in 0..4 {
            for rank in 0..9 {
                actions.push(Action::Attack(Card {
                    suit: Suit::from(suit),
                    rank: rank + 6,
                }));
            }
        }
        for suit in 0..4 {
            for rank in 0..9 {
                actions.push(Action::Defend(Card {
                    suit: Suit::from(suit),
                    rank: rank + 6,
                }));
            }
        }
        actions
    }

    #[test]
    fn test_action_to_u8_and_back() {
        let actions = get_all_actions();

        for action in actions {
            let num = u8::from(action);
            let action_back = Action::from(num);
            assert_eq!(action, action_back);
        }
    }

    #[test]
    fn test_to_from_bitmaps() {
        let actions = get_all_actions();
        let action_list = ActionList(actions.clone());
        let bitmap = action_list.to_bitmap();
        let action_list_from_bitmap = ActionList::from_bitmap(bitmap);

        assert_eq!(action_list, action_list_from_bitmap);

        let actions_back = action_list_from_bitmap.0;
        assert_eq!(actions, actions_back);
    }
}
