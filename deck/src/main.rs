#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // make it short for ease of typing
        let suits = ["Hearts", "Spades", "Clubs"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);

                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&self) {}
}

fn main() {
    let deck = Deck::new();

    deck.shuffle();

    println!("Here's your deck: {:#?}", deck);
}
