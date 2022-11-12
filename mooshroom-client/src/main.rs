mod minecraft;
mod player;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            position: WindowPosition::Centered(MonitorSelection::Current),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(minecraft::MinecraftPlugin)
        .add_startup_system(setup_system)
        .add_system(ui_example_system)
        .run();
}

fn ui_example_system(
    mut egui_context: ResMut<EguiContext>,
    mut player: Query<&mut player::EntityHealth, With<player::Player>>,
) {
    let mut player = player.single_mut();
    egui::TopBottomPanel::bottom("bottom_panel").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut player.health, 0.0..=100.0).text("health"));
    });
}
fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    //commands.insert_resource(AmbientLight {
    //    color: Color::WHITE,
    //    brightness: 1.0,
    //});

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

   
}
