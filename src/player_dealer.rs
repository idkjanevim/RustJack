use bevy::prelude::*;
use crate::{Bet, deck::{Card, Deck}, constants::{SPRITE_WIDTH, SPRITE_Y_DOWN, SPRITE_Y_UP}, sprite_renderer::{card_spawner, CardTag, clear}, StateOfGame, GameState, input::BetChange};

#[derive(Resource)]
pub struct Player
{
    pub hand: Vec<Card>,
    pub chips: i128,
    pub points: i32
}

impl Player
{
    pub fn take_card(&mut self, deck: &mut ResMut<Deck>, commands: &mut Commands, asset_server: &Res<AssetServer>)
    {
        for card in &mut self.hand
        {
            card.position.x = card.position.x - (SPRITE_WIDTH/4.);
        }
        let mut card = deck.get_card();
        card.position.y = SPRITE_Y_DOWN;
        card.position.x = (SPRITE_WIDTH/4.) * self.hand.len() as f32;
        card.position.z = self.hand.len() as f32 + 1.;

        self.hand.append(&mut vec![card.clone()]);

        self.points = calculate_points(&self.hand);

        card_spawner(card, commands, asset_server);
    }
}

#[derive(Resource)]
pub struct Dealer
{
    pub hand: Vec<Card>,
    pub points: i32
}

impl Dealer
{
    pub fn take_card(&mut self, deck: &mut ResMut<Deck>, commands: &mut Commands, asset_server: &Res<AssetServer>)
    {
        for card in &mut self.hand
        {
            card.position.x = card.position.x - (SPRITE_WIDTH/4.);
        }
        let mut card = deck.get_card();
        card.position.y = SPRITE_Y_UP;
        card.position.x = (SPRITE_WIDTH/4.) * self.hand.len() as f32;
        card.position.z = self.hand.len() as f32 + 1.;

        self.hand.append(&mut vec![card.clone()]);

        self.points = calculate_points(&self.hand);
        
        card_spawner(card, commands, asset_server);
    }
    pub fn take_face_down_card(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>)
    {
        self.hand.insert(0,Card{card: "Back".to_owned(), position: (0.,SPRITE_Y_UP, 0.).into(), value: (0,0)});
        card_spawner(self.hand[0].clone(), commands, asset_server);
    }
    pub fn remove_face_down_card(&mut self)
    {
        self.hand.remove(0);
        for card in &mut self.hand
        {
            card.position.x = 0.;
            card.position.z = 0.;
        }
    }
}


fn calculate_points(cards: &Vec<Card>) -> i32
{
    let mut ace: i32 = 0;
    let mut count: i32 = 0;
    for card in cards
    {
        if card.value.0 != card.value.1
        {
            ace += 1;
        }
        count += card.value.1;
    }

    while ace > 0
    {
        if count > 21  
        {
            count -= 10;
        }
        ace -= 1;
    }

    count
}

pub fn dealer_turn(
    mut commands: Commands, mut dealer: ResMut<Dealer>, mut game_state: ResMut<GameState>, asset_server: Res<AssetServer>, mut deck: ResMut<Deck>,
    q_cards: Query<(Entity, &CardTag), With<CardTag>>
)
{
    if game_state.0 != StateOfGame::Dealer
    {
        return;
    }
    if dealer.hand[0].card == "Back".to_owned()
    {
        dealer.remove_face_down_card();
        for (entity, tag) in q_cards.iter()
        {
            if tag.0  == "Back".to_owned()
            {
                commands.entity(entity).despawn();
            }
        }
    }
    if dealer.points < 17
    {
        dealer.take_card(&mut deck, &mut commands, &asset_server);
    }
    else
    {
        *game_state = GameState(StateOfGame::End);
    }
}


pub fn end(
    mut commands: Commands, mut game_state: ResMut<GameState>, mut player: ResMut<Player>, mut dealer: ResMut<Dealer>, bet: Res<Bet>, input: Res<Input<KeyCode>>,
    mut ev_bet_change: EventWriter<BetChange>, q_cards: Query<Entity, With<CardTag>>, mut deck: ResMut<Deck>
)
{
    if game_state.0 != StateOfGame::End { return; }

    if input.just_pressed(KeyCode::Return)
    {
        if (player.points == 21 && player.hand.len() == 2) && (dealer.points != 21 || (dealer.points == 21 && dealer.hand.len() != 2))
        {
            player.chips += (bet.0 * 2) + (bet.0 / 2);
        }
        else if player.points <= 21 && (player.points > dealer.points || dealer.points > 21)
        {
            player.chips += bet.0 * 2;
        }
        else if player.points == dealer.points
        {
            player.chips += bet.0;
        }
        ev_bet_change.send(BetChange("0".to_owned()));

        player.hand = vec![];
        dealer.hand = vec![];
        player.points = 0;
        dealer.points = 0;
        deck.create_deck();
        deck.shuffle();
        clear(&mut commands, &q_cards);
        *game_state = GameState(StateOfGame::Bet);
    }
}       
