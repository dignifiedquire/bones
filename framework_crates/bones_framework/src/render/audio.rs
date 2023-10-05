//! Audio components.

use std::{io::Cursor, sync::Arc};

use crate::prelude::*;

pub use kira::{self, sound::static_sound::StaticSoundData};
use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager as KiraAudioManager},
    sound::SoundData,
};

/// The game plugin for the audio system.
pub fn game_plugin(game: &mut Game) {
    game.insert_shared_resource(AudioManager::default());
    game.init_shared_resource::<AssetServer>()
        .register_asset::<AudioSource>();
}

/// The audio manager resource which can be used to play sounds.
#[derive(HasSchema, Clone, Deref, DerefMut)]
pub struct AudioManager(Arc<AtomicCell<KiraAudioManager>>);
impl Default for AudioManager {
    fn default() -> Self {
        Self(Arc::new(AtomicCell::new(
            KiraAudioManager::<CpalBackend>::new(default()).unwrap(),
        )))
    }
}

/// The audio source asset type, contains no data, but [`Handle<AudioSource>`] is still useful
/// because it uniquely represents a sound/music that may be played outside of bones.
#[derive(Clone, HasSchema, Debug)]
#[schema(no_default)]
#[type_data(asset_loader(["ogg", "mp3", "flac", "wav"], AudioLoader))]
pub struct AudioSource(pub StaticSoundData);

impl SoundData for &AudioSource {
    type Error = <StaticSoundData as SoundData>::Error;
    type Handle = <StaticSoundData as SoundData>::Handle;

    fn into_sound(self) -> Result<(Box<dyn kira::sound::Sound>, Self::Handle), Self::Error> {
        self.0.clone().into_sound()
    }
}

/// The audio file asset loader.
pub struct AudioLoader;
impl AssetLoader for AudioLoader {
    fn load(&self, _ctx: AssetLoadCtx, bytes: &[u8]) -> anyhow::Result<SchemaBox> {
        let data = StaticSoundData::from_cursor(Cursor::new(bytes.to_vec()), default())?;
        Ok(SchemaBox::new(AudioSource(data)))
    }
}