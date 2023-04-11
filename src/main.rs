use bevy::prelude::App;
use snake::GamePlugin;


fn main() {
    App::new().add_plugin(GamePlugin).run();
}