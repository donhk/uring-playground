use crate::{ChunkId, LogId, Offset, RepositoryId};
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

#[derive(Debug, Default, Clone, Copy)]
pub struct ChunkMetadata {
    pub chunk_id: ChunkId,
    pub log_id: LogId,
    pub offset: Offset,
    pub len: u32,
    pub is_deleted: bool,
    pub timestamp: u64,
}
