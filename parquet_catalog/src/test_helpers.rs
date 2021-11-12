use crate::{
    core::{PreservedCatalog, PreservedCatalogConfig},
    interface::{
        CatalogParquetInfo, CatalogState, CatalogStateAddError, CatalogStateRemoveError,
        CheckpointData, ChunkAddrWithoutDatabase,
    },
    internals::{
        proto_io::{load_transaction_proto, store_transaction_proto},
        types::TransactionKey,
    },
};
use data_types::{chunk_metadata::ChunkId, timestamp::TimestampRange};
use dml::{DeleteExpr, DeletePredicate, Op, Scalar};
use iox_object_store::{IoxObjectStore, ParquetFilePath, TransactionFilePath};
use parquet_file::{
    metadata::IoxParquetMetaData,
    test_utils::{chunk_addr, make_iox_object_store, make_metadata, TestSize},
};
use snafu::ResultExt;
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap, HashSet,
    },
    fmt::Debug,
    sync::Arc,
};

/// Metrics need a database name, but what the database name is doesn't matter for what's tested
/// in this crate. This is an arbitrary name that can be used wherever a database name is needed.
pub const DB_NAME: &str = "db1";

#[derive(Clone, Debug, Default)]
pub struct Table {
    pub partitions: HashMap<Arc<str>, Partition>,
}

#[derive(Clone, Debug, Default)]
pub struct Partition {
    pub chunks: HashMap<ChunkId, Chunk>,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub parquet_info: CatalogParquetInfo,
    pub delete_predicates: Vec<Arc<DeletePredicate>>,
}

/// In-memory catalog state, for testing.
#[derive(Clone, Debug, Default)]
pub struct TestCatalogState {
    /// Map of all parquet files that are currently registered.
    pub tables: HashMap<Arc<str>, Table>,
}

impl TestCatalogState {
    /// Simple way to create [`CheckpointData`].
    pub fn checkpoint_data(&self) -> CheckpointData {
        CheckpointData {
            files: self
                .files()
                .map(|info| (info.path.clone(), info.clone()))
                .collect(),
            delete_predicates: self.delete_predicates(),
        }
    }

    /// Returns an iterator over the files in this catalog state
    pub fn files(&self) -> impl Iterator<Item = &CatalogParquetInfo> {
        self.tables.values().flat_map(|table| {
            table
                .partitions
                .values()
                .flat_map(|partition| partition.chunks.values().map(|chunk| &chunk.parquet_info))
        })
    }

    /// Return an iterator over all predicates in this catalog.
    pub fn delete_predicates(
        &self,
    ) -> HashMap<Arc<DeletePredicate>, HashSet<ChunkAddrWithoutDatabase>> {
        let mut predicates: HashMap<Arc<DeletePredicate>, HashSet<ChunkAddrWithoutDatabase>> =
            Default::default();

        for (table_name, table) in &self.tables {
            for (partition_key, partition) in &table.partitions {
                for (chunk_id, chunk) in &partition.chunks {
                    for predicate in &chunk.delete_predicates {
                        let pred_chunk_closure = || ChunkAddrWithoutDatabase {
                            table_name: Arc::clone(table_name),
                            partition_key: Arc::clone(partition_key),
                            chunk_id: *chunk_id,
                        };
                        predicates
                            .entry(Arc::clone(predicate))
                            .and_modify(|chunks| {
                                chunks.insert(pred_chunk_closure());
                            })
                            .or_insert_with(|| {
                                IntoIterator::into_iter([pred_chunk_closure()]).collect()
                            });
                    }
                }
            }
        }

        predicates
    }

    /// Inserts a file into this catalog state
    pub fn insert(&mut self, info: CatalogParquetInfo) -> Result<(), CatalogStateAddError> {
        use crate::interface::MetadataExtractFailed;

        let iox_md = info
            .metadata
            .decode()
            .context(MetadataExtractFailed {
                path: info.path.clone(),
            })?
            .read_iox_metadata()
            .context(MetadataExtractFailed {
                path: info.path.clone(),
            })?;

        let table = self.tables.entry(iox_md.table_name).or_default();
        let partition = table.partitions.entry(iox_md.partition_key).or_default();

        match partition.chunks.entry(iox_md.chunk_id) {
            Occupied(o) => {
                return Err(CatalogStateAddError::ParquetFileAlreadyExists {
                    path: o.get().parquet_info.path.clone(),
                });
            }
            Vacant(v) => v.insert(Chunk {
                parquet_info: info,
                delete_predicates: vec![],
            }),
        };

        Ok(())
    }
}

impl CatalogState for TestCatalogState {
    fn add(
        &mut self,
        _iox_object_store: Arc<IoxObjectStore>,
        info: CatalogParquetInfo,
    ) -> Result<(), CatalogStateAddError> {
        self.insert(info)
    }

    fn remove(&mut self, path: &ParquetFilePath) -> Result<(), CatalogStateRemoveError> {
        let partitions = self
            .tables
            .values_mut()
            .flat_map(|table| table.partitions.values_mut());
        let mut removed = 0;

        for partition in partitions {
            let to_remove: Vec<_> = partition
                .chunks
                .iter()
                .filter_map(|(id, chunk)| {
                    if &chunk.parquet_info.path == path {
                        return Some(*id);
                    }
                    None
                })
                .collect();

            for id in to_remove {
                removed += 1;
                partition.chunks.remove(&id).unwrap();
            }
        }

        match removed {
            0 => Err(CatalogStateRemoveError::ParquetFileDoesNotExist { path: path.clone() }),
            _ => Ok(()),
        }
    }

    fn delete_predicate(
        &mut self,
        predicate: Arc<DeletePredicate>,
        chunks: Vec<ChunkAddrWithoutDatabase>,
    ) {
        for addr in chunks {
            if let Some(chunk) = self
                .tables
                .get_mut(&addr.table_name)
                .map(|table| table.partitions.get_mut(&addr.partition_key))
                .flatten()
                .map(|partition| partition.chunks.get_mut(&addr.chunk_id))
                .flatten()
            {
                chunk.delete_predicates.push(Arc::clone(&predicate));
            }
        }
    }
}

/// Test whether the catalog exists or not, expecting the operation to succeed
pub async fn exists(iox_object_store: &Arc<IoxObjectStore>) -> bool {
    PreservedCatalog::exists(iox_object_store).await.unwrap()
}

/// Load a `PreservedCatalog` and unwrap, expecting the operation to succeed
pub async fn load_ok(
    config: PreservedCatalogConfig,
) -> Option<(PreservedCatalog, TestCatalogState)> {
    PreservedCatalog::load(config, TestCatalogState::default())
        .await
        .unwrap()
}

/// Load a `PreservedCatalog` and unwrap the error, expecting the operation to fail
pub async fn load_err(config: PreservedCatalogConfig) -> crate::core::Error {
    PreservedCatalog::load(config, TestCatalogState::default())
        .await
        .unwrap_err()
}

/// Create a new empty catalog with the TestCatalogState, expecting the operation to succeed
pub async fn new_empty(config: PreservedCatalogConfig) -> PreservedCatalog {
    PreservedCatalog::new_empty(config).await.unwrap()
}

/// Break preserved catalog by moving one of the transaction files into a weird unknown version.
pub async fn break_catalog_with_weird_version(catalog: &PreservedCatalog) {
    let tkey = get_tkey(catalog);
    let path = TransactionFilePath::new_transaction(tkey.revision_counter, tkey.uuid);
    let mut proto = load_transaction_proto(&catalog.iox_object_store(), &path)
        .await
        .unwrap();
    proto.version = 42;
    store_transaction_proto(&catalog.iox_object_store(), &path, &proto)
        .await
        .unwrap();
}

/// Helper function to ensure that guards don't leak into the future state machine.
fn get_tkey(catalog: &PreservedCatalog) -> TransactionKey {
    let revision_counter = catalog.revision_counter();
    let uuid = catalog.revision_uuid();
    TransactionKey {
        revision_counter,
        uuid,
    }
}

/// Torture-test implementations for [`CatalogState`].
///
/// A function to extract [`CheckpointData`] from the [`CatalogState`] must be provided.
pub async fn assert_catalog_state_implementation<S, F>(mut state: S, f: F)
where
    S: CatalogState + Debug + Send + Sync,
    F: Fn(&S) -> CheckpointData + Send,
{
    let config = make_config().await;

    // The expected state of the catalog
    let mut expected_files: HashMap<ChunkId, (ParquetFilePath, Arc<IoxParquetMetaData>)> =
        HashMap::new();
    let mut expected_predicates: HashMap<Arc<DeletePredicate>, HashSet<ChunkAddrWithoutDatabase>> =
        HashMap::new();
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // add files
    {
        for chunk_id in 0..5 {
            let (path, metadata) = make_metadata(
                &config.iox_object_store,
                "ok",
                chunk_addr(chunk_id),
                TestSize::Full,
            )
            .await;
            state
                .add(
                    Arc::clone(&config.iox_object_store),
                    CatalogParquetInfo {
                        path: path.clone(),
                        file_size_bytes: 33,
                        metadata: Arc::new(metadata.clone()),
                    },
                )
                .unwrap();
            expected_files.insert(ChunkId::new_test(chunk_id), (path, Arc::new(metadata)));
        }
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // remove files
    {
        let (path, _) = expected_files.remove(&ChunkId::new_test(1)).unwrap();
        state.remove(&path).unwrap();
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // add and remove in the same transaction
    {
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "ok",
            chunk_addr(5),
            TestSize::Full,
        )
        .await;
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata),
                },
            )
            .unwrap();
        state.remove(&path).unwrap();
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // remove and add in the same transaction
    {
        let (path, metadata) = expected_files.get(&ChunkId::new_test(3)).unwrap();
        state.remove(path).unwrap();
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::clone(metadata),
                },
            )
            .unwrap();
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // add, remove, add in the same transaction
    {
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "ok",
            chunk_addr(6),
            TestSize::Full,
        )
        .await;
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata.clone()),
                },
            )
            .unwrap();
        state.remove(&path).unwrap();
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata.clone()),
                },
            )
            .unwrap();
        expected_files.insert(ChunkId::new_test(6), (path, Arc::new(metadata)));
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // remove, add, remove in same transaction
    {
        let (path, metadata) = expected_files.remove(&ChunkId::new_test(4)).unwrap();
        state.remove(&path).unwrap();
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::clone(&metadata),
                },
            )
            .unwrap();
        state.remove(&path).unwrap();
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // error handling, no real opt
    {
        // TODO: Error handling should disambiguate between chunk collision and filename collision

        // chunk with same ID already exists (should also not change the metadata)
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "fail",
            chunk_addr(0),
            TestSize::Full,
        )
        .await;
        let err = state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path,
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata),
                },
            )
            .unwrap_err();
        assert!(matches!(
            err,
            CatalogStateAddError::ParquetFileAlreadyExists { .. }
        ));
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // error handling, still something works
    {
        // already exists (should also not change the metadata)
        let (_, metadata) = expected_files.get(&ChunkId::new_test(0)).unwrap();
        let err = state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    // Intentionally "incorrect" path
                    path: ParquetFilePath::new(&chunk_addr(10)),
                    file_size_bytes: 33,
                    metadata: Arc::clone(metadata),
                },
            )
            .unwrap_err();
        assert!(matches!(
            err,
            CatalogStateAddError::ParquetFileAlreadyExists { .. }
        ));

        // this transaction will still work
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "ok",
            chunk_addr(7),
            TestSize::Full,
        )
        .await;
        let metadata = Arc::new(metadata);
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::clone(&metadata),
                },
            )
            .unwrap();
        expected_files.insert(ChunkId::new_test(7), (path.clone(), Arc::clone(&metadata)));

        // recently added
        let err = state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path,
                    file_size_bytes: 33,
                    metadata: Arc::clone(&metadata),
                },
            )
            .unwrap_err();
        assert!(matches!(
            err,
            CatalogStateAddError::ParquetFileAlreadyExists { .. }
        ));

        // this still works
        let (path, _) = expected_files.remove(&ChunkId::new_test(7)).unwrap();
        state.remove(&path).unwrap();

        // recently removed
        let err = state.remove(&path).unwrap_err();
        assert!(matches!(
            err,
            CatalogStateRemoveError::ParquetFileDoesNotExist { .. }
        ));
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // add predicates
    {
        // create two chunks that we can use for delete predicate
        let chunk_addr_1 = chunk_addr(8);
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "ok",
            chunk_addr_1.clone(),
            TestSize::Full,
        )
        .await;
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata.clone()),
                },
            )
            .unwrap();
        expected_files.insert(chunk_addr_1.chunk_id, (path, Arc::new(metadata)));

        let chunk_addr_2 = chunk_addr(9);
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "ok",
            chunk_addr_2.clone(),
            TestSize::Full,
        )
        .await;
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata.clone()),
                },
            )
            .unwrap();
        expected_files.insert(chunk_addr_2.chunk_id, (path, Arc::new(metadata)));

        // first predicate used only a single chunk
        let predicate_1 = create_delete_predicate(1);
        let chunks_1 = vec![chunk_addr_1.clone().into()];
        state.delete_predicate(Arc::clone(&predicate_1), chunks_1.clone());
        expected_predicates.insert(predicate_1, chunks_1.into_iter().collect());

        // second predicate uses both chunks (but not the older chunks)
        let predicate_2 = create_delete_predicate(2);
        let chunks_2 = vec![chunk_addr_1.into(), chunk_addr_2.into()];
        state.delete_predicate(Arc::clone(&predicate_2), chunks_2.clone());
        expected_predicates.insert(predicate_2, chunks_2.into_iter().collect());

        // chunks created afterwards are unaffected
        let chunk_addr_3 = chunk_addr(10);
        let (path, metadata) = make_metadata(
            &config.iox_object_store,
            "ok",
            chunk_addr_3.clone(),
            TestSize::Full,
        )
        .await;
        state
            .add(
                Arc::clone(&config.iox_object_store),
                CatalogParquetInfo {
                    path: path.clone(),
                    file_size_bytes: 33,
                    metadata: Arc::new(metadata.clone()),
                },
            )
            .unwrap();
        expected_files.insert(chunk_addr_3.chunk_id, (path, Arc::new(metadata)));
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // removing a chunk will also remove its predicates
    {
        let (path, _) = expected_files.remove(&ChunkId::new_test(8)).unwrap();
        state.remove(&path).unwrap();
        expected_predicates = expected_predicates
            .into_iter()
            .filter_map(|(predicate, chunks)| {
                let chunks: HashSet<_> = chunks
                    .into_iter()
                    .filter(|addr| addr.chunk_id != ChunkId::new_test(8))
                    .collect();
                (!chunks.is_empty()).then(|| (predicate, chunks))
            })
            .collect();
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);

    // Registering predicates for unknown chunks is just ignored because chunks might been in "persisting" intermediate
    // state while the predicate was reported.
    {
        let predicate = create_delete_predicate(1);
        let chunks = vec![ChunkAddrWithoutDatabase {
            table_name: Arc::from("some_table"),
            partition_key: Arc::from("part"),
            chunk_id: ChunkId::new_test(1000),
        }];
        state.delete_predicate(Arc::clone(&predicate), chunks);
    }
    assert_checkpoint(&state, &f, &expected_files, &expected_predicates);
}

/// Assert that tracked files and their linked metadata are equal.
fn assert_checkpoint<S, F>(
    state: &S,
    f: &F,
    expected_files: &HashMap<ChunkId, (ParquetFilePath, Arc<IoxParquetMetaData>)>,
    expected_predicates: &HashMap<Arc<DeletePredicate>, HashSet<ChunkAddrWithoutDatabase>>,
) where
    F: Fn(&S) -> CheckpointData,
{
    let data = f(state);
    let actual_files = data.files;

    let sorted_keys_actual = get_sorted_keys(actual_files.keys());
    let sorted_keys_expected = get_sorted_keys(expected_files.values().map(|(path, _)| path));
    assert_eq!(sorted_keys_actual, sorted_keys_expected);

    for (path, md_expected) in expected_files.values() {
        let md_actual = &actual_files[path].metadata;

        let md_actual = md_actual.decode().unwrap();
        let md_expected = md_expected.decode().unwrap();

        let iox_md_actual = md_actual.read_iox_metadata().unwrap();
        let iox_md_expected = md_expected.read_iox_metadata().unwrap();
        assert_eq!(iox_md_actual, iox_md_expected);

        let schema_actual = md_actual.read_schema().unwrap();
        let schema_expected = md_expected.read_schema().unwrap();
        assert_eq!(schema_actual, schema_expected);

        let stats_actual = md_actual.read_statistics(&schema_actual).unwrap();
        let stats_expected = md_expected.read_statistics(&schema_expected).unwrap();
        assert_eq!(stats_actual, stats_expected);
    }

    assert_eq!(&data.delete_predicates, expected_predicates);
}

/// Get a sorted list of keys from an iterator.
fn get_sorted_keys<'a>(
    keys: impl Iterator<Item = &'a ParquetFilePath>,
) -> Vec<&'a ParquetFilePath> {
    let mut keys: Vec<_> = keys.collect();
    keys.sort();
    keys
}

/// Helper to create a simple delete predicate.
pub fn create_delete_predicate(value: i64) -> Arc<DeletePredicate> {
    Arc::new(DeletePredicate {
        range: TimestampRange { start: 11, end: 22 },
        exprs: vec![DeleteExpr::new(
            "foo".to_string(),
            Op::Eq,
            Scalar::I64(value),
        )],
    })
}

/// Creates a new [`PreservedCatalogConfig`] with an in-memory object store
pub async fn make_config() -> PreservedCatalogConfig {
    let iox_object_store = make_iox_object_store().await;
    let time_provider = Arc::new(time::SystemProvider::new());
    PreservedCatalogConfig::new(iox_object_store, DB_NAME.to_string(), time_provider)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_catalog_state() {
        assert_catalog_state_implementation(
            TestCatalogState::default(),
            TestCatalogState::checkpoint_data,
        )
        .await;
    }
}
