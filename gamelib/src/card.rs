#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub value: String,
    pub color: String,
}

pub(crate) fn load_deck() -> Vec<Card> {
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
