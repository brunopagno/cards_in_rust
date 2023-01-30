#[derive(Debug)]
pub struct Card<'c> {
    pub id: u32,
    pub value: &'c str,
    pub color: &'c str,
}

impl<'c> Card<'c> {
    pub(crate) fn load_deck() -> Vec<Card<'c>> {
        vec![
            Card {
                id: 1,
                value: "A",
                color: "red",
            },
            Card {
                id: 2,
                value: "2",
                color: "red",
            },
            Card {
                id: 3,
                value: "3",
                color: "red",
            },
            Card {
                id: 4,
                value: "4",
                color: "red",
            },
            Card {
                id: 5,
                value: "5",
                color: "red",
            },
            Card {
                id: 6,
                value: "6",
                color: "red",
            },
            Card {
                id: 7,
                value: "7",
                color: "red",
            },
            Card {
                id: 8,
                value: "8",
                color: "red",
            },
            Card {
                id: 9,
                value: "9",
                color: "red",
            },
            Card {
                id: 10,
                value: "10",
                color: "red",
            },
            Card {
                id: 11,
                value: "A",
                color: "blue",
            },
            Card {
                id: 12,
                value: "2",
                color: "blue",
            },
            Card {
                id: 13,
                value: "3",
                color: "blue",
            },
            Card {
                id: 14,
                value: "4",
                color: "blue",
            },
            Card {
                id: 15,
                value: "5",
                color: "blue",
            },
            Card {
                id: 16,
                value: "6",
                color: "blue",
            },
            Card {
                id: 17,
                value: "7",
                color: "blue",
            },
            Card {
                id: 18,
                value: "8",
                color: "blue",
            },
            Card {
                id: 19,
                value: "9",
                color: "blue",
            },
            Card {
                id: 20,
                value: "10",
                color: "blue",
            },
            Card {
                id: 21,
                value: "A",
                color: "yellow",
            },
            Card {
                id: 22,
                value: "2",
                color: "yellow",
            },
            Card {
                id: 23,
                value: "3",
                color: "yellow",
            },
            Card {
                id: 24,
                value: "4",
                color: "yellow",
            },
            Card {
                id: 25,
                value: "5",
                color: "yellow",
            },
            Card {
                id: 26,
                value: "6",
                color: "yellow",
            },
            Card {
                id: 27,
                value: "7",
                color: "yellow",
            },
            Card {
                id: 28,
                value: "8",
                color: "yellow",
            },
            Card {
                id: 29,
                value: "9",
                color: "yellow",
            },
            Card {
                id: 30,
                value: "10",
                color: "yellow",
            },
            Card {
                id: 31,
                value: "A",
                color: "green",
            },
            Card {
                id: 32,
                value: "2",
                color: "green",
            },
            Card {
                id: 33,
                value: "3",
                color: "green",
            },
            Card {
                id: 34,
                value: "4",
                color: "green",
            },
            Card {
                id: 35,
                value: "5",
                color: "green",
            },
            Card {
                id: 36,
                value: "6",
                color: "green",
            },
            Card {
                id: 37,
                value: "7",
                color: "green",
            },
            Card {
                id: 38,
                value: "8",
                color: "green",
            },
            Card {
                id: 39,
                value: "9",
                color: "green",
            },
            Card {
                id: 40,
                value: "10",
                color: "green",
            },
        ]
    }
}
