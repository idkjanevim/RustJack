// Made by Ondřej Hrzán (idkjanevim)
// https://github.com/idkjanevim
// Sprites from https://opengameart.org/content/cards-set


#![allow(non_snake_case)]
mod constants;
mod player_dealer;
mod sprite_renderer;
mod deck;
mod text;
mod input;


use bevy::{prelude::*, window::PresentMode};
use constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
use deck::Deck;
use player_dealer::{Player, Dealer, dealer_turn, end};
use sprite_renderer::move_cards;
use text::{spawn_text, update_text};
use input::{input, BetChange};

#[derive(Resource)]
pub struct GameState(StateOfGame);

#[derive(Resource)]
pub struct Bet(i128);

#[derive(PartialEq)]
pub enum StateOfGame 
{
    Bet,
    Player,
    Dealer,
    End
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_text));
        app.add_systems(Update, (move_cards, update_text, input, dealer_turn, end));
    }
}

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("#35654d").unwrap()));
        app.insert_resource(Player
        {   hand: vec![],
            chips: 2000,
            points: 0
        });
        app.insert_resource(Dealer{hand: vec![], points: 0});
        app.insert_resource(Deck{cards: vec![]});
        app.insert_resource(GameState(StateOfGame::Bet));
        app.insert_resource(Bet(0));
    }
}

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BetChange>();
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Jack".into(),
                resolution: (WINDOW_WIDTH,WINDOW_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
    }), SystemsPlugin, ResourcesPlugin, EventPlugin))
    .run();
}

fn setup(
    mut commands: Commands, mut deck: ResMut<Deck>
) {
    commands.spawn(Camera2dBundle::default());
    deck.create_deck();
    deck.shuffle();
}
