use std::collections::HashSet;

use anyhow::{anyhow, bail};
use bevy::{
    app::{App, Plugin, Startup},
    asset::{AssetServer, Handle, LoadState},
    log::info,
    prelude::Res,
};
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

/// An unloaded loopable music track
pub struct MusicLoop<S: ToString> {
    /// Path to music asset
    pub asset_path: S,
    /// Beats per minute of the music asset
    pub bpm: usize,
}

impl<S: ToString> MusicLoop<S> {
    /// Loads a music loop, returning a handled version ([MusicLoopHandle])
    pub fn load(self, asset_server: &AssetServer) -> MusicLoopHandle {
        let handle = asset_server.load(self.asset_path.to_string());
        MusicLoopHandle {
            handle,
            bpm: self.bpm,
        }
    }
}

/// A loaded (or loading) audio source
pub struct MusicLoopHandle {
    /// Bevy Audio Source Handle
    pub handle: Handle<AudioSource>,
    /// Beats per minute of the Audio Source
    pub bpm: usize,
}

impl MusicLoopHandle {
    /// Is the music loop loaded?
    pub fn loaded(&self, asset_server: &AssetServer) -> bool {
        asset_server
            .get_load_state(self.handle.id())
            .is_some_and(|state| state == LoadState::Loaded)
    }
}

/// A backing song.
///
/// A song has a single tempo (declared via bpm) and has a series of tracks.
pub struct Song {
    /// Tempo of the song (in beats per minute)
    pub bpm: usize,
    /// Loaded tracks for the song
    pub tracks: Vec<MusicLoopHandle>,
}

impl Song {
    /// Attempt to build a song from music loops
    pub fn try_from_loops<S: ToString, MusicIntoIter: IntoIterator<Item = MusicLoop<S>>>(
        music_list: MusicIntoIter,
        asset_server: &AssetServer,
    ) -> anyhow::Result<Self> {
        let mut iter = music_list.into_iter();
        let found_bpms: HashSet<usize> = iter.by_ref().map(|music_loop| music_loop.bpm).collect();
        if found_bpms.is_empty() {
            bail!("Music list was empty");
        }

        let smallest_bpm = found_bpms
            .iter()
            .min()
            .expect("Set was somehow empty. This shouldn't happen");

        if found_bpms.len() > 1 {
            info!("Song has multiple bpms, ensuring they are all divisible");
            let everything_is_divisible = found_bpms.iter().all(|bpm| bpm % smallest_bpm == 0);
            if !everything_is_divisible {
                bail!("Music loop contains multiple bpms that aren't all divisible by each other {found_bpms:?}");
            }
        }

        let handles = iter
            .map(|music_loop| music_loop.load(asset_server))
            .collect();

        Ok(Song {
            bpm: *smallest_bpm,
            tracks: handles,
        })
    }

    /// Plays a song if all tracks are loaded
    pub fn play(&self, asset_server: &AssetServer, audio: &Res<Audio>) -> anyhow::Result<()> {
        let loops_loaded = self.tracks.iter().all(|track| track.loaded(asset_server));

        if loops_loaded {
            self.tracks.iter().for_each(|track| {
                audio.play(track.handle.clone()).looped();
            });

            Ok(())
        } else {
            bail!("Not all tracks are loaded");
        }
    }
}

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {}
}
