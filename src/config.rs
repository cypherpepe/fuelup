use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use std::io;

use crate::ops::fuelup_toolchain::toolchain;
use crate::path::toolchain_dir;

pub struct Config {
    toolchains_dir: PathBuf,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self> {
        Ok(Self {
            toolchains_dir: toolchain_dir(),
        })
    }

    pub(crate) fn list_official_toolchains(&self) -> Result<Vec<String>> {
        if self.toolchains_dir.is_dir() {
            let toolchains: Vec<String> = fs::read_dir(&self.toolchains_dir)?
                .filter_map(io::Result::ok)
                .filter(|e| {
                    e.file_type().map(|f| f.is_dir()).unwrap_or(false)
                        // TODO: match nightly/stable when channels are available
                        && e.file_name().to_string_lossy().starts_with(toolchain::LATEST)
                })
                .map(|e| e.file_name().into_string().ok().unwrap_or_default())
                .collect();
            Ok(toolchains)
        } else {
            Ok(Vec::new())
        }
    }
}
