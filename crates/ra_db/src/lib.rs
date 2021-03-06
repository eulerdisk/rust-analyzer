//! ra_db defines basic database traits. The concrete DB is defined by ra_ide_api.
mod cancellation;
mod input;
mod loc2id;
pub mod mock;

use std::{
    panic, sync::Arc,
};

use ra_syntax::{TextUnit, TextRange, SourceFile, TreeArc};
use relative_path::RelativePathBuf;

pub use ::salsa as salsa;
pub use crate::{
    cancellation::Canceled,
    input::{
        FileId, CrateId, SourceRoot, SourceRootId, CrateGraph, Dependency,
    },
    loc2id::LocationIntener,
};

pub trait BaseDatabase: salsa::Database + panic::RefUnwindSafe {
    /// Aborts current query if there are pending changes.
    ///
    /// rust-analyzer needs to be able to answer semantic questions about the
    /// code while the code is being modified. A common problem is that a
    /// long-running query is being calculated when a new change arrives.
    ///
    /// We can't just apply the change immediately: this will cause the pending
    /// query to see inconsistent state (it will observe an absence of
    /// repeatable read). So what we do is we **cancel** all pending queries
    /// before applying the change.
    ///
    /// We implement cancellation by panicking with a special value and catching
    /// it on the API boundary. Salsa explicitly supports this use-case.
    fn check_canceled(&self) {
        if self.salsa_runtime().is_current_revision_canceled() {
            Canceled::throw()
        }
    }

    fn catch_canceled<F: FnOnce(&Self) -> T + panic::UnwindSafe, T>(
        &self,
        f: F,
    ) -> Result<T, Canceled> {
        panic::catch_unwind(|| f(self)).map_err(|err| match err.downcast::<Canceled>() {
            Ok(canceled) => *canceled,
            Err(payload) => panic::resume_unwind(payload),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub file_id: FileId,
    pub offset: TextUnit,
}

#[derive(Clone, Copy, Debug)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

#[salsa::query_group(FilesDatabaseStorage)]
pub trait FilesDatabase: salsa::Database {
    /// Text of the file.
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;
    /// Path to a file, relative to the root of its source root.
    #[salsa::input]
    fn file_relative_path(&self, file_id: FileId) -> RelativePathBuf;
    /// Source root of the file.
    #[salsa::input]
    fn file_source_root(&self, file_id: FileId) -> SourceRootId;
    /// Contents of the source root.
    #[salsa::input]
    fn source_root(&self, id: SourceRootId) -> Arc<SourceRoot>;
    fn source_root_crates(&self, id: SourceRootId) -> Arc<Vec<CrateId>>;
    /// The set of "local" (that is, from the current workspace) roots.
    /// Files in local roots are assumed to change frequently.
    #[salsa::input]
    fn local_roots(&self) -> Arc<Vec<SourceRootId>>;
    /// The set of roots for crates.io libraries.
    /// Files in libraries are assumed to never change.
    #[salsa::input]
    fn library_roots(&self) -> Arc<Vec<SourceRootId>>;
    /// The crate graph.
    #[salsa::input]
    fn crate_graph(&self) -> Arc<CrateGraph>;
}

fn source_root_crates(db: &impl FilesDatabase, id: SourceRootId) -> Arc<Vec<CrateId>> {
    let root = db.source_root(id);
    let graph = db.crate_graph();
    let res = root
        .files
        .values()
        .filter_map(|&it| graph.crate_id_for_crate_root(it))
        .collect::<Vec<_>>();
    Arc::new(res)
}

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase: FilesDatabase + BaseDatabase {
    fn source_file(&self, file_id: FileId) -> TreeArc<SourceFile>;
}

fn source_file(db: &impl SyntaxDatabase, file_id: FileId) -> TreeArc<SourceFile> {
    let text = db.file_text(file_id);
    SourceFile::parse(&*text)
}
