mod snake;
mod window_conf;

use bevy::prelude::*;

fn main() {
    let mut app = App::build();
    app.add_plugin(window_conf::WindowConf);

    app.add_startup_system(setup_camera.system());   

    app.add_plugins(DefaultPlugins);
    app.add_plugin(snake::SnakePlugin);
        
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}