use thiserror::Error;

#[derive(Error, Debug)]
/// Error of this library
pub enum Error {
    #[error("Failed to read os-release: {0}")]
    OsReleaseError(std::io::Error),
    #[error("Failed to call uname: {0}")]
    UnameError(std::io::Error),
    #[error("Failed to create temporary directrory: {0}")]
    TempDirError(std::io::Error),
    #[error("Failed to unpack tar archive: {0}")]
    TarUnpackError(std::io::Error),
    #[error("Failed to read `{0}`: {1}")]
    FileReadError(String, std::io::Error),
}
