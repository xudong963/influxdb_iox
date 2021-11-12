//! Methods to cleanup the object store.
use std::{collections::HashSet, sync::Arc};

use futures::TryStreamExt;
use iox_object_store::{IoxObjectStore, ParquetFilePath};
use object_store::{ObjectStore, ObjectStoreApi};
use observability_deps::tracing::info;
use parking_lot::Mutex;
use dml::DeletePredicate;
use snafu::{ResultExt, Snafu};

use crate::{
    core::PreservedCatalog,
    interface::{
        CatalogParquetInfo, CatalogState, CatalogStateAddError, CatalogStateRemoveError,
        ChunkAddrWithoutDatabase,
    },
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error from read operation while cleaning object store: {}", source))]
    ReadError {
        source: <ObjectStore as ObjectStoreApi>::Error,
    },

    #[snafu(display("Error from write operation while cleaning object store: {}", source))]
    WriteError {
        source: <ObjectStore as ObjectStoreApi>::Error,
    },

    #[snafu(display("Error from catalog loading while cleaning object store: {}", source))]
    CatalogLoadError { source: crate::core::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Get unreferenced parquet files.
///
/// The resulting vector is in no particular order. It may be passed to [`delete_files`].
///
/// # Locking / Concurrent Actions
///
/// While this method is running you MUST NOT create any new parquet files or modify the preserved
/// catalog in any other way. Hence this method needs exclusive access to the preserved catalog and
/// the parquet file. Otherwise this method may report files for deletion that you are about to
/// write to the catalog!
///
/// **This method does NOT acquire the transaction lock!**
///
/// To limit the time the exclusive access is required use `max_files` which will limit the number
/// of files to be detected in this cleanup round.
///
/// The exclusive access can be dropped after this method returned and before calling
/// [`delete_files`].
pub async fn get_unreferenced_parquet_files(
    catalog: &PreservedCatalog,
    max_files: usize,
) -> Result<Vec<ParquetFilePath>> {
    let iox_object_store = catalog.iox_object_store();
    let all_known = {
        // replay catalog transactions to track ALL (even dropped) files that are referenced
        let (_catalog, state) = PreservedCatalog::load::<TracerCatalogState>(
            catalog.config(),
            TracerCatalogState::default(),
        )
        .await
        .context(CatalogLoadError)?
        .expect("catalog gone while reading it?");

        state.files.into_inner()
    };

    // gather a list of "files to remove" eagerly so we do not block transactions on the catalog
    // for too long
    let mut to_remove = vec![];
    let mut stream = iox_object_store.parquet_files().await.context(ReadError)?;

    'outer: while let Some(paths) = stream.try_next().await.context(ReadError)? {
        for path in paths {
            if to_remove.len() >= max_files {
                info!(%max_files, "reached limit of number of files to cleanup in one go");
                break 'outer;
            }

            // only delete if file is not tracked by the catalog
            if !all_known.contains(&path) {
                to_remove.push(path);
            }
        }
    }

    if !to_remove.is_empty() {
        info!(n_files = to_remove.len(), "Found files to delete");
    }

    Ok(to_remove)
}

/// Delete all `files` from the store linked to the preserved catalog.
///
/// A file might already be deleted (or entirely absent) when this method is called. This will NOT
/// result in an error.
///
/// # Locking / Concurrent Actions
/// File creation and catalog modifications can be done while calling this method. Even
/// [`get_unreferenced_parquet_files`] can be called while is method is in-progress.
pub async fn delete_files(catalog: &PreservedCatalog, files: &[ParquetFilePath]) -> Result<()> {
    let store = catalog.iox_object_store();

    for path in files {
        info!(?path, "Delete file");
        store.delete_parquet_file(path).await.context(WriteError)?;
    }

    if !files.is_empty() {
        info!(n_files = files.len(), "Finished deletion, removed files.");
    }

    Ok(())
}

/// Catalog state that traces all used parquet files.
#[derive(Debug, Default)]
struct TracerCatalogState {
    files: Mutex<HashSet<ParquetFilePath>>,
}

impl CatalogState for TracerCatalogState {
    fn add(
        &mut self,
        _iox_object_store: Arc<IoxObjectStore>,
        info: CatalogParquetInfo,
    ) -> Result<(), CatalogStateAddError> {
        self.files.lock().insert(info.path);
        Ok(())
    }

    fn remove(&mut self, _path: &ParquetFilePath) -> Result<(), CatalogStateRemoveError> {
        // Do NOT remove the file since we still need it for time travel
        Ok(())
    }

    fn delete_predicate(
        &mut self,
        _predicate: Arc<DeletePredicate>,
        _chunks: Vec<ChunkAddrWithoutDatabase>,
    ) {
        // No need to track delete predicates, because the cleanup's job is to remove unreferenced parquet files. Delete
        // predicates however are stored directly within the preserved catalog and therefore don't need pruning.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{make_config, new_empty};
    use parquet_file::test_utils::{chunk_addr, make_metadata, TestSize};
    use std::{collections::HashSet, sync::Arc};
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_cleanup_empty() {
        let config = make_config().await;

        let catalog = new_empty(config).await;

        // run clean-up
        let files = get_unreferenced_parquet_files(&catalog, 1_000)
            .await
            .unwrap();
        delete_files(&catalog, &files).await.unwrap();
    }

    #[tokio::test]
    async fn test_cleanup_rules() {
        let config = make_config().await;
        let iox_object_store = &config.iox_object_store;

        let catalog = new_empty(config.clone()).await;

        // create some data
        let mut paths_keep = vec![];
        let mut paths_delete = vec![];
        {
            let mut transaction = catalog.open_transaction().await;

            // an ordinary tracked parquet file => keep
            let (path, metadata) =
                make_metadata(iox_object_store, "foo", chunk_addr(1), TestSize::Full).await;
            let metadata = Arc::new(metadata);
            let info = CatalogParquetInfo {
                path,
                file_size_bytes: 33,
                metadata,
            };

            transaction.add_parquet(&info);
            paths_keep.push(info.path);

            // another ordinary tracked parquet file that was added and removed => keep (for time
            // travel)
            let (path, metadata) =
                make_metadata(iox_object_store, "foo", chunk_addr(2), TestSize::Full).await;
            let metadata = Arc::new(metadata);
            let info = CatalogParquetInfo {
                path,
                file_size_bytes: 33,
                metadata,
            };
            transaction.add_parquet(&info);
            transaction.remove_parquet(&info.path);
            paths_keep.push(info.path);

            // an untracked parquet file => delete
            let (path, _md) =
                make_metadata(iox_object_store, "foo", chunk_addr(3), TestSize::Full).await;
            paths_delete.push(path);

            transaction.commit().await.unwrap();
        }

        // run clean-up
        let files = get_unreferenced_parquet_files(&catalog, 1_000)
            .await
            .unwrap();
        delete_files(&catalog, &files).await.unwrap();

        // deleting a second time should just work
        delete_files(&catalog, &files).await.unwrap();

        // list all files
        let all_files = list_all_files(iox_object_store).await;
        for p in paths_keep {
            assert!(dbg!(&all_files).contains(dbg!(&p)));
        }
        for p in paths_delete {
            assert!(!dbg!(&all_files).contains(dbg!(&p)));
        }
    }

    #[tokio::test]
    async fn test_cleanup_with_parallel_transaction() {
        let config = make_config().await;
        let iox_object_store = &config.iox_object_store;
        let lock: RwLock<()> = Default::default();

        let catalog = new_empty(config.clone()).await;

        // try multiple times to provoke a conflict
        for i in 0..100 {
            // Every so often try to create a file with the same ChunkAddr beforehand. This should
            // not trick the cleanup logic to remove the actual file because file paths contains a
            // UUIDv4 part.
            if i % 2 == 0 {
                make_metadata(iox_object_store, "foo", chunk_addr(i), TestSize::Full).await;
            }

            let (path, _) = tokio::join!(
                async {
                    let guard = lock.read().await;
                    let (path, md) =
                        make_metadata(iox_object_store, "foo", chunk_addr(i), TestSize::Full).await;

                    let metadata = Arc::new(md);
                    let info = CatalogParquetInfo {
                        path,
                        file_size_bytes: 33,
                        metadata,
                    };

                    let mut transaction = catalog.open_transaction().await;
                    transaction.add_parquet(&info);
                    transaction.commit().await.unwrap();

                    drop(guard);

                    info.path
                },
                async {
                    let guard = lock.write().await;
                    let files = get_unreferenced_parquet_files(&catalog, 1_000)
                        .await
                        .unwrap();
                    drop(guard);

                    delete_files(&catalog, &files).await.unwrap();
                },
            );

            let all_files = list_all_files(iox_object_store).await;
            assert!(dbg!(all_files).contains(dbg!(&path)));
        }
    }

    #[tokio::test]
    async fn test_cleanup_max_files() {
        let config = make_config().await;
        let iox_object_store = &config.iox_object_store;

        let catalog = new_empty(config.clone()).await;

        // create some files
        let mut to_remove = HashSet::default();
        for chunk_id in 0..3 {
            let (path, _md) = make_metadata(
                iox_object_store,
                "foo",
                chunk_addr(chunk_id),
                TestSize::Full,
            )
            .await;
            to_remove.insert(path);
        }

        // run clean-up
        let files = get_unreferenced_parquet_files(&catalog, 2).await.unwrap();
        assert_eq!(files.len(), 2);
        delete_files(&catalog, &files).await.unwrap();

        // should only delete 2
        let all_files = list_all_files(iox_object_store).await;
        let leftover: HashSet<_> = all_files.intersection(&to_remove).collect();
        assert_eq!(leftover.len(), 1);

        // run clean-up again
        let files = get_unreferenced_parquet_files(&catalog, 2).await.unwrap();
        assert_eq!(files.len(), 1);
        delete_files(&catalog, &files).await.unwrap();

        // should delete remaining file
        let all_files = list_all_files(iox_object_store).await;
        let leftover: HashSet<_> = all_files.intersection(&to_remove).collect();
        assert_eq!(leftover.len(), 0);
    }

    async fn list_all_files(iox_object_store: &IoxObjectStore) -> HashSet<ParquetFilePath> {
        iox_object_store
            .parquet_files()
            .await
            .unwrap()
            .try_concat()
            .await
            .unwrap()
            .into_iter()
            .collect()
    }
}
