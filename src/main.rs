use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck{
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Spades"];
        let values = ["Ace", "2", "3"];
    
        let mut cards = vec![];
    
        for suit in suits {
            for value in values {
                let card = format!{"{} of {}", value, suit};
                cards.push(card);
            }
        }
    
        Deck { cards }
    }

    fn shuffle (&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    } 

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        if self.cards.len() < num_cards{
            return self.cards.split_off(0);
        }
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    //deck.shuffle();

    let cards = deck.deal(3);

    println!("Here is your hand: {:#?}", cards);
}
