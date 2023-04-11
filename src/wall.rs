use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, Sprite, SpriteBundle},
};

use crate::{snake::SnakeHead, AppState, Cell, CELL_SIZE, MISSING_COLOR};

#[derive(Component)]
pub struct Wall;

pub fn wall_spawn(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(CELL_SIZE),
                color: Color::hex("FF00FF").unwrap_or(MISSING_COLOR),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3 { x, y, z: 0. }),
            ..Default::default()
        },
        Wall,
        Cell(x as u32, y as u32),
    ));
}

pub fn check_wall_collision(
    mut play_state: ResMut<NextState<AppState>>,
    head: Query<&Transform, With<SnakeHead>>,
    walls: Query<&Transform, With<Wall>>,
) {
    let Ok(head) = head.get_single() else {
        return;
    };
    for wall in &walls {
        if collide(head.translation, CELL_SIZE, wall.translation, CELL_SIZE).is_some() {
            play_state.set(AppState::Over);
        }
    }
}
