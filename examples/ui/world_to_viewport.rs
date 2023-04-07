//! This example demonstrates how to use the `Camera::world_to_viewport` method.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_labels, rotate_camera))
        .run();
}

#[derive(Component)]
struct Label {
    target: Entity,
}

fn update_labels(
    mut label_query: Query<(&mut Style, &Label)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<&Transform>,
) {
    let (camera, camera_global_transform) = camera_query.single();

    for (mut style, label) in &mut label_query {
        let transform = transforms.get(label.target).unwrap();
        let position = transform.translation + Vec3::Y * 0.5;

        let Some(viewport_position) = camera.world_to_viewport(camera_global_transform, position) else { continue };

        style.top = Val::Px(viewport_position.y);
        style.left = Val::Px(viewport_position.x);
    }
}

fn rotate_camera(mut camera_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    camera_query.single_mut().rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_y(time.delta_seconds() * 0.25),
    );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // cube
    let cube = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 1.0),
            ..default()
        })
        .id();

    // sphere
    let sphere = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 0.5,
                    ..default()
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, -1.0),
            ..default()
        })
        .id();

    let mut spawn_label = |target: Entity, label: &str| {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Label { target },
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        label,
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.),
                        ..default()
                    })
                    .with_text_alignment(TextAlignment::Center),
                );
            });
    };

    spawn_label(cube, "Cube");
    spawn_label(sphere, "Sphere");
}
