pub mod camera;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "nanonaut".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup)
        .add_startup_stage("player_setup", SystemStage::single(spawn_player))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

// Systems
fn spawn_player(mut commands: Commands) {
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() }.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }.into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                color: Color::GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player);
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(camera::new_camera_2d());
}
