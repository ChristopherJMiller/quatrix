use bevy::{
    app::{App, Plugin, Startup},
    asset::{AssetServer, Handle, LoadState},
    prelude::Res,
};
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

pub struct MusicLoop<S: ToString> {
    pub asset_path: S,
    pub bpm: usize,
}

impl<S: ToString> MusicLoop<S> {
    pub fn load(self, asset_server: &AssetServer) -> MusicLoopHandle {
        let handle = asset_server.load(self.asset_path.to_string());
        MusicLoopHandle {
            handle,
            bpm: self.bpm,
        }
    }
}

pub struct MusicLoopHandle {
    pub handle: Handle<AudioSource>,
    pub bpm: usize,
}

pub struct Song {
    pub bpm: usize,
    pub tracks: Vec<MusicLoopHandle>,
}

impl MusicLoopHandle {
    pub fn loaded(&self, asset_server: &AssetServer) -> bool {
        asset_server
            .get_load_state(self.handle.id())
            .is_some_and(|state| state == LoadState::Loaded)
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("audio/music/736740_piano.wav"))
        .looped();
}

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_background_audio);
    }
}
