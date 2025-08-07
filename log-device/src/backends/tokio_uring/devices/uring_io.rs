use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;
use std::fs::Metadata;
use std::io::Result;
use std::os::fd::RawFd;

#[async_trait]
pub trait URingIO: Send + Sync + Debug {
    /// Reads `len` bytes into the given buffer starting at offset.
    async fn read_at(&self, buf: Bytes, offset: u64) -> Result<(usize, Bytes)>;

    /// Writes data at the given offset.
    async fn write_at(&self, buf: Bytes, offset: u64) -> Result<(usize, Bytes)>;

    /// Syncs all metadata and data.
    async fn sync_all(&self) -> Result<()>;

    /// Syncs only file data.
    async fn sync_data(&self) -> Result<()>;

    /// Truncates the file to a given size.
    async fn truncate(&self, size: u64) -> Result<()>;

    /// Returns file metadata.
    async fn metadata(&self) -> Result<Metadata>;

    /// Returns the raw file descriptor (only for real impls).
    fn as_raw_fd(&self) -> Option<RawFd>;
}
