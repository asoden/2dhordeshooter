use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::constants::*;
use crate::state::GameState;

pub struct ResourcesPlugin;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct GlobalAudioSource{
    pub weapon_effect: Option<Handle<AudioSource>>
}

#[derive(Resource, Debug)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(GlobalAudioSource::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                (update_cursor_position).run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlas>,
    mut audio_source: ResMut<GlobalAudioSource>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("loading assets");
    texture_atlas.image = Some(asset_server.load(SPRITE_SHEET_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH, TILE_HEIGHT),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        None,
        None,
    );
    texture_atlas.layout = Some(texture_atlas_layouts.add(layout));

    audio_source.weapon_effect = Some(asset_server.load("audio/effects/attack.wav"));

    next_state.set(GameState::MainMenu);
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
