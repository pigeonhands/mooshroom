mod camera;
mod minecraft;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode, PrimaryWindow, WindowTheme},
};
use bevy_text_mesh::prelude::*;
use camera::FlyCameraPlugin;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    #[default]
    Paused,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    resolution: (1000., 500.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin,
        ))
        .add_state::<AppState>()
        .add_plugins((FlyCameraPlugin, TextMeshPlugin, minecraft::MinecraftPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, cursor_grab_system)
        .run();
}


fn cursor_grab_system(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut app_state: ResMut<NextState<AppState>>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = q_windows.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;

        window.cursor.visible = false;
        app_state.set(AppState::InGame);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;

        window.cursor.visible = true;
        app_state.set(AppState::Paused);
    }
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    //commands.insert_resource(AmbientLight {
    //    color: Color::WHITE,
    //    brightness: 1.0,
    //});
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.02,
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
