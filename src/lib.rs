use bevy::{prelude::*, window::WindowResolution};

// mod control;

mod snake;
use food::FoodPlugin;
use snake::SnakePlugin;

mod food;

mod wall;
use wall::{check_wall_collision, wall_spawn};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Over,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Playing,
    #[default]
    Paused,
}

#[derive(Clone, Copy)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
pub struct AppResource {
    pub direction: SnakeDirection,
}

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const MISSING_COLOR: Color = Color::FUCHSIA;
pub const CELL_SIZE: Vec2 = Vec2::splat(50.);

#[derive(Component, Debug)]
pub struct Cell(u32, u32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .add_state::<AppState>()
            .add_state::<GameState>()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH, HEIGHT),
                    title: "贪吃蛇".to_string(),
                    resizable: false,
                    ..Default::default()
                }),
                ..default()
            }))
            .add_plugin(SnakePlugin)
            .add_plugin(FoodPlugin)
            .insert_resource(AppResource {
                direction: SnakeDirection::Right,
            })
            .add_systems((control_input, check_wall_collision).in_set(OnUpdate(GameState::Playing)))
            .add_system(game_pause.in_set(OnUpdate(AppState::InGame)))
            .add_startup_system(setup);
    }
}

pub const X_LENGTH: i32 = 15;
pub const Y_LENGTH: i32 = 10;

fn setup(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = 250.;
    camera.transform.translation.y = 250.;
    commands.spawn(camera);
    for i in 0..X_LENGTH {
        for j in 0..Y_LENGTH {
            if i != 0 && j != 0 && i != X_LENGTH - 1 && j != Y_LENGTH - 1 {
                continue;
            }
            wall_spawn(&mut commands, i as f32 * 50., j as f32 * 50.);
        }
    }
    app_state.set(AppState::InGame);
    game_state.set(GameState::Playing);
}

fn control_input(
    mut resource: ResMut<AppResource>,
    keyboard_input: Res<Input<KeyCode>>,
    // mouse_button_input: Res<Input<MouseButton>>,
    // touch_input: Res<Touches>,
) {
    resource.direction = if keyboard_input.just_pressed(KeyCode::Up) {
        SnakeDirection::Up
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        SnakeDirection::Down
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        SnakeDirection::Left
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        SnakeDirection::Right
    } else {
        resource.direction
    };
}

fn game_pause(
    state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    // touch_input: Res<Touches>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.0 {
            GameState::Playing => game_state.set(GameState::Paused),
            GameState::Paused => game_state.set(GameState::Playing),
        }
    }
}
