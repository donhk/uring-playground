use crate::{
    BytesNumber, ChunksNumber, DeleteChunkReq, RDataChunkResp, RDeleteChunkResp, RReadChunkResp,
    ReadChunkReq, RepositoryResult, RepositoryState, RequestId, WriteChunkReq,
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

    async fn update_state(&self) -> RepositoryResult<RepositoryState>;

    async fn bytes(&self) -> RepositoryResult<BytesNumber>;

    async fn deleted_bytes(&self) -> RepositoryResult<BytesNumber>;

    async fn chunks_count(&self) -> RepositoryResult<ChunksNumber>;
}
