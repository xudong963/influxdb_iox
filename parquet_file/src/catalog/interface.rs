//! Abstract interfaces to make different users work with the perserved catalog.
use std::{collections::HashMap, sync::Arc};

use iox_object_store::{IoxObjectStore, ParquetFilePath};
use snafu::Snafu;

use crate::metadata::IoxParquetMetaData;

/// Struct containing all information that a catalog received for a new parquet file.
#[derive(Debug, Clone)]
pub struct CatalogParquetInfo {
    /// Path within this database.
    pub path: ParquetFilePath,

    /// Size of the parquet file, in bytes
    pub file_size_bytes: usize,

    /// Associated parquet metadata.
    pub metadata: Arc<IoxParquetMetaData>,
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CatalogStateAddError {
    #[snafu(display("Cannot extract metadata from {:?}: {}", path, source))]
    MetadataExtractFailed {
        source: crate::metadata::Error,
        path: ParquetFilePath,
    },

    #[snafu(display("Schema for {:?} does not work with existing schema: {}", path, source))]
    SchemaError {
        source: Box<dyn std::error::Error + Send + Sync>,
        path: ParquetFilePath,
    },

    #[snafu(
        display(
            "Internal error: Using checkpoints from {:?} leads to broken replay plan: {}, catalog likely broken",
            path,
            source
        ),
    )]
    ReplayPlanError {
        source: Box<dyn std::error::Error + Send + Sync>,
        path: ParquetFilePath,
    },

    #[snafu(display("Cannot create parquet chunk from {:?}: {}", path, source))]
    ChunkCreationFailed {
        source: crate::chunk::Error,
        path: ParquetFilePath,
    },

    #[snafu(display("Parquet already exists in catalog: {:?}", path))]
    ParquetFileAlreadyExists { path: ParquetFilePath },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CatalogStateRemoveError {
    #[snafu(display("Parquet does not exist in catalog: {:?}", path))]
    ParquetFileDoesNotExist { path: ParquetFilePath },
}

/// Abstraction over how the in-memory state of the catalog works.
pub trait CatalogState {
    /// Input to create a new empty instance.
    ///
    /// See [`new_empty`](Self::new_empty) for details.
    type EmptyInput: Send;

    /// Create empty state w/o any known files.
    fn new_empty(db_name: &str, data: Self::EmptyInput) -> Self;

    /// Add parquet file to state.
    fn add(
        &mut self,
        iox_object_store: Arc<IoxObjectStore>,
        info: CatalogParquetInfo,
    ) -> Result<(), CatalogStateAddError>;

    /// Remove parquet file from state.
    fn remove(&mut self, path: &ParquetFilePath) -> Result<(), CatalogStateRemoveError>;
}

/// Structure that holds all information required to create a checkpoint.
///
/// Note that while checkpoint are addressed using the same schema as we use for transaction
/// (revision counter, UUID), they contain the changes at the end (aka including) the transaction
/// they refer.
#[derive(Debug)]
pub struct CheckpointData {
    /// List of all Parquet files that are currently (i.e. by the current version) tracked by the
    /// catalog.
    ///
    /// If a file was once added but later removed it MUST NOT appear in the result.
    pub files: HashMap<ParquetFilePath, CatalogParquetInfo>,
}