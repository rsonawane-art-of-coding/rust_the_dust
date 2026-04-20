/* Enums */
#![allow(unused)]

#[derive(Debug)]
enum CardsSuits {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
struct CardGame {
    no_of_cards:u32,
    card_combination:CardsSuits,
} 

/* Tuple like Struct*/
#[derive(Debug)]
struct SpecialCards (
    char,
    char,
    char,
    char,
);

fn main()
{
    let first_card = CardsSuits::Hearts;
    let second_card = CardsSuits::Diamonds;
    
    println!("{:?} {:?}", first_card, second_card);
    
    let mut swapped_card = second_card;

    println!("{:?}" , swapped_card);

    /* Structure Example with enum */
    let rummy = CardGame {
        no_of_cards:52,
        card_combination: CardsSuits::Hearts,
    }; 

    println!("Rummy {:?}", rummy);


    /* Tuple example using enums*/
    let my_favorite_cards: (CardsSuits, CardsSuits) = (
        CardsSuits::Diamonds,
        CardsSuits::Clubs,
    );
    
    println!("My Favorite Card Types {:?}", my_favorite_cards);


    let special_cards = SpecialCards ('A', 'K', 'Q', 'J');

    println!("Special Cards {:?}", special_cards);
}


