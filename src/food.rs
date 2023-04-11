use bevy::{prelude::*, sprite::collide_aabb::collide};

use rand::prelude::*;

use crate::{snake::SnakeHead, AppState, Cell, CELL_SIZE, X_LENGTH, Y_LENGTH};

#[derive(Component)]
pub struct Food;

pub struct EatFoodEvent(pub Vec3);

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatFoodEvent>()
            .add_system(
                food_spawn
                    .in_set(OnUpdate(AppState::InGame))
                    .run_if(cheak_food_exist),
            )
            .add_system(check_food_collision.in_set(OnUpdate(AppState::InGame)));
    }
}

fn food_spawn(mut commands: Commands, cells: Query<&Cell>) {
    if cells.iter().len() >= (X_LENGTH * Y_LENGTH) as usize {
        return;
    }
    let mut rng = rand::thread_rng();
    let cell = 'outer: loop {
        let possible_cell = Cell(
            (rng.gen_range(0..X_LENGTH) * 50).try_into().unwrap(),
            (rng.gen_range(0..Y_LENGTH) * 50).try_into().unwrap(),
        );
        for cell in cells.iter() {
            if cell.0 == possible_cell.0 && cell.1 == possible_cell.1 {
                continue 'outer;
            }
        }
        break possible_cell;
    };
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(CELL_SIZE),
                color: Color::YELLOW,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(cell.0 as f32, cell.1 as f32, 0.)),
            ..Default::default()
        },
        cell,
        Food,
    ));
}

fn cheak_food_exist(food: Query<&Food>) -> bool {
    food.is_empty()
}

fn check_food_collision(
    mut commands: Commands,
    mut event_writer: EventWriter<EatFoodEvent>,
    head: Query<&Transform, With<SnakeHead>>,
    food: Query<(&Transform, Entity), With<Food>>,
) {
    let Ok(head) = head.get_single() else {
        return;
    };
    let Ok(food) = food.get_single() else {
        return;
    };
    if collide(head.translation, CELL_SIZE, food.0.translation, CELL_SIZE).is_some() {
        commands.entity(food.1).despawn_recursive();
        event_writer.send(EatFoodEvent(food.0.translation));
    }
}
