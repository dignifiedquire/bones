use std::path::{Path, PathBuf};

use bones_utils::HashMap;

/// [`AssetIo`] is a trait that is implemented for backends capable of loading all the games assets
/// and returning the raw bytes stored in asset files.
pub trait AssetIo: Sync + Send {
    /// List the names of the non-core asset pack folders that are installed.
    ///
    /// These names, are not necessarily the names of the pack, but the names of the folders that
    /// they are located in. These names can be used to load files from the pack in the
    /// [`load_file()`][Self::load_file] method.
    fn enumerate_packs(&self) -> anyhow::Result<Vec<String>>;
    /// Get the binary contents of an asset.
    ///
    /// The `pack_folder` is the name of a folder returned by
    /// [`enumerate_packs()`][Self::enumerate_packs], or [`None`] to refer to the core pack.
    fn load_file(&self, pack_folder: Option<&str>, path: &Path) -> anyhow::Result<Vec<u8>>;

    /// Subscribe to asset changes.
    fn watch(&self) -> Option<async_channel::Receiver<AssetChange>>;
}

/// Change event returned by [`AssetIo::watch`].
#[derive(Clone, Debug)]
pub struct AssetChange {
    /// The path of the asset that changed.
    pub path: PathBuf,
    /// The pack that the changed asset was in, or [`None`] if it was the core pack.
    pub pack: Option<String>,
}

/// [`AssetIo`] implementation that loads from the filesystem.
#[cfg(not(target_arch = "wasm32"))]
pub struct FileAssetIo {
    /// The directory to load the core asset pack.
    pub core_dir: PathBuf,
    /// The directory to load the asset packs from.
    pub packs_dir: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl AssetIo for FileAssetIo {
    fn enumerate_packs(&self) -> anyhow::Result<Vec<String>> {
        if !self.packs_dir.exists() {
            return Ok(Vec::new());
        }

        // List the folders in the asset packs dir.
        let dirs = std::fs::read_dir(&self.packs_dir)?
            .map(|entry| {
                let entry = entry?;
                let name = entry
                    .file_name()
                    .to_str()
                    .expect("non-unicode filename")
                    .to_owned();
                Ok::<_, std::io::Error>(name)
            })
            .filter(|x| {
                x.as_ref()
                    .map(|name| self.packs_dir.join(name).is_dir())
                    .unwrap_or(true)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(dirs)
    }

    fn load_file(&self, pack_folder: Option<&str>, path: &Path) -> anyhow::Result<Vec<u8>> {
        let base_dir = match pack_folder {
            Some(folder) => self.packs_dir.join(folder),
            None => self.core_dir.clone(),
        };
        let path = base_dir.join(path);
        Ok(std::fs::read(path)?)
    }

    fn watch(&self) -> Option<async_channel::Receiver<AssetChange>> {
        // TODO: Implement filesystem watcher for asset loader.
        None
    }
}

/// Dummy [`AssetIo`] implementation used for debugging or as a placeholder.
pub struct DummyIo {
    core: HashMap<PathBuf, Vec<u8>>,
    packs: HashMap<String, HashMap<PathBuf, Vec<u8>>>,
}

impl DummyIo {
    /// Initialize a new [`DummyIo`] from an iterator of `(string_path, byte_data)` items.
    pub fn new<'a, I: IntoIterator<Item = (&'a str, Vec<u8>)>>(core: I) -> Self {
        Self {
            core: core
                .into_iter()
                .map(|(p, d)| (PathBuf::from(p), d))
                .collect(),
            packs: Default::default(),
        }
    }
}

impl AssetIo for DummyIo {
    fn enumerate_packs(&self) -> anyhow::Result<Vec<String>> {
        Ok(self.packs.keys().cloned().collect())
    }

    fn load_file(
        &self,
        pack_folder: Option<&str>,
        path: &std::path::Path,
    ) -> anyhow::Result<Vec<u8>> {
        let err = || {
            anyhow::format_err!(
                "File not found: `{:?}` in pack `{:?}`",
                path,
                pack_folder.unwrap_or("[core]")
            )
        };
        if let Some(pack_folder) = pack_folder {
            self.packs
                .get(pack_folder)
                .ok_or_else(err)?
                .get(path)
                .cloned()
                .ok_or_else(err)
        } else {
            self.core.get(path).cloned().ok_or_else(err)
        }
    }

    fn watch(&self) -> Option<async_channel::Receiver<AssetChange>> {
        None
    }
}
