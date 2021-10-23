use bevy::prelude::{Plugin, AppBuilder, WindowDescriptor, ClearColor, Color};
pub struct WindowConf;

impl Plugin for WindowConf {
    fn build(&self, app: &mut AppBuilder) {
        let desc = WindowDescriptor {
            title: String::from("Bevy Snake"),
            width: 500.,
            height: 500.,
            ..Default::default()
        };
        app.insert_resource(desc);
        app.insert_resource(ClearColor(Color::BLACK));
    }
}