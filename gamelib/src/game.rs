use crate::*;
use rand::prelude::*;

#[derive(PartialEq, Debug)]
enum GameState {
    CREATED,
    STARTED,
    FINISHED,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    PLAY,
    DRAW,
    SKIP,
}

pub struct Game<'g> {
    state: GameState,
    cards: Vec<Card<'g>>,
    pub(crate) deck: Vec<u32>,
    pub(crate) pile: Vec<u32>,
    pub players: Vec<Player>,
    current_player_id: u32,
}

impl<'g> Game<'g> {
    pub fn new(player_names: Vec<&str>) -> Option<Game> {
        let players: Vec<Player> = player_names
            .iter()
            .enumerate()
            .map(|(i, name)| Player::new((i + 1) as u32, name.to_string()))
            .collect();

        if players.len() > 1 {
            Some(Game {
                state: GameState::CREATED,
                cards: Card::load_deck(),
                deck: Vec::new(),
                pile: Vec::new(),
                current_player_id: players.first()?.id,
                players,
            })
        } else {
            None
        }
    }

    pub fn setup(&mut self) {
        if self.state != GameState::CREATED {
            println!("Incorrect game state");
            return;
        }

        self.deck = self.cards.iter().map(|c| c.id).collect();
        self.deck.shuffle(&mut thread_rng());
        for player in &mut self.players {
            for _ in 0..7 {
                let card_id = self.deck.pop().unwrap();
                player.hand.push(card_id);
            }
        }
        self.pile.push(self.deck.pop().unwrap());

        self.state = GameState::STARTED;
    }

    pub fn available_actions(&self) -> Vec<Action> {
        match self.current_player().has_drawn {
            true => vec![Action::PLAY, Action::SKIP],
            false => vec![Action::PLAY, Action::DRAW],
        }
    }

    pub fn action(&mut self, action: Action, target_id: Option<u32>) -> Option<()> {
        if self.state != GameState::STARTED {
            println!("Incorrect game state");
            return None;
        }
        let mut player = self
            .players
            .iter_mut()
            .find(|p| p.id == self.current_player_id)?;
        match action {
            Action::PLAY => {
                let target_id = target_id?;
                let target_card = self.cards.iter().find(|c| c.id == target_id)?;
                let top_pile_id = *self.pile.last()?;
                let top_card = self.cards.iter().find(|c| c.id == top_pile_id)?;
                if target_card.color != top_card.color && target_card.value != top_card.value {
                    return None;
                }
                let card = player
                    .hand
                    .remove(player.hand.iter().position(|cid| *cid == target_id)?);
                self.pile.push(card);
                player.has_played = true;
            }
            Action::DRAW => {
                if self.deck.len() == 0 {
                    self.deck.append(&mut self.pile);
                    self.deck.shuffle(&mut thread_rng());
                }
                let card = self.deck.pop()?;
                player.hand.push(card);
                player.has_drawn = true;
            }
            Action::SKIP => {
                if player.has_drawn {
                    player.has_played = true;
                } else {
                    return None;
                }
            }
        }
        Some(())
    }

    pub fn next(&mut self) -> bool {
        if self.state != GameState::STARTED {
            println!("Incorrect state");
            return false;
        }
        if self.current_player().has_played {
            if self.current_player().hand.len() == 0 {
                self.state = GameState::FINISHED;
            } else {
                self.next_player();
            }
            true
        } else {
            false
        }
    }

    fn current_player(&self) -> &Player {
        &self
            .players
            .iter()
            .find(|player| player.id == self.current_player_id)
            .unwrap()
    }

    fn next_player(&mut self) {
        let current_player = self
            .players
            .iter_mut()
            .find(|p| p.id == self.current_player_id)
            .unwrap();
        current_player.has_drawn = false;
        let current_player_index = self
            .players
            .iter()
            .position(|player| player.id == self.current_player_id)
            .unwrap();
        let next_player_index = (current_player_index + 1) % self.players.len();
        self.current_player_id = self.players[next_player_index].id;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_game_not_enough_players() {
        let result = Game::new(vec!["Aaron"]);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_new_game() {
        let game = Game::new(vec!["Aaron", "Bea"]).unwrap();
        assert_eq!(game.state, GameState::CREATED);
        assert_eq!(game.deck.len(), 0);
        assert_eq!(game.pile.len(), 0);
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.current_player_id, game.players.first().unwrap().id);
    }

    #[test]
    fn test_current_player() {
        let game = Game::new(vec!["Aaron", "Bea"]).unwrap();
        let result = game.current_player();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Aaron");
        assert_eq!(result.hand.len(), 0);
    }

    #[test]
    fn test_next_player() {
        let mut game = Game::new(vec!["Aaron", "Bea", "Cindy"]).unwrap();
        game.next_player();
        assert_eq!(game.current_player_id, 2);
        game.next_player();
        assert_eq!(game.current_player_id, 3);
        game.next_player();
        assert_eq!(game.current_player_id, 1);
    }

    #[test]
    fn test_setup() {
        let mut game = Game::new(vec!["Aaron", "Bea"]).unwrap();
        game.setup();
        assert_eq!(game.deck.len(), 40 - 15);
        assert_eq!(game.pile.len(), 1);
        assert_eq!(game.players.first().unwrap().hand.len(), 7);
        assert_eq!(game.players.last().unwrap().hand.len(), 7);
        assert_eq!(game.state, GameState::STARTED);
    }

    #[test]
    fn test_available_actions() {
        let mut game = Game::new(vec!["Aaron", "Bea"]).unwrap();
        game.setup();
        let result = game.available_actions();
        assert_eq!(result.len(), 2);
        assert_eq!(result, vec![Action::PLAY, Action::DRAW]);
    }

    #[test]
    fn test_action_play_when_wrong_card_id() {
        let mut game = helper_create_game(1);
        let result = game.action(Action::PLAY, Some(0));
        assert!(result.is_none());
    }

    #[test]
    fn test_action_play_when_card_is_not_playable() {
        let mut game = helper_create_game(2);
        let result = game.action(Action::PLAY, Some(0));
        assert!(result.is_none());
    }

    #[test]
    fn test_action_play_when_card_is_played() {
        let mut game = helper_create_game(1);
        let result = game.action(Action::PLAY, Some(2));
        assert!(result.is_some());
        assert_eq!(game.pile.len(), 2);
        assert_eq!(*game.pile.last().unwrap(), 2);
        assert_eq!(game.players.first().unwrap().hand.len(), 0);
        assert_eq!(game.current_player_id, 1);
        assert!(game.players.first().unwrap().has_played);
    }

    #[test]
    fn test_action_draw() {
        let mut game = helper_create_game(1);
        let result = game.action(Action::DRAW, None);
        assert!(result.is_some());
        assert_eq!(game.pile.len(), 1);
        assert_eq!(game.players.first().unwrap().hand.len(), 2);
        assert_eq!(game.players.first().unwrap().has_drawn, true);
        assert_eq!(game.available_actions(), vec![Action::PLAY, Action::SKIP]);
    }

    #[test]
    fn test_action_skip_when_has_not_drawn() {
        let mut game = helper_create_game(1);
        let result = game.action(Action::SKIP, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_action_skip_when_has_drawn() {
        let mut game = helper_create_game(1);
        game.action(Action::DRAW, None);
        let result = game.action(Action::SKIP, None);
        assert!(result.is_some());
        assert_eq!(game.current_player_id, 1);
        assert!(game.players.first().unwrap().has_played);
    }

    #[test]
    fn test_action_next_when_player_has_not_finished_turn() {
        let mut game = helper_create_game(1);
        let result = game.next();
        assert!(!result);
    }

    #[test]
    fn test_action_next_when_player_won_the_game() {
        let mut game = helper_create_game(1);
        game.action(Action::PLAY, Some(2));
        let result = game.next();
        assert!(result);
        assert_eq!(game.state, GameState::FINISHED);
    }

    #[test]
    fn test_action_next_when_player_passes_the_turn() {
        let mut game = helper_create_game(1);
        game.action(Action::DRAW, None);
        game.action(Action::SKIP, None);
        let result = game.next();
        assert!(result);
        assert_eq!(game.state, GameState::STARTED);
    }

    fn helper_create_game<'c>(current_player_id: u32) -> Game<'c> {
        Game {
            state: GameState::STARTED,
            cards: vec![
                Card {
                    id: 0,
                    color: "blue",
                    value: "1",
                },
                Card {
                    id: 1,
                    color: "red",
                    value: "1",
                },
                Card {
                    id: 2,
                    color: "yellow",
                    value: "1",
                },
                Card {
                    id: 3,
                    color: "yellow",
                    value: "9",
                },
            ],
            pile: vec![0],
            deck: vec![1],
            current_player_id,
            players: vec![
                Player {
                    id: 1,
                    name: "Aaron".to_string(),
                    hand: vec![2],
                    has_drawn: false,
                    has_played: false,
                },
                Player {
                    id: 2,
                    name: "Bea".to_string(),
                    hand: vec![3],
                    has_drawn: false,
                    has_played: false,
                },
            ],
        }
    }
}
