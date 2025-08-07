use crate::URingIO;
use async_trait::async_trait;
use bytes::Bytes;
use std::fs::Metadata;
use std::os::fd::RawFd;
use std::sync::Arc;
use tokio_uring::fs::File;

#[derive(Debug)]
pub struct TokioUringDevice {}

impl TokioUringDevice {
    pub async fn new(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let x = tokio_uring::start(async {
            // Open a file
            let file = File::open("hello.txt").await?;

            let buf = vec![0; 4096];
            // Read some data, the buffer is passed by ownership and
            // submitted to the kernel. When the operation completes,
            // we get the buffer back.
            let (res, buf) = file.read_at(buf, 0).await;
            let n = res?;

            // Display the contents
            println!("{:?}", &buf[..n]);

            Ok(())
        });
        // Open a file
        let file = Arc::new(File::open(filepath).await?);
        Ok(TokioUringDevice { file })
    }
}

#[async_trait]
impl URingIO for TokioUringDevice {
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
