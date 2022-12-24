use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod controller;
mod food;
mod grid;
mod snake;

use controller::ControllerPlugin;
use food::FoodPlugin;
use grid::*;
use snake::SnakePlugin;

pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const SCREEN_WRAP: bool = true;
// pub const NUM_TILES: u32 = 11;
// pub const TILE_WIDTH: f32 = WINDOW_WIDTH / NUM_TILES as f32;

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: String::from("Rusty Snake"),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        // Inspector
        .add_plugin(WorldInspectorPlugin::new())
        // Systems and Plugins
        .add_startup_system(setup)
        .add_plugin(SnakePlugin)
        .add_plugin(GridPlugin)
        .add_plugin(FoodPlugin)
        .add_plugin(ControllerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // _tile_board(&mut commands);
}

/*
/// A helper function for displaying the tile board's configuration
fn _tile_board(commands: &mut Commands) {
    let colors = [Color::ANTIQUE_WHITE, Color::MAROON, Color::OLIVE];

    for i in 0..NUM_TILES {
        for j in 0..NUM_TILES {
            // First tile is green, others decided according to index
            let mut color_index = ((i + j) % 2) as usize;
            if i == 0 && j == 0 {
                color_index = 2;
            }

            // Position correction for tile's center
            let correction = 0.5 - NUM_TILES as f32 / 2.0;

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_WIDTH)),
                    color: colors[color_index],
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    TILE_WIDTH * (i as f32 + correction),
                    TILE_WIDTH * (j as f32 + correction),
                    0.0,
                )),
                ..default()
            });
        }
    }
}
*/
