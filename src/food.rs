use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    controller::EatEvent,
    grid::{Position, ScaleSize, GRID_HEIGHT, GRID_WIDTH},
};

const FOOD_COLOR: Color = Color::rgb(210. / 255., 101. / 255., 126. / 255.);

#[derive(Component)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_initial_food);
    }
}

fn spawn_initial_food(mut commands: Commands, used_positions: Query<&Position>) {
    food_spawner(&mut commands, used_positions);
}

/// Helper function for spawning new food
fn food_spawner(commands: &mut Commands, used_positions: Query<&Position>) {
    let used_positions: Vec<&Position> = used_positions.iter().collect();

    // Todo: Is there a better way to do this? Pre-generate a 2D lookup array and remove used positions from a copy of it?
    let mut open_positions = vec![];

    for i in 0..GRID_WIDTH {
        for j in 0..GRID_HEIGHT {
            let candidate = Position {
                x: i as i32,
                y: j as i32,
            };

            if !used_positions.contains(&&candidate) {
                open_positions.push(candidate);
            }
        }
    }

    let spawn_position = open_positions.choose(&mut rand::thread_rng()).unwrap();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Position {
            x: spawn_position.x,
            y: spawn_position.y,
        })
        .insert(Food)
        .insert(ScaleSize::square(0.6))
        .insert(Name::from("Food"));
}

pub fn spawn_new_food(
    mut commands: Commands,
    mut eat_reader: EventReader<EatEvent>,
    used_positions: Query<&Position>,
) {
    if eat_reader.iter().next().is_some() {
        food_spawner(&mut commands, used_positions);
    }
}
