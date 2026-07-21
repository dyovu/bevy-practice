#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::basic::SILVER,
    input::common_conditions::{input_just_pressed, input_toggle_active},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};


mod renderer_3d_tutorial;
use renderer_3d_tutorial::{setup, rotate, advance_rows, toggle_wireframe};
mod render_tetrahedron;
use render_tetrahedron::{setup_tetra, rotate_tetra};
mod fps_practice;
use fps_practice::{spawn_player_view, mouse_look, move_body};
mod first_point_shooter;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            WireframePlugin::default(),
        ))
        // .add_plugins(HelloPlugin)
        .add_systems(Startup, setup_tetra)
        .add_systems(Update, rotate_tetra)
        .add_systems(Startup, spawn_player_view)
        .add_systems(Update, (mouse_look, move_body))
        .add_systems(Update,
            #[cfg(not(target_arch = "wasm32"))]
            toggle_wireframe
        )
        .run();
}


// plugin
// システムをまとめたもの
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

// システム, ロジックの部分
// 通常の関数. entityをどう動かすとか何表示するとかの設定
fn hello_world() {
    println!("hello world!");
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}


// etityの作成と登録
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}


// 自分のresource設定
#[derive(Resource)]
struct GreetTimer(Timer);
