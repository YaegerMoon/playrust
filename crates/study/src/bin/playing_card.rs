/**
* Create an enum Card with the following variants:

King, Queen, Jack (unit variants for face cards).
Numbered(u8, String) representing a numbered card with its value and suit.
Write a function card_description that takes a Card and returns a description of the card:

For King, return "King".
For Queen, return "Queen".
For Jack, return "Jack".
For Numbered(value, suit), return "{value} of {suit}", e.g., "7 of Hearts".
Ignore error handling for this challenge.
*/

pub enum Card {
    King,
    Queen,
    Jack,
    Numbered(u32, String),
}

fn main() {
    assert_eq!(card_description(&Card::King), "King");
    assert_eq!(card_description(&Card::Queen), "Queen");
    assert_eq!(card_description(&Card::Jack), "Jack");

    let card = Card::Numbered(7, "Hearts".to_string());
    assert_eq!(card_description(&card), "7 of Hearts");

    let card = Card::Numbered(10, "Diamonds".to_string());
    assert_eq!(card_description(&card), "10 of Diamonds");

    let card = Card::Numbered(1, "Clubs".to_string());
    assert_eq!(card_description(&card), "1 of Clubs");

    let card = Card::Numbered(13, "Spades".to_string());
    assert_eq!(card_description(&card), "13 of Spades");
}

pub fn card_description(card: &Card) -> String {
    match card {
        Card::King => String::from("King"),
        Card::Queen => String::from("Queen"),
        Card::Jack => String::from("Jack"),
        Card::Numbered(value, suit) => format!("{} of {}", value, suit),
    }
}
