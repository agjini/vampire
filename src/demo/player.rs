//! Player-specific behavior.

use crate::{
    asset_tracking::LoadResource, demo::movement::MovementController, AppSystems, PausableSystems,
};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, DebugRender, LinearVelocity, LockedAxes, RigidBody,
};
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;
const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
const AIM_RADIUS: f32 = 75.;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect)]
enum AimSource {
    Mouse,
    Gamepad,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct AimRig {
    radius: f32,
    source: AimSource,
}


pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(
    max_speed: f32,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> impl Bundle {
    let body_mesh = meshes.add(Rectangle::new(32., 32.));
    let body_material = materials.add(ColorMaterial::from(Color::WHITE));
    let aim_circle_mesh = meshes.add(Circle::new(AIM_RADIUS));
    let aime_circle_material = materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.2)));
    let cross_horizontal_mesh = meshes.add(Rectangle::new(16., 2.));
    let cross_vertical_mesh = meshes.add(Rectangle::new(2., 16.));
    let cross_material = materials.add(ColorMaterial::from(Color::WHITE));
    (
        Name::new("Player"),
        Player,
        Mesh2d(body_mesh),
        MeshMaterial2d(body_material),
        Transform::from_scale(Vec2::splat(4.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 32.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        DebugRender::default().with_collider_color(Color::WHITE),
        children![
            (
                Name::new("AimRig"),
                Transform::default(),
                AimRig {
                    radius: AIM_RADIUS,
                    source: AimSource::Mouse,
                },
                children![
                    (
                        Name::new("AimCircle"),
                        Mesh2d(aim_circle_mesh),
                        MeshMaterial2d(aime_circle_material),
                        Transform::from_xyz(0.0, 0.0, -0.1),
                    ),
                    (
                        Name::new("AimCross"),
                        Transform::from_translation(Vec3::new(AIM_RADIUS, 0., 0.1)),
                        children![
                            (
                                Mesh2d(cross_horizontal_mesh),
                                MeshMaterial2d(cross_material.clone()),
                            ),
                            (
                                Mesh2d(cross_vertical_mesh),
                                MeshMaterial2d(cross_material),
                            ),
                        ],
                    )
                ]
            )
        ]
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;
    if input.any_pressed(UP) {
        intent.y += 1.0;
    }
    if input.any_pressed(DOWN) {
        intent.y -= 1.0;
    }
    if input.any_pressed(LEFT) {
        intent.x -= 1.0;
    }
    if input.any_pressed(RIGHT) {
        intent.x += 1.0;
    }
    let intent = intent.normalize_or_zero();

    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
