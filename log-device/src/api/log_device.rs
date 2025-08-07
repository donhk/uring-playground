use crate::Offset;
use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait LogDevice: Send + Sync {
    /// Appends data and returns the starting offset where it was written.
    async fn append(&self, data: &[u8]) -> std::io::Result<Offset>;

    /// Reads len bytes starting from the given offset.
    async fn read_at(&self, offset: u64, len: usize) -> std::io::Result<Bytes>;

    /// Flushes all in-flight data to disk.
    async fn sync(&self) -> std::io::Result<()>;

    /// Truncates the beginning of the log up to (but not including) offset.
    /// Used after compaction to reclaim space.
    async fn truncate(&self, offset: u64) -> std::io::Result<()>;
}
