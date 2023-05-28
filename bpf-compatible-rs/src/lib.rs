//!  SPDX-License-Identifier: MIT
//!
//! Copyright (c) 2023, eunomia-bpf
//! All rights reserved.
//!
use std::path::{Path, PathBuf};

pub use crate::error::Error;
use tar::Archive;
pub use tempfile;
pub use tar;
use tempfile::{tempdir, TempDir};
pub type Result<T> = std::result::Result<T, Error>;

/// Errors of this library
pub mod error;

/// Generate the btf archive path of the running kernel
/// It returns somethings like `ubuntu/20.04/x86_64/xxxxxxx.btf`
pub fn generate_current_system_btf_archive_path() -> Result<String> {
    let release_info = os_release::OsRelease::new().map_err(Error::OsReleaseError)?;
    let uname = uname_rs::Uname::new().map_err(Error::UnameError)?;
    let btf_path = format!(
        "{}/{}/{}/{}.btf",
        release_info.id, release_info.version_id, uname.machine, uname.release
    );
    Ok(btf_path)
}

/// Try to get the btf file of the running system under the archive directory
pub fn get_current_system_btf_file(archive_path: impl AsRef<Path>) -> Result<PathBuf> {
    Ok(archive_path
        .as_ref()
        .join(generate_current_system_btf_archive_path()?))
}
/// A helper type definition for simplicity
pub type BtfArchive = Option<(PathBuf, TempDir)>;

/// Unpack a tar archive, returning the contents of `package.json`;
///
/// It will also try get the btfhub-archive path in the unpacked directory.
///
/// It will return the btf archive path and the temporary path to hold it, if applies
///
/// Note: once the tempdir was destructed, the btf archive will be deleted
pub fn unpack_tar(tar_data: &[u8]) -> Result<(Vec<u8>, BtfArchive)> {
    let mut archive = Archive::new(tar_data);
    // tempdir
    let tmp_dir = tempdir().map_err(Error::TempDirError)?;
    archive
        .unpack(tmp_dir.path())
        .map_err(Error::TarUnpackError)?;

    let json_object_buffer = std::fs::read(tmp_dir.path().join("package.json"))
        .map_err(|e| Error::FileReadError("package.json".to_string(), e))?;
    let btf_archive_path = tmp_dir.path().join("btfhub-archive");
    let btf_archive_path = if btf_archive_path.exists() && btf_archive_path.is_dir() {
        Some((btf_archive_path, tmp_dir))
    } else {
        None
    };
    Ok((json_object_buffer, btf_archive_path))
}
