use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::enemy::Enemy;
use crate::state::GameState;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(OnEnter(GameState::GameInit), spawn_debug_text)
            .add_systems(
                Update,
                update_debug_text.run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_debug_text(mut commands: Commands) {
    commands.spawn(TextBundle::from_section(
        "hello\nbevy!",
        TextStyle {
            font_size: 20.0,
            color: Color::srgb(0.0, 0.0, 1.0),
            ..default()
        },
    ));
}

fn update_debug_text(
    mut query: Query<&mut Text, With<Text>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>
) {
    if query.is_empty() {
        return;
    }

    let num_enemies = enemy_query.iter().count();
    let mut text = query.single_mut();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("FPS:{value:.2}\nEnemies:{num_enemies}");
        }
    }
}
