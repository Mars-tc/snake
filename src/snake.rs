use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    food::EatFoodEvent, AppResource, AppState, Cell, GameState, SnakeDirection, CELL_SIZE,
};

#[derive(Resource)]
pub struct Snake {
    pub body: Vec<Vec2>,
}

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeBody;

const START: Vec2 = Vec2::splat(250.);

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(snake_spawn.in_schedule(OnEnter(AppState::InGame)))
            .add_system(stronger.in_set(OnUpdate(GameState::Playing)))
            .add_system(
                snake_move
                    .in_set(OnUpdate(GameState::Playing))
                    .run_if(on_timer(Duration::from_secs_f32(0.5))),
            );
    }
}

fn snake_spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(CELL_SIZE),
                ..Default::default()
            },
            transform: Transform::from_translation(START.extend(0.)),
            ..Default::default()
        },
        SnakeHead,
        SnakeBody,
        Cell(0, 0),
    ));
    commands.insert_resource(Snake { body: vec![START] });
}

fn snake_move(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    head: Query<Entity, With<SnakeHead>>,
    mut transforms: Query<(&mut Transform, Entity), With<SnakeBody>>,
    resource: Res<AppResource>,
) {
    let Ok(head) = head.get_single() else {
        return;
    };
    let length = snake.body.len();
    let tail = snake.body[length - 1].extend(0.);
    for (mut transform, entity) in &mut transforms {
        if transform.translation == tail {
            let next = get_next_position(snake.body[0], &resource.direction);
            transform.translation = next;
            snake.body.pop();
            snake.body.insert(0, next.truncate());
            commands.entity(head).remove::<SnakeHead>();
            commands.entity(entity).insert(SnakeHead);
            break;
        }
    }
}

fn get_next_position(snake_head: Vec2, direction: &SnakeDirection) -> Vec3 {
    match direction {
        SnakeDirection::Up => snake_head + Vec2::new(0., 50.),
        SnakeDirection::Down => snake_head - Vec2::new(0., 50.),
        SnakeDirection::Left => snake_head - Vec2::new(50., 0.),
        SnakeDirection::Right => snake_head + Vec2::new(50., 0.),
    }
    .extend(0.)
}

fn stronger(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    head: Query<Entity, With<SnakeHead>>,
    mut eat_events: EventReader<EatFoodEvent>,
) {
    let Ok(head) = head.get_single() else {
        return;
    };
    for event in eat_events.iter() {
        commands.entity(head).remove::<SnakeHead>();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(CELL_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_translation(event.0),
                ..Default::default()
            },
            SnakeHead,
            SnakeBody,
            Cell(0, 0),
        ));
        snake.body.insert(0, event.0.truncate());
    }
}
