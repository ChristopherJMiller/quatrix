mod music;
mod sfx;

use bevy::prelude::*;
use music::play_music;
use sfx::{on_play_sfx, SfxTable};

pub use sfx::{PlaySoundEffect, SoundEffect};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SfxTable>()
            .add_event::<PlaySoundEffect>()
            .add_systems(Startup, (SoundEffect::fill_sfx_table, play_music))
            .add_systems(Update, on_play_sfx);
    }
}
