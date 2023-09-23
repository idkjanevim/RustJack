use bevy::prelude::*;

use crate::{constants::{WINDOW_HEIGHT, SPRITE_HEIGHT, WINDOW_WIDTH}, player_dealer::{Player, Dealer}, input::BetChange, GameState, StateOfGame, sprite_renderer::CardTag};

#[derive(Component)]
pub struct PlayerPoints;
#[derive(Component)]
pub struct DealerPoints;
#[derive(Component)]
pub struct Balance;
#[derive(Component)]
pub struct BetInput;


pub fn spawn_text( mut commands: Commands, asset_server: Res<AssetServer> )
{
    let font: Handle<Font> = asset_server.load("fonts/RobotoCondensed-Light.ttf");
    let mut text_style = TextStyle {
        font: font.clone(),
        font_size: 50.,
        color: Color::WHITE
    };

    commands.spawn((Text2dBundle {
        text: Text::from_section("0", text_style.clone())
            .with_alignment(TextAlignment::Center),
        transform: Transform{
            translation: (0.,((WINDOW_HEIGHT/-2.)+SPRITE_HEIGHT+40.) as f32, 0.).into(),
            ..Default::default()
        },
            ..Default::default()   
    }, PlayerPoints));


    commands.spawn((Text2dBundle {
        text: Text::from_section("0", text_style.clone())
            .with_alignment(TextAlignment::Center),
        transform: Transform{
            translation: (0.,((WINDOW_HEIGHT/2.)-SPRITE_HEIGHT-40.) as f32, 0.).into(),
            ..Default::default()
        },
            ..Default::default()   
    }, DealerPoints));

    commands.spawn((Text2dBundle {
        text: Text::from_section("0", text_style.clone())
            .with_alignment(TextAlignment::Center),
        transform: Transform{
            translation: ((WINDOW_WIDTH/-2.)+50.,-50., 0.).into(),
            ..Default::default()
        },
            ..Default::default()   
    }, Balance));


    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(Vec2::new(250.,60.)),
            ..Default::default()
        },
        transform: Transform { translation: (0.,0.,10.).into(), ..Default::default() },
        ..Default::default()
    })
    .with_children(|builder|{
        builder.spawn(
            (Text2dBundle {
                text: Text::from_section("0", text_style.clone())
                    .with_alignment(TextAlignment::Center),
                transform: Transform { translation: (0.,0.,11.).into(), ..Default::default() },
                ..Default::default()
            }, BetInput)
        );
    });




    text_style = TextStyle {
        font: font.clone(),
        font_size: 30.,
        color: Color::WHITE
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section("Balance", text_style.clone())
            .with_alignment(TextAlignment::Center),
        transform: Transform{
            translation: ((WINDOW_WIDTH/-2.)+50.,-25., 0.).into(),
            ..Default::default()
        },
            ..Default::default()   
    });
}

pub fn update_text(
    _commands: Commands, player: Res<Player>, dealer: Res<Dealer>,
        mut q_playerpoints: Query<&mut Text, (With<PlayerPoints>, Without<DealerPoints>)>,
        mut q_dealerpoints: Query<&mut Text, (With<DealerPoints>, Without<PlayerPoints>)>,
        mut q_balance: Query<&mut Text, (With<Balance>, Without<DealerPoints>, Without<PlayerPoints>)>,
        mut q_input_fied: Query<&mut Text, (With<BetInput>, Without<Balance>,Without<DealerPoints>, Without<PlayerPoints>)>,
        mut q_input_field_bg: Query<&mut Sprite, (Without<CardTag>,Without<BetInput>, Without<Balance>,Without<DealerPoints>, Without<PlayerPoints>)>,
        asset_server: Res<AssetServer>, game_state: Res<GameState>,
        mut bet_input: EventReader<BetChange>
)
{
    let font: Handle<Font> = asset_server.load("fonts/RobotoCondensed-Light.ttf");
    let mut text_style = TextStyle {
        font: font.clone(),
        font_size: 50.,
        color: Color::WHITE
    };
    for mut txt in &mut q_playerpoints
    {
        txt.sections = vec![TextSection{value: player.points.to_string().to_owned(), style: text_style.clone()}];
    }
    for mut txt in &mut q_dealerpoints
    {
        txt.sections = vec![TextSection{value: dealer.points.to_string().to_owned(), style: text_style.clone()}];
    }



    for mut txt in &mut q_input_fied
    {
        for ev in bet_input.iter()
        {
            if game_state.0 == StateOfGame::Bet
            {
                txt.sections = vec![TextSection{value: ev.0.to_owned(), style: text_style.clone()}];
            }
        }
        for mut sprt in &mut q_input_field_bg
        {
            if game_state.0 == StateOfGame::Bet
            {
                sprt.color = Color::GRAY;
            }
            else
            {
                sprt.color = Color::rgba(0., 0., 0., 0.);
            }
        }
    }
    


    text_style = TextStyle {
        font: font.clone(),
        font_size: 25.,
        color: Color::WHITE
    };
    for mut txt in &mut q_balance
    {
        txt.sections = vec![TextSection{value: player.chips.to_string().to_owned(), style: text_style.clone()}];
    }
}
