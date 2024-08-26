use bevy::prelude::*;

use crate::state::GameState;
use crate::player::Player;
use crate::weapon::Weapon;
use crate::CursorPosition;

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animation_timer_tick, animate_player, flip_weapon_sprite_y).run_if(in_state(GameState::InGame)));
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut TextureAtlas, &mut Sprite, &Transform, &AnimationTimer), With<Player>>
) {
    if player_query.is_empty() {
        return;
    }

    let (mut texture, mut sprite, transform, timer) = player_query.single_mut();
    if timer.just_finished() {
        texture.index = (texture.index + 1) % 2;
    }

    // flip player when cursor to left of player sprite
    if let Some(cursor_position) = cursor_position.0 {
        sprite.flip_x = cursor_position.x < transform.translation.x;
    }
}

fn flip_weapon_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut weapon_query: Query<(&mut Sprite, &Transform), With<Weapon>>,
) {
    if weapon_query.is_empty() {
        return;
    }
    
    let (mut sprite, transform) = weapon_query.single_mut();
    
    // flip weapon when cursor to left of player sprite
    if let Some(cursor_position) = cursor_position.0 {
        sprite.flip_y = cursor_position.x < transform.translation.x;
    }
}