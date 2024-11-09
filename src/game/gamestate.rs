use core::fmt;

use numpy::ndarray::{concatenate, Array1, Axis};

use super::{
    cards::{Card, Deck, Hand},
    utils::indices_to_bitmap_as_array1,
};

macro_rules! pub_struct {
  ($name:ident {$($field:ident: $t:ty,)*}) => {
      #[derive(Clone, PartialEq)] // ewww
      pub struct $name {
          $(pub $field: $t),*
      }
  }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum GamePlayer {
    Player1,
    Player2,
}

impl GamePlayer {
    pub fn other(&self) -> GamePlayer {
        match self {
            GamePlayer::Player1 => GamePlayer::Player2,
            GamePlayer::Player2 => GamePlayer::Player1,
        }
    }
}

// ignore unused variable for now
#[derive(Clone)]
pub struct ObservableGameState {
    pub player: GamePlayer,
    pub num_cards_in_deck: u8,
    pub attack_table: Vec<Card>,
    pub defense_table: Vec<Card>,
    pub hand: Hand,
    pub visible_card: Card,
    pub defender_has_taken: bool,
    pub acting_player: GamePlayer,
    pub defender: GamePlayer,
    pub cards_in_opponent: u8,
}

impl ObservableGameState {
    #[allow(dead_code)]
    pub fn to_numpy(self) -> Result<Array1<u8>, String> {
        let hand_arr = <Hand as Into<Array1<u8>>>::into(self.hand);
        let player_acting_arr = indices_to_bitmap_as_array1(vec![self.acting_player as usize], 2);
        let attack_table_arr = <Hand as Into<Array1<u8>>>::into(Hand(self.attack_table));
        let defense_table_arr = <Hand as Into<Array1<u8>>>::into(Hand(self.defense_table));
        let visible_card_arr =
            <Hand as Into<Array1<u8>>>::into(Hand(vec![<Card as Clone>::clone(
                &self.visible_card,
            )]));
        let defender_arr = indices_to_bitmap_as_array1(vec![self.defender as usize], 2);
        let defender_has_taken_arr = Array1::from_vec(vec![self.defender_has_taken as u8]);
        let deck_size_arr = Array1::from_vec(vec![self.num_cards_in_deck]);
        let cards_in_opp_arr = Array1::from_vec(vec![self.cards_in_opponent]);
        let cat = concatenate(
            numpy::ndarray::Axis(0),
            &[
                player_acting_arr.view(),
                hand_arr.view(),
                attack_table_arr.view(),
                defense_table_arr.view(),
                deck_size_arr.view(),
                visible_card_arr.view(),
                defender_has_taken_arr.view(),
                defender_arr.view(),
                cards_in_opp_arr.view(),
            ],
        );
        match cat {
            Ok(a) => Ok(a as Array1<u8>),
            Err(_e) => Err(String::from("Shape Error")),
        }
    }
}

pub_struct!(GameState {
    deck: Deck,
    attack_table: Vec<Card>,
    defense_table: Vec<Card>,
    hand1: Hand,
    hand2: Hand,
    acting_player: GamePlayer,
    defending_player: GamePlayer,
    visible_card: Card,
    defender_has_taken: bool,
    graveyard: Vec<Card>,
});

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\n\tDeck: {:?}\n\tAttack: {:?}\n\tDefense: {:?}\n\tHand1: {:?}\n\tHand2: {:?}\n\tActing: {:?}\n\tDefending: {:?}\n\tVisible: {:?}\n\tDefender has taken: {}\n\tGraveyard: {:?}\n}}",
            self.deck,
            self.attack_table,
            self.defense_table,
            self.hand1,
            self.hand2,
            self.acting_player,
            self.defending_player,
            self.visible_card,
            self.defender_has_taken,
            self.graveyard,
        )
    }
}

impl GameState {
    pub fn new(
        deck: Deck,
        attack_table: Vec<Card>,
        defense_table: Vec<Card>,
        hand1: Hand,
        hand2: Hand,
        acting_player: GamePlayer,
        defending_player: GamePlayer,
        visible_card: Card,
        defender_has_taken: bool,
        graveyard: Vec<Card>,
    ) -> GameState {
        GameState {
            deck,
            attack_table,
            defense_table,
            hand1,
            hand2,
            acting_player,
            defending_player,
            visible_card,
            defender_has_taken,
            graveyard,
        }
    }

    #[allow(dead_code)]
    pub fn to_numpy(&self) -> Array1<u8> {
        let deck_arr = <Deck as Into<Array1<u8>>>::into(self.deck.clone());
        let attack_table_arr = <Hand as Into<Array1<u8>>>::into(Hand(self.attack_table.clone()));
        let defense_table_arr = <Hand as Into<Array1<u8>>>::into(Hand(self.defense_table.clone()));
        let hand1_arr = <Hand as Into<Array1<u8>>>::into(self.hand1.clone());
        let hand2_arr = <Hand as Into<Array1<u8>>>::into(self.hand2.clone());
        let acting_player_arr = indices_to_bitmap_as_array1(vec![self.acting_player as usize], 2);
        let defending_player_arr =
            indices_to_bitmap_as_array1(vec![self.defending_player as usize], 2);
        let visible_card_arr = Array1::from_vec(vec![self.visible_card.into()]);
        let defender_has_taken_arr = Array1::from_vec(vec![self.defender_has_taken as u8]);
        let graveyard_arr = <Hand as Into<Array1<u8>>>::into(Hand(self.graveyard.clone()));

        concatenate(
            Axis(0),
            &[
                deck_arr.view(),
                attack_table_arr.view(),
                defense_table_arr.view(),
                hand1_arr.view(),
                hand2_arr.view(),
                acting_player_arr.view(),
                defending_player_arr.view(),
                visible_card_arr.view(),
                defender_has_taken_arr.view(),
                graveyard_arr.view(),
            ],
        )
        .unwrap()
    }

    pub fn observe(&self, player: GamePlayer) -> ObservableGameState {
        let hand = match player {
            GamePlayer::Player1 => self.hand1.clone(),
            GamePlayer::Player2 => self.hand2.clone(),
        };
        ObservableGameState {
            player,
            num_cards_in_deck: self.deck.len() as u8,
            attack_table: self.attack_table.clone(),
            defense_table: self.defense_table.clone(),
            hand,
            visible_card: self.visible_card.clone(),
            defender_has_taken: self.defender_has_taken,
            acting_player: self.acting_player.clone(),
            defender: self.defending_player.clone(),
            cards_in_opponent: match player {
                GamePlayer::Player1 => self.hand2.0.len() as u8,
                GamePlayer::Player2 => self.hand1.0.len() as u8,
            },
        }
    }

    pub fn num_undefended(&self) -> u8 {
        let num_attack = self.attack_table.len() as u8;
        let num_defend = self.defense_table.len() as u8;
        num_attack - num_defend
    }

    fn _defender_hand(&self) -> &Hand {
        match self.defending_player {
            GamePlayer::Player1 => &self.hand1,
            GamePlayer::Player2 => &self.hand2,
        }
    }

    fn _attacker_hand(&self) -> &Hand {
        match self.defending_player.other() {
            GamePlayer::Player1 => &self.hand1,
            GamePlayer::Player2 => &self.hand2,
        }
    }
}
