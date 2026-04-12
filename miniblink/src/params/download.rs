use crate::types::{WebFrameHandle, DownloadJob};

/// The download parameters.
pub struct DownloadParameters {
    /// The frame handler.
    pub frame_id: WebFrameHandle,
    /// The url.
    pub url: String,
    /// The download job.
    pub download_job: DownloadJob,
}
