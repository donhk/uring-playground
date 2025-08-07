use crate::URingIO;
use async_trait::async_trait;
use bytes::Bytes;
use std::fs::Metadata;
use std::os::fd::RawFd;

#[derive(Debug)]
pub struct TokioUringMem {}

impl Default for TokioUringMem {
    fn default() -> Self {
        TokioUringMem {}
    }
}

#[async_trait]
impl URingIO for TokioUringMem {
    async fn read_at(&self, buf: Bytes, offset: u64) -> std::io::Result<(usize, Bytes)> {
        todo!()
    }

    async fn write_at(&self, buf: Bytes, offset: u64) -> std::io::Result<(usize, Bytes)> {
        todo!()
    }

    async fn sync_all(&self) -> std::io::Result<()> {
        todo!()
    }

    async fn sync_data(&self) -> std::io::Result<()> {
        todo!()
    }

    async fn truncate(&self, size: u64) -> std::io::Result<()> {
        todo!()
    }

    async fn metadata(&self) -> std::io::Result<Metadata> {
        todo!()
    }

    fn as_raw_fd(&self) -> Option<RawFd> {
        todo!()
    }
}
