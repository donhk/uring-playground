use crate::{ChunkId, RepositoryId};
use bytes::Bytes;

#[derive(Debug, Default, Clone)]
pub enum RepositoryState {
    #[default]
    New,
    Ready,
    Sealing,
    Sealed,
    Compacting,
    Corrupted,
}

#[derive(Debug, Default, Clone)]
pub struct WriteChunkReq {
    buffer: Bytes,
}

#[derive(Debug, Default, Clone)]
pub struct WriteChunkResp {
    chunk_id: ChunkId,
    repository_id: RepositoryId,
}

#[derive(Debug, Default, Clone)]
pub struct ReadChunkReq {
    chunk_id: ChunkId,
}

#[derive(Debug, Default, Clone)]
pub struct ReadChunkResp {
    buffer: Bytes,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteChunkReq {
    chunk_id: ChunkId,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteChunkResp {}
