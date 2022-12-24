use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    food::{spawn_new_food, Food},
    grid::{Position, GRID_HEIGHT, GRID_WIDTH},
    snake::{grow_snake, spawn_snake, LastTailPosition, SnakeHead, SnakeSegment, SnakeSegmentsVec},
    SCREEN_WRAP,
};

#[derive(PartialEq, Clone, Copy)]
pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl SnakeDirection {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub struct EatEvent;

struct GameOverEvent;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(snake_controller.before(move_snake))
            .add_event::<EatEvent>()
            .add_event::<GameOverEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.15))
                    .with_system(move_snake)
                    .with_system(eat_food.after(move_snake))
                    .with_system(grow_snake.after(eat_food))
                    .with_system(spawn_new_food.after(grow_snake)),
            )
            .add_system(game_over.after(move_snake));
    }
}

/// Controls the snake's movement direction
fn snake_controller(keyboard: Res<Input<KeyCode>>, mut head: Query<&mut SnakeHead>) {
    // Todo: Fix crash on game_over
    let mut head = head.single_mut();

    let try_direction = if keyboard.pressed(KeyCode::Left) {
        SnakeDirection::Left
    } else if keyboard.pressed(KeyCode::Right) {
        SnakeDirection::Right
    } else if keyboard.pressed(KeyCode::Up) {
        SnakeDirection::Up
    } else if keyboard.pressed(KeyCode::Down) {
        SnakeDirection::Down
    } else {
        head.direction
    };

    /*
    Todo: Fix the 180 turn bug;
    *   Separate the direction the snake is currently moving and the last inputted direction
    */
    if try_direction != head.direction.opposite() {
        head.direction = try_direction;
    }
}

/// Moves the snake
fn move_snake(
    mut head: Query<(Entity, &SnakeHead)>,
    segments: ResMut<SnakeSegmentsVec>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    // Todo: Using Vec.get() is gross. Refactor the snake to avoid this. Actually this entire function is gross; kill it with fire
    let (head_entity, head) = head.single_mut();
    let segment_positions = segments
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();
    let mut head_position = positions.get_mut(head_entity).unwrap();

    match head.direction {
        SnakeDirection::Left => head_position.x -= 1,
        SnakeDirection::Right => head_position.x += 1,
        SnakeDirection::Down => head_position.y -= 1,
        SnakeDirection::Up => head_position.y += 1,
    }

    // Check for gameover
    if (!SCREEN_WRAP
        && (head_position.x < 0
            || head_position.y < 0
            || head_position.x as u32 > GRID_WIDTH
            || head_position.y as u32 > GRID_HEIGHT))
        || segment_positions.contains(&head_position)
    {
        game_over_writer.send(GameOverEvent);
    }

    segment_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(position, segment)| {
            *positions.get_mut(*segment).unwrap() = *position;
        });

    *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
}

fn eat_food(
    mut commands: Commands,
    mut eat_writer: EventWriter<EatEvent>,
    food_position: Query<(Entity, &Position), With<Food>>,
    head_position: Query<&Position, With<SnakeHead>>,
) {
    let head_position = head_position.single();

    // Todo: Things get break if we use food_position.single() as above. Restructure to avoid this?
    for (food_entity, food_position) in food_position.iter() {
        if food_position == head_position {
            commands.entity(food_entity).despawn_recursive();
            eat_writer.send(EatEvent);
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    segments_resource: ResMut<SnakeSegmentsVec>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if game_over_reader.iter().next().is_some() {
        println!("hi");
        for entity in segments.iter().chain(food.iter()) {
            commands.entity(entity).despawn_recursive();
        }
        spawn_snake(commands, segments_resource);
    }
}
