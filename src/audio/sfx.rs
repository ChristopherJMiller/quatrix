use std::collections::HashMap;

use bevy::prelude::*;
use bevy_kira_audio::prelude::Volume;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SoundEffect {
    UiHover,
    UiClick,
    Rotate,
    Drop,
    Clear,
    LevelUp,
    RankBoost,
}

impl SoundEffect {
    pub fn audio_file(&self) -> &'static str {
        match self {
            SoundEffect::UiHover => "sfx/uiHover.ogg",
            SoundEffect::UiClick => "sfx/uiClick.ogg",
            SoundEffect::Rotate => "sfx/rotate.ogg",
            SoundEffect::Drop => "sfx/drop.ogg",
            SoundEffect::Clear => "sfx/clear.ogg",
            SoundEffect::LevelUp => "sfx/levelUp.ogg",
            SoundEffect::RankBoost => "sfx/rankBoost.ogg",
        }
    }

    pub fn fill_sfx_table(asset_server: Res<AssetServer>, mut sfx_table: ResMut<SfxTable>) {
        for file in Self::iter() {
            sfx_table
                .0
                .insert(file, asset_server.load(file.audio_file()));
        }
    }
}

#[derive(Default, Resource)]
pub struct SfxTable(pub HashMap<SoundEffect, Handle<bevy_kira_audio::AudioSource>>);

#[derive(Event)]
pub struct PlaySoundEffect(pub SoundEffect);

pub fn on_play_sfx(
    audio: Res<Audio>,
    sfx_table: Res<SfxTable>,
    mut events: EventReader<PlaySoundEffect>,
) {
    for evt in events.read() {
        audio
            .play(
                sfx_table
                    .0
                    .get(&evt.0)
                    .expect("Failed to find sound effect. This should never happen.")
                    .clone_weak(),
            )
            .with_volume(Volume::Amplitude(2.0));
    }
}
