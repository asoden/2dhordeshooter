use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use kd_tree::{KdPoint, KdTree};

use crate::enemy::Enemy;
use crate::state::GameState;
use crate::weapon::Bullet;
use crate::{BULLET_DAMAGE, KD_TREE_REFRESH_RATE};

pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
}

#[derive(Resource)]
struct EnemyKdTree(KdTree<Collidable>);

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyKdTree::default()).add_systems(
            Update,
            (
                handle_enemy_bullet_collision,
                update_enemy_dk_tree
                    .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_enemy_dk_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut enemies = Vec::new();
    for (transform, entity) in enemy_query.iter() {
        enemies.push(Collidable {
            entity,
            pos: transform.translation.truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(enemies);
}

fn handle_enemy_bullet_collision(
    bullet_query: Query<&Transform, With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for bullet_transform in bullet_query.iter() {
        let pos = bullet_transform.translation;
        let enemies = tree.0.within_radius(&[pos.x, pos.y], 50.0);

        for e in enemies {
            if let Ok(mut enemy) = enemy_query.get_mut(e.entity) {
                enemy.health -= BULLET_DAMAGE;
            }
        }
    }
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = typenum::U2;

    fn at(&self, i: usize) -> f32 {
        if i == 0 {
            return self.pos.x;
        }

        self.pos.y
    }
}

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}
