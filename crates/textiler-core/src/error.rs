//! Contains the main error type

use std::string::FromUtf8Error;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(target_arch = "wasm32")]
    #[error("Web error: {0:?}")]
    Web(Option<JsValue>),
    #[error("Mounting not supported on this architecture")]
    MountingUnsupported,
    #[error("an error occurred while trying to convert sx to css")]
    SxToCssError,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}
