use crate::card::Card;

pub struct Player<'c> {
    pub id: u32,
    pub name: String,
    pub hand: Vec<Card<'c>>,
    pub(crate) has_drawn: bool,
    pub(crate) has_played: bool,
}

impl<'c> Player<'c> {
    pub(crate) fn new(id: u32, name: String) -> Player<'c> {
        Player {
            id,
            name,
            hand: vec![],
            has_drawn: false,
            has_played: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_player() {
        let player = Player::new(99, String::from("Aaron"));
        assert_eq!(player.id, 99);
        assert_eq!(player.name, String::from("Aaron"));
        assert_eq!(player.hand.len(), 0);
    }
}
