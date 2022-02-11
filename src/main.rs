use bevy::prelude::*;

// Components
fn hello_world() {
    println!("hello world!");
}

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
        .run();
}

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

// Systems
fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
