use crate::{LogDevice, Offset, TokioUringMem, URingIO};
use async_trait::async_trait;
use bytes::Bytes;
use derive_builder::Builder;
use std::sync::Arc;

#[derive(Builder, Debug)]
pub struct TokioLogDeviceConfig {
    #[builder(default = "Arc::new(TokioUringMem::default())")]
    io_device: Arc<dyn URingIO>,
}

pub struct TokioLogDevice {
    io_device: Arc<dyn URingIO>,
}

#[async_trait]
impl LogDevice for TokioLogDevice {
    async fn append(&self, data: &[u8]) -> std::io::Result<Offset> {
        todo!()
    }

    async fn read_at(&self, offset: u64, len: usize) -> std::io::Result<Bytes> {
        todo!()
    }

    async fn sync(&self) -> std::io::Result<()> {
        todo!()
    }

    async fn truncate(&self, offset: u64) -> std::io::Result<()> {
        todo!()
    }
}
