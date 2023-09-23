use bevy::prelude::*;

use crate::{player_dealer::{Player, Dealer}, deck::Deck, GameState, Bet, StateOfGame};

#[derive(Event)]
pub struct BetChange(pub String);


pub fn input(
    mut commands: Commands, kbd: Res<Input<KeyCode>>, mut player: ResMut<Player>, asset_server: Res<AssetServer>, mut deck: ResMut<Deck>,
        mut game_state: ResMut<GameState>, mut bet: ResMut<Bet> , mut evr_char: EventReader<ReceivedCharacter>, mut input_bet: Local<String>,
        mut ev_bet_change: EventWriter<BetChange>, mut dealer: ResMut<Dealer>
)
{
    // Player input
    if game_state.0 == StateOfGame::Player
    {
        if deck.cards.len() == 52
        {
            dealer.take_face_down_card(&mut commands, &asset_server);
            dealer.take_card(&mut deck, &mut commands, &asset_server);
            player.take_card(&mut deck, &mut commands, &asset_server);
            player.take_card(&mut deck, &mut commands, &asset_server);
        }

        if kbd.just_pressed(KeyCode::Space)
        {
            player.take_card(&mut deck, &mut commands, &asset_server);
            if player.points > 21
            {
                *game_state = GameState(StateOfGame::End);
            }
        }
        if kbd.just_pressed(KeyCode::Return)
        {
            *game_state = GameState(StateOfGame::Dealer);
        }
    }
    
    // Bet input
    if game_state.0 == StateOfGame::Bet
    {
        if kbd.just_pressed(KeyCode::Return) && !input_bet.is_empty()
        {
            println!("Text input: {}", &*input_bet);
            let i = input_bet.to_string().parse::<i128>().unwrap();
            *bet = Bet(i);
            player.chips -= i;
            input_bet.clear();
            ev_bet_change.send(BetChange(input_bet.to_string()));
            *game_state = GameState(StateOfGame::Player);
        }
        if kbd.just_pressed(KeyCode::Back)
        {
            input_bet.pop();
            ev_bet_change.send(BetChange(input_bet.to_string()))
        }
        for ev in evr_char.iter()
        {
            if ev.char.is_numeric()
            {
                let mut s: String = input_bet.to_string();
                s.push(ev.char);
                let i = s.parse::<i128>().unwrap(); 
                s = i.to_string();
                if i < 99999999999 && i <= player.chips
                {
                    input_bet.clear();
                    input_bet.push_str(&s);
                    ev_bet_change.send(BetChange(input_bet.to_string()))
                }
            }
        }
    }


}



