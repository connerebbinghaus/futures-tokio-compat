use crate::LocalFrom;
use log::warn;

impl LocalFrom<tokio::executor::SpawnError> for futures_core::task::SpawnError {
    fn from(err: tokio::executor::SpawnError) -> Self {
        if err.is_shutdown() {
            Self::shutdown()
        } else {
            warn!("Converting {:?} into generic shutdown error", err);
            Self::shutdown()
        }
    }
}

impl LocalFrom<futures_core::task::SpawnError> for tokio::executor::SpawnError {
    fn from(err: futures_core::task::SpawnError) -> Self {
        if err.is_shutdown() {
            Self::shutdown()
        } else {
            warn!("Converting {:?} into generic shutdown error", err);
            Self::shutdown()
        }
    }
}
