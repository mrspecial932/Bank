use rand::{thread_rng, seq::SliceRandom};
#[derive(Debug)]
struct Deck {
    cards :Vec<String>,
}
impl Deck {
    fn new()->Self{
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];


    let mut cards = vec![];


    for suit in suits{
        for value in values{
            let card = format!("{}  of {} ", value, suit);
            cards.push(card)
        }
    }
     Deck {cards}
    
    }
    fn shuffle(&mut self){ 
       let mut rng = thread_rng();
       self.cards.shuffle(&mut rng); 
    }
    fn deal(&mut self, num_Cards: usize)->Vec<String>{
        return  self.cards.split_off(self.cards.len()- num_Cards)
    }
}
fn main(){
    let mut deck = Deck::new();
     
    deck.shuffle()

    let cards = deck.deal(3);
    println!("Here is your deck : {:#?}", deck);
    println!("Here is your deck : {:#?}", cards);
 
}