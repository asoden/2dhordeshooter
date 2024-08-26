use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::state::GameState;
use crate::weapon::Bullet;
use crate::BULLET_DAMAGE;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_enemy_bullet_collision.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_enemy_bullet_collision(
    bullet_query: Query<&Transform, With<Bullet>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for bullet_transform in bullet_query.iter() {
        for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if bullet_transform
                .translation
                .distance_squared(enemy_transform.translation)
                <= 1000.0
            {
                enemy.health -= BULLET_DAMAGE;
            }
        }
    }
}
