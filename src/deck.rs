use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct Card
{
    pub card: String,
    pub position: Vec3,
    pub value: (i32,i32),
    
}

impl Card
{
}

#[derive(Resource)]
pub struct Deck 
{
    pub cards: Vec<Card>
}

impl Deck
{
    pub fn shuffle(&mut self)
    {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn create_deck(&mut self)
    {
        let mut cards: Vec<Card> = vec![];
        for i in 1..14 
        {
            for j in 1..5
            {
                cards.append(&mut vec![Card{card: if j==1 {"Clubs ".to_owned()} 
                                                    else if j==2 {"Diamond ".to_owned()} 
                                                    else if j==3 {"Hearts ".to_owned()} 
                                                    else {"Spades ".to_owned()}+&i.to_string(), position: (0.,0.,0.).into(),
                                            value: match i {
                                                1 => (1,11),
                                                2 => (2,2),
                                                3 => (3,3),
                                                4 => (4,4),
                                                5 => (5,5),
                                                6 => (6,6),
                                                7 => (7,7),
                                                8 => (8,8),
                                                9 => (9,9),
                                                10 | 11 | 12 | 13 => (10,10),
                                                _ => (0,0)
                                            }}]);
            }
        }
        self.cards = cards;
    }

    pub fn get_card(&mut self) -> Card
    {
        let card = self.cards[0].clone();
        self.cards.remove(0);
        return card
    }

}

