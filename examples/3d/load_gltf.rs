//! Loads and renders a glTF file as a scene.

use bevy::{
    pbr::{
        CascadeShadowConfigBuilder, DirectionalLightShadowMap, ScreenSpaceAmbientOcclusionBundle,
    },
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::*;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, animate_light_direction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_transform = Transform::from_xyz(0.2, 0.6, 0.3);
    camera_transform.rotate(Quat::from_euler(EulerRot::XYZ, -0.8, 0.7, 1.3));
    commands
        .spawn((
            Camera3dBundle {
                transform: camera_transform,
                // .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
                ..default()
            },
            EnvironmentMapLight {
                diffuse_map: asset_server.load("environment_maps/diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("environment_maps/specular_rgb9e5_zstd.ktx2"),
                intensity: 1000.0,
            },
        ))
        .insert(ScreenSpaceAmbientOcclusionBundle::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/SSAO/PlaneEngine/scene.gltf#Scene0")),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    // for mut transform in &mut query {
    //     transform.rotation = Quat::from_euler(
    //         EulerRot::ZYX,
    //         0.0,
    //         time.elapsed_seconds() * PI / 5.0,
    //         -FRAC_PI_4,
    //     );
    // }
}
