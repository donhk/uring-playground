mod api;
mod backends;

pub use api::catalog::{
    ChunkMetadata, DeleteChunkReq, DeleteChunkResp, ReadChunkReq, ReadChunkResp, RepositoryState,
    WriteChunkReq, WriteChunkResp,
};

pub use api::repository::Repository;

pub use api::repository_errors::RepositoryErr;

pub use api::types::{
    BytesNumber, ChunkId, ChunksNumber, LogId, Offset, RDataChunkResp, RDeleteChunkResp,
    RReadChunkResp, RepositoryId, RepositoryResult, RequestId,
};

pub use backends::v1::repository::RepositoryV1;
