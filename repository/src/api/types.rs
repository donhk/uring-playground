use crate::{DeleteChunkResp, ReadChunkResp, RepositoryErr, WriteChunkResp};
use uuid::Uuid;

pub type RepositoryResult<T> = Result<T, RepositoryErr>;

pub type RDataChunkResp = Result<WriteChunkResp, RepositoryErr>;

pub type RDeleteChunkResp = Result<DeleteChunkResp, RepositoryErr>;

pub type RReadChunkResp = Result<ReadChunkResp, RepositoryErr>;

pub type RequestId = Uuid;

pub type ChunkId = Uuid;

pub type RepositoryId = Uuid;

pub type BytesNumber = u64;

pub type ChunksNumber = u64;
