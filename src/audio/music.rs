use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioEasing, AudioTween};

const MAIN_TRACK: &'static str = "music/Three Red Hearts - Go (No Vocal).ogg";

pub fn play_music(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio
        .play(asset_server.load(MAIN_TRACK))
        .looped()
        .fade_in(AudioTween::new(
            Duration::new(1, 0),
            AudioEasing::InOutPowi(5),
        ));
}
