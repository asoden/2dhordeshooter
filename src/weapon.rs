use std::f32::consts::PI;
use std::time::Instant;

use audio::SoundEffect;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::player::Player;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::*;

pub struct WeaponPlugin;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct WeaponTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct SpawnInstant(pub Instant);
#[derive(Component)]
struct BulletDirection(Vec3);

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_weapon_transform,
                update_bullets,
                handle_weapon_input,
                despawn_old_bullets,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    for (instant, entity) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            commands.entity(entity).despawn();
        }
    }
}

fn update_weapon_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut weapon_query: Query<&mut Transform, (With<Weapon>, Without<Player>)>,
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };

    let mut weapon_transform = weapon_query.single_mut();

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    weapon_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_weapon_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );

    weapon_transform.translation = vec3(
        new_weapon_pos.x,
        new_weapon_pos.y,
        weapon_transform.translation.z,
    );
    weapon_transform.translation.z = 15.0;
}

fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    mut weapon_query: Query<(&Transform, &mut WeaponTimer), With<Weapon>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
    audio: Res<GlobalAudioSource>,
) {
    if weapon_query.is_empty() {
        return;
    }

    let (weapon_transform, mut weapon_timer) = weapon_query.single_mut();
    let weapon_pos = weapon_transform.translation.truncate();
    weapon_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let mut rng = rand::thread_rng();
    let bullet_direction = weapon_transform.local_x();
    if weapon_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        weapon_timer.0.reset();

        for _ in 0..NUM_BULLETS_PER_SHOT {
            let dir = vec3(
                bullet_direction.x + rng.gen_range(-0.5..0.5),
                bullet_direction.y + rng.gen_range(-0.5..0.5),
                bullet_direction.z,
            );
            commands.spawn((
                SpriteBundle {
                    texture: handle.image.clone().unwrap(),
                    transform: Transform::from_translation(vec3(weapon_pos.x, weapon_pos.y, 1.0))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 15,
                },
                Bullet,
                BulletDirection(dir),
                SpawnInstant(Instant::now()),
                GameEntity,
            ));
        }
        commands.spawn((
            AudioBundle {
                source: audio.weapon_effect.clone().unwrap(),
                settings: PlaybackSettings::DESPAWN,
            },
            SoundEffect,
        ));
    }
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut transform, direction) in bullet_query.iter_mut() {
        transform.translation += direction.0.normalize() * Vec3::splat(BULLET_SPEED);
        transform.translation.z = 10.0;
    }
}
