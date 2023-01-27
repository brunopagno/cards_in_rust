use rand::prelude::*;

/*
 * let game = Game::new("Aaron", "Bea", "Camille", "David");
 * => checks for minimum amount of players
 * => selects starting player
 *
 * game.setup();
 * => shuffles deck
 * => deals cards
 * => flips the top card
 * => start player turn
 *
 * let players = game.players();
 * let current_player = game.current_player();
 * let current_player_cards = current_player.cards();
 * let card_id = current_player_cards[0].id;
 *
 * let actions = game.available_actions();
 * => { Play, Draw, Skip };
 *
 * game.act(player, Actions::Play, Some(card_id));
 * => check if player is the current player
 * => check if card can be played
 * => move the card from hand to the pile
 * => mark player turn as complete
 *
 * game.act(player, Actions::Draw, None);
 * => check if player has drawn this turn
 * => draw card
 *
 * game.act(player, Actions::Skip, None);
 * => check if player has drawn this turn
 * => mark player turn as complete
 *
 * game.next();
 * => check if player turn is complete
 * => check for winner
 * => select next player
 * => loop next turn
 *
 */

#[derive(PartialEq, Debug)]
enum GameState {
    CREATED,
    STARTED,
    PLAYING,
    FINISHED,
}

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub value: String,
    pub color: String,
}

struct Player {
    id: u32,
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    //    fn draw(&self, game: &Game) {
    //    game.deck.drain maybe?
    //        game.deck
    //    }
}

pub struct Game {
    state: GameState,
    pub deck: Vec<Card>,
    pub pile: Vec<Card>,
    players: Vec<Player>,
    current_player_id: u32,
}

impl Game {
    pub fn new(player_names: Vec<&str>) -> Option<Game> {
        let players: Vec<Player> = player_names
            .iter()
            .enumerate()
            .map(|(i, name)| Player {
                id: (i + 1) as u32,
                name: name.to_string(),
                hand: vec![],
            })
            .collect();
        Some(Game {
            state: GameState::CREATED,
            deck: load_deck(),
            pile: vec![],
            current_player_id: players.first()?.id,
            players,
        })
    }

    fn current_player(&self) -> &Player {
        &self
            .players
            .iter()
            .find(|player| player.id == self.current_player_id)
            .unwrap()
    }

    //    fn top_card(&self) -> Card {
    //        let result = self.deck.first();
    //        match result {
    //            Some(card) -> dbg!(card),
    //            None -> {
    //                reset_pile();
    //                self.top_card(),
    //            }
    //        }
    //    }

    fn reset_pile(&mut self) {
        self.deck.append(&mut self.pile);
        self.deck.shuffle(&mut thread_rng());
    }

    pub fn setup(&mut self) {
        if self.state != GameState::CREATED {
            println!("Incorrect state");
            return;
        }

        self.deck.shuffle(&mut thread_rng());
        // shuffles deck
        // deals cards
        // flips the top card
        // start player turn
    }
}

fn load_deck() -> Vec<Card> {
    vec![
        Card {
            id: 1,
            value: String::from("A"),
            color: String::from("red"),
        },
        Card {
            id: 2,
            value: String::from("2"),
            color: String::from("red"),
        },
        Card {
            id: 3,
            value: String::from("3"),
            color: String::from("red"),
        },
        Card {
            id: 4,
            value: String::from("4"),
            color: String::from("red"),
        },
        Card {
            id: 5,
            value: String::from("5"),
            color: String::from("red"),
        },
        Card {
            id: 6,
            value: String::from("6"),
            color: String::from("red"),
        },
        Card {
            id: 7,
            value: String::from("7"),
            color: String::from("red"),
        },
        Card {
            id: 8,
            value: String::from("8"),
            color: String::from("red"),
        },
        Card {
            id: 9,
            value: String::from("9"),
            color: String::from("red"),
        },
        Card {
            id: 10,
            value: String::from("10"),
            color: String::from("red"),
        },
        Card {
            id: 11,
            value: String::from("A"),
            color: String::from("blue"),
        },
        Card {
            id: 12,
            value: String::from("2"),
            color: String::from("blue"),
        },
        Card {
            id: 13,
            value: String::from("3"),
            color: String::from("blue"),
        },
        Card {
            id: 14,
            value: String::from("4"),
            color: String::from("blue"),
        },
        Card {
            id: 15,
            value: String::from("5"),
            color: String::from("blue"),
        },
        Card {
            id: 16,
            value: String::from("6"),
            color: String::from("blue"),
        },
        Card {
            id: 17,
            value: String::from("7"),
            color: String::from("blue"),
        },
        Card {
            id: 18,
            value: String::from("8"),
            color: String::from("blue"),
        },
        Card {
            id: 19,
            value: String::from("9"),
            color: String::from("blue"),
        },
        Card {
            id: 20,
            value: String::from("10"),
            color: String::from("blue"),
        },
        Card {
            id: 21,
            value: String::from("A"),
            color: String::from("yellow"),
        },
        Card {
            id: 22,
            value: String::from("2"),
            color: String::from("yellow"),
        },
        Card {
            id: 23,
            value: String::from("3"),
            color: String::from("yellow"),
        },
        Card {
            id: 24,
            value: String::from("4"),
            color: String::from("yellow"),
        },
        Card {
            id: 25,
            value: String::from("5"),
            color: String::from("yellow"),
        },
        Card {
            id: 26,
            value: String::from("6"),
            color: String::from("yellow"),
        },
        Card {
            id: 27,
            value: String::from("7"),
            color: String::from("yellow"),
        },
        Card {
            id: 28,
            value: String::from("8"),
            color: String::from("yellow"),
        },
        Card {
            id: 29,
            value: String::from("9"),
            color: String::from("yellow"),
        },
        Card {
            id: 30,
            value: String::from("10"),
            color: String::from("yellow"),
        },
        Card {
            id: 31,
            value: String::from("A"),
            color: String::from("green"),
        },
        Card {
            id: 32,
            value: String::from("2"),
            color: String::from("green"),
        },
        Card {
            id: 33,
            value: String::from("3"),
            color: String::from("green"),
        },
        Card {
            id: 34,
            value: String::from("4"),
            color: String::from("green"),
        },
        Card {
            id: 35,
            value: String::from("5"),
            color: String::from("green"),
        },
        Card {
            id: 36,
            value: String::from("6"),
            color: String::from("green"),
        },
        Card {
            id: 37,
            value: String::from("7"),
            color: String::from("green"),
        },
        Card {
            id: 38,
            value: String::from("8"),
            color: String::from("green"),
        },
        Card {
            id: 39,
            value: String::from("9"),
            color: String::from("green"),
        },
        Card {
            id: 40,
            value: String::from("10"),
            color: String::from("green"),
        },
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_new_game() {
        let game = Game::new(vec!["Aaron", "Bea"]).unwrap();
        assert_eq!(game.state, GameState::CREATED);
        assert_eq!(game.deck.len(), 40);
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
    fn test_reset_pile() {
        let mut game = Game {
            deck: vec![],
            pile: vec![
                Card {
                    id: 0,
                    color: String::from("red"),
                    value: String::from("3"),
                },
                Card {
                    id: 1,
                    color: String::from("red"),
                    value: String::from("4"),
                },
            ],
            state: GameState::PLAYING,
            players: vec![],
            current_player_id: 0,
        };

        game.reset_pile();

        assert_eq!(game.deck.len(), 2);
        assert_eq!(game.pile.len(), 0);
    }

    #[test]
    fn test_setup() {
        let mut game = Game::new(vec!["Aaron", "Bea"]).unwrap();
        let before = game.deck.iter().map(|c| c.id).collect::<Vec<u32>>();
        game.setup();
        let after = game.deck.iter().map(|c| c.id).collect::<Vec<u32>>();
        assert_ne!(before, after);
        assert_eq!(game.deck.len(), 40);
    }
}
