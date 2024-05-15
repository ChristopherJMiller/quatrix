use bevy::{
    app::{App, Plugin},
    asset::{AssetServer, Handle},
    ecs::system::Resource,
    render::texture::Image,
};

#[derive(Resource)]
pub struct BoardSprites {
    pub open: Handle<Image>,
    pub closed: Handle<Image>,
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        let asset_server = app.world.resource::<AssetServer>();

        app.insert_resource(BoardSprites {
            open: asset_server.load("sprite/open.png"),
            closed: asset_server.load("sprite/closed.png"),
        });
    }
}
