use animation::AnimationTimer;
use bevy::{math::vec3, prelude::*, time::Stopwatch};
use rand::Rng;
use weapon::{Weapon, WeaponTimer};

use crate::*;
use player::{Health, Player, PlayerState};
use state::GameState;

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        Player,
        Health(PLAYER_HEALTH),
        PlayerState::default(),
        GameEntity,
    ));

    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 14,
        },
        Weapon,
        WeaponTimer(Stopwatch::new()),
        GameEntity,
    ));

    next_state.set(GameState::InGame);
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.gen_range(-WORLD_HEIGHT..WORLD_HEIGHT);
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(12..=13),
            },
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    all_entities: Query<Entity, With<GameEntity>>,
    mut commands: Commands,
) {
    if all_entities.is_empty() {
        return;
    }

    for entity in all_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
