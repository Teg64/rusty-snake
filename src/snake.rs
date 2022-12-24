use bevy::prelude::*;

use crate::{
    controller::{EatEvent, SnakeDirection},
    grid::{Position, ScaleSize},
};

const SNAKE_COLOR: Color = Color::ANTIQUE_WHITE;
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(217. / 255., 195. / 255., 167. / 255.);

#[derive(Component)]
pub struct SnakeHead {
    pub direction: SnakeDirection,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeSegmentsVec(Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeSegmentsVec::default())
            .insert_resource(LastTailPosition::default())
            .add_startup_system(spawn_snake);
    }
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegmentsVec>) {
    *segments = SnakeSegmentsVec(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: SnakeDirection::Up,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(ScaleSize::square(0.8))
            .insert(Name::from("Snake"))
            .id(),
        spawn_segment(&mut commands, Position { x: 3, y: 2 }),
    ]);
}

fn spawn_segment(commands: &mut Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(ScaleSize::square(0.65))
        .id()
}

pub fn grow_snake(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegmentsVec>,
    mut eat_reader: EventReader<EatEvent>,
) {
    if eat_reader.iter().next().is_some() {
        segments.push(spawn_segment(&mut commands, last_tail_position.0.unwrap()));
    }
}
