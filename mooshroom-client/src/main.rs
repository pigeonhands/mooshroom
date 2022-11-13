mod minecraft;
mod camera;
use bevy::{prelude::*, window::CursorGrabMode};
use camera::FlyCameraPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor {
            width: 500.0,
            height: 500.0,
            position: WindowPosition::Centered,
            cursor_grab_mode: CursorGrabMode::Locked,
            ..Default::default()
        },
        ..Default::default()
        }))
        .add_state(AppState::Paused)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(minecraft::MinecraftPlugin)
        .add_startup_system(setup_system)
        .add_system(cursor_grab_system)
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(lock_cursor_position)
        )
        .run();
}

fn lock_cursor_position(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if window.cursor_grab_mode() == CursorGrabMode::Locked{
            window.set_cursor_position(Vec2::new(window.width() / 2., window.height() / 2.));
        }
    }
}

fn cursor_grab_system(
    mut window_res: ResMut<Windows>,
    mut app_state: ResMut<State<AppState>>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = window_res.primary_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        app_state.set(AppState::InGame).unwrap();
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_grab_mode(CursorGrabMode::None);
        app_state.set(AppState::Paused).unwrap();
    }
}


fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    //commands.insert_resource(AmbientLight {
    //    color: Color::WHITE,
    //    brightness: 1.0,
    //});

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
