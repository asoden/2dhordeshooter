use std::f32::consts::PI;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::PrimaryWindow;

// Window
const WW: f32 = 1200.0;
const WH: f32 = 900.0;

// Sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const SPRITE_SHEET_WIDTH: u32 = 4;
const SPRITE_SHEET_HEIGHT: u32 = 4;

// Player
const PLAYER_SPEED: f32 = 2.0;

// Colors
const BG_COLOR: (f32, f32, f32) = (0.773, 0.8, 0.723);

// Wepaon
const BULLET_SPAWN_INTERVAL: f32 = 0.1;

// Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Resource, Debug)]
struct CursorPosition(Option<Vec2>);

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Weapon;

#[derive(Component)]
struct WeaponTimer(Stopwatch);
#[derive(Component)]
struct Bullet;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2)))
        .insert_resource(Msaa::Off)
        // Custom Resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPosition(None))
        //Systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(
            Update,
            (
                handle_player_input,
                update_weapon_transform,
                update_cursor_position,
                handle_weapon_input,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, close_on_esc)
        .run();
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("loading assets");
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH, TILE_HEIGHT),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        None,
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::GameInit);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_handle.0.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 0,
        },
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle.0.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 2,
        },
        Weapon,
        WeaponTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::InGame);
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;

    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }

    delta = delta.normalize_or_zero();

    transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
}

fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    mut weapon_query: Query<(&Transform, &mut WeaponTimer), With<Weapon>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
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

    if weapon_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        weapon_timer.0.reset();

        commands.spawn((
            SpriteBundle {
                texture: image_handle.0.clone().unwrap(),
                transform: Transform::from_translation(vec3(weapon_pos.x, weapon_pos.y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 3,
            },
            Bullet,
        ));
    }
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
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
