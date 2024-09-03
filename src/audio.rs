use bevy::prelude::*;

use crate::state::GameState;

pub struct GameAudioPlugin;

#[derive(Component)]
pub struct Music;

#[derive(Component)]
pub struct SoundEffect;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct GameplayMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameplayMusic>()
            .add_systems(OnEnter(GameState::InGame), play_gameplay_music)
            .add_systems(OnExit(GameState::InGame), stop_music);
    }
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load("audio/music/davidkbd-temple_of_madness.ogg"),
            entity: None,
        }
    }
}

fn play_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}
