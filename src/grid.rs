use bevy::prelude::*;

use crate::{SCREEN_WRAP, WINDOW_HEIGHT, WINDOW_WIDTH};

pub const GRID_WIDTH: u32 = 11;
pub const GRID_HEIGHT: u32 = 11;
pub const TILE_WIDTH: f32 = WINDOW_WIDTH / GRID_WIDTH as f32;
pub const TILE_HEIGHT: f32 = WINDOW_WIDTH / GRID_HEIGHT as f32;

/// A position in the grid
#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Debug)]
#[reflect(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Size relative to a single grid tile
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ScaleSize {
    width: f32,
    height: f32,
}

impl ScaleSize {
    pub fn square(x: f32) -> Self {
        ScaleSize {
            width: x,
            height: x,
        }
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>()
            .register_type::<ScaleSize>()
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(snap_to_grid)
                    .with_system(scale_sizes),
            );
    }
}

// Scales anything with a ScaleSize relative to the
fn scale_sizes(mut sizes: Query<(&ScaleSize, &mut Transform)>) {
    for (size_scale, mut transform) in &mut sizes {
        transform.scale = Vec3::new(
            size_scale.width / GRID_WIDTH as f32 * WINDOW_WIDTH as f32,
            size_scale.height / GRID_HEIGHT as f32 * WINDOW_HEIGHT as f32,
            1.,
        );
    }
}

/// Moves entities onto the grid based on their Position
fn snap_to_grid(mut positions: Query<(&mut Position, &mut Transform)>) {
    for (mut position, mut transform) in &mut positions {
        // Add screen wrap just because :)
        if SCREEN_WRAP {
            position.x = pos_modulo(position.x, GRID_WIDTH as i32);
            position.y = pos_modulo(position.y, GRID_HEIGHT as i32);
        }

        transform.translation = Vec3::new(
            grid_position(position.x, GRID_WIDTH, TILE_WIDTH),
            grid_position(position.y, GRID_HEIGHT, TILE_HEIGHT),
            0.,
        );
    }
}

/// Helper function for computing the exact position of a given tile, given its horizontal or vertical index
fn grid_position(tile_index: i32, tiles_in_grid: u32, tile_size: f32) -> f32 {
    tile_size * (tile_index as f32 + 0.5 - tiles_in_grid as f32 / 2.)
}

/// Performs the modulo operation, adjusted to always return a positive result
fn pos_modulo(dividend: i32, divisor: i32) -> i32 {
    let mut res = dividend % divisor;

    if res < 0 {
        res += divisor;
    }

    res
}
