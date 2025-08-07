use crate::{
    BytesNumber, ChunkMetadata, ChunksNumber, DeleteChunkReq, LogId, RDataChunkResp,
    RDeleteChunkResp, RReadChunkResp, ReadChunkReq, RepositoryResult, RepositoryState, RequestId,
    WriteChunkReq,
};
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn write_chunks(
        &self,
        chunks: Vec<WriteChunkReq>,
    ) -> RepositoryResult<Vec<(RequestId, RDataChunkResp)>>;

    async fn read_chunks(
        &self,
        chunks: Vec<ReadChunkReq>,
    ) -> RepositoryResult<Vec<(RequestId, RReadChunkResp)>>;

    async fn delete_chunks(
        &self,
        chunks: Vec<DeleteChunkReq>,
    ) -> RepositoryResult<Vec<(RequestId, RDeleteChunkResp)>>;

    async fn state(&self) -> RepositoryResult<RepositoryState>;

    async fn update_state(&self, state: RepositoryState) -> RepositoryResult<RepositoryState>;

    async fn bytes(&self) -> RepositoryResult<BytesNumber>;

    async fn deleted_bytes(&self) -> RepositoryResult<BytesNumber>;

    async fn chunks_count(&self) -> RepositoryResult<ChunksNumber>;

    /// Lists chunk operations (live and/or deleted), starting after a given log ID.
    /// Can be used for compaction, scanning, or incremental state recovery.
    async fn list_chunks_metadata(
        &self,
        start_after: Option<LogId>,
        include_deleted: bool,
        limit: Option<usize>,
    ) -> RepositoryResult<Vec<ChunkMetadata>>;
}
