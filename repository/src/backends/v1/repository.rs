use crate::backends::v1::config::RepoConfig;
use crate::{
    BytesNumber, ChunkMetadata, ChunksNumber, DeleteChunkReq, LogId, RDataChunkResp,
    RDeleteChunkResp, RReadChunkResp, ReadChunkReq, Repository, RepositoryResult, RepositoryState,
    RequestId, WriteChunkReq,
};
use async_trait::async_trait;

pub struct RepositoryV1 {
    config: RepoConfig,
}

impl RepositoryV1 {
    pub async fn create(config: RepoConfig) -> RepositoryResult<RepositoryV1> {
        // starts the file manager (owner)
        Ok(Self { config })
    }
}

#[async_trait]
impl Repository for RepositoryV1 {
    async fn write_chunks(
        &self,
        _chunks: Vec<WriteChunkReq>,
    ) -> RepositoryResult<Vec<(RequestId, RDataChunkResp)>> {
        todo!()
    }

    async fn read_chunks(
        &self,
        _chunks: Vec<ReadChunkReq>,
    ) -> RepositoryResult<Vec<(RequestId, RReadChunkResp)>> {
        todo!()
    }

    async fn delete_chunks(
        &self,
        _chunks: Vec<DeleteChunkReq>,
    ) -> RepositoryResult<Vec<(RequestId, RDeleteChunkResp)>> {
        todo!()
    }

    async fn state(&self) -> RepositoryResult<RepositoryState> {
        todo!()
    }

    async fn update_state(&self, _state: RepositoryState) -> RepositoryResult<RepositoryState> {
        todo!()
    }

    async fn bytes(&self) -> RepositoryResult<BytesNumber> {
        todo!()
    }

    async fn deleted_bytes(&self) -> RepositoryResult<BytesNumber> {
        todo!()
    }

    async fn chunks_count(&self) -> RepositoryResult<ChunksNumber> {
        todo!()
    }

    async fn list_chunks_metadata(
        &self,
        _start_after: Option<LogId>,
        _include_deleted: bool,
        _limit: Option<usize>,
    ) -> RepositoryResult<Vec<ChunkMetadata>> {
        todo!()
    }
}
