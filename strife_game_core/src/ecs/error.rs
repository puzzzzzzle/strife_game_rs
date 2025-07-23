use thiserror::Error;
#[derive(Error, Debug)]
pub enum EcsError {
    #[error("unknown error")]
    Unknown,
    #[error("creat registry filed: `{0}`")]
    CreateRegistryFailed(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },

}
