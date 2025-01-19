use bevy::{asset::LoadState, prelude::*, render::render_asset::RenderAssetUsages};
use image::DynamicImage;
use noise::NoiseFn;

use super::board::state::GameState;

/// Represents a sprite that will be a gradient background that spans the camera
#[derive(Component)]
pub struct GradientBackground(pub u32);

impl Default for GradientBackground {
    fn default() -> Self {
        Self(0)
    }
}

impl GradientBackground {
    pub fn build(&self, seed: u32) -> DynamicImage {
        let grad = colorgrad::magma();

        let scale = 0.05;

        let ns = noise::OpenSimplex::new(self.0 + seed);
        let mut imgbuf = image::ImageBuffer::new(32, 32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let t = ns.get([x as f64 * scale, y as f64 * scale]);
            let rgba = grad.at(t.remap(-0.5, 0.5, 0.0, 1.0)).to_rgba8();
            *pixel = image::Rgba(rgba.map(|x| (x as f32 * 0.75) as u8));
        }

        DynamicImage::ImageRgba8(imgbuf)
    }

    pub fn load(&self, asset_server: &Res<AssetServer>, seed: u32) -> Handle<Image> {
        let image = self.build(seed);
        let image = Image::from_dynamic(image, false, RenderAssetUsages::default());
        let handle = asset_server.add(image);
        handle
    }
}

fn update_gradient_size(
    assets: Res<Assets<Image>>,
    mut gradient_trans: Query<(&mut Transform, &Handle<Image>), With<GradientBackground>>,
    single_camera: Query<&Camera>,
) {
    if let Ok(single_camera) = single_camera.get_single() {
        if let Some(size) = single_camera.physical_viewport_size() {
            gradient_trans
                .iter_mut()
                .for_each(|(mut trans, sprite_image)| {
                    if let Some(image) = assets.get(sprite_image) {
                        trans.scale = (Vec2::new(size.x as f32, size.y as f32) / image.size_f32())
                            .extend(0.0);
                    }
                });
        }
    }
}

fn update_gradient(
    // Gradient Image as background
    mut gradient_sprite: Query<(&mut Handle<Image>, &GradientBackground)>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    // Counts how much time has passed, which acts as a "rerender frame rate" for the background
    mut time_passed: Local<f32>,
    // Handle to wait to be loaded before replacing the background
    mut loading_handle: Local<Option<Handle<Image>>>,
    // The seed to add to slowly via game state
    mut seed: Local<u32>,
    // Game state used to accelerate the seed
    game_state: Res<GameState>,
) {
    *time_passed += time.delta_seconds() * 5.0;

    *seed +=
        (time.delta_seconds() * game_state.data_board.score().current_mult()).floor() as u32 * 100;

    if *time_passed >= 0.1 {
        *loading_handle = Some(gradient_sprite.single().1.load(&asset_server, *seed))
    }

    if loading_handle.as_ref().is_some_and(|x| {
        asset_server
            .get_load_state(x)
            .is_some_and(|x| x == LoadState::Loaded)
    }) {
        *time_passed = 0.0;
        let handle = loading_handle.take().unwrap();

        let (mut image, _) = gradient_sprite.single_mut();

        *image = handle;
    }
}

fn spawn_gradient(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gradient = GradientBackground::default();

    commands
        .spawn(SpriteBundle {
            texture: gradient.load(&asset_server, 0),
            transform: Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::ONE),
            ..default()
        })
        .insert(gradient);
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gradient)
            .add_systems(Update, update_gradient_size)
            .add_systems(FixedUpdate, update_gradient);
    }
}
