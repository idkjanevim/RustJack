use bevy::prelude::*;
use crate::player_dealer::{Player, Dealer};
use crate::deck::Card;

#[derive(Component)]
pub struct CardTag(pub String);

pub fn card_spawner(card: Card, commands: &mut Commands, asset_server: &Res<AssetServer>)
{
    commands.spawn((SpriteBundle {
        texture: asset_server.load(card.card.to_owned()+".png"),
        transform: Transform {
            scale: (1.,1.,1.).into(),
            translation: card.position,
            ..Default::default()
        },
        ..Default::default()
    }, CardTag(card.card)));

}

pub fn move_cards(
    mut _commands: Commands, mut q: Query<(&mut Transform, &CardTag)>, player: Res<Player>, dealer: Res<Dealer>
)
{
    for (mut card, tag) in &mut q.iter_mut()
    {
        for p_card in &player.hand
        {
            if p_card.card == tag.0 
            {
                card.translation = p_card.position;
            }
        }
        for d_card in &dealer.hand
        {
            if d_card.card == tag.0
            {
                card.translation = d_card.position;
            }
        }
    }
}

pub fn clear(
    commands: &mut Commands, q_cards: &Query<Entity, With<CardTag>>
)
{
    for entity in q_cards.iter()
    {
        commands.entity(entity).despawn();
    }
}

